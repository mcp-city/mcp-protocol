use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "mcp")]
#[command(about = "mcp:// protocol CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect to an MCP server
    Connect {
        /// Server URL (mcp:// or https://)
        url: String,
    },
    /// Discover MCP servers via DNS
    Discover {
        /// Domain to discover
        domain: String,
    },
    /// Register a server with MCP.city
    Register {
        /// Server name
        #[arg(short, long)]
        name: String,
        /// Server endpoint
        #[arg(short, long)]
        endpoint: String,
        /// Server description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// List tools on a server
    ListTools {
        /// Server URL
        url: String,
    },
    /// Call a tool
    Call {
        /// Server URL
        url: String,
        /// Tool name
        tool: String,
        /// Tool arguments (JSON)
        #[arg(short, long)]
        args: Option<String>,
    },
    /// Start local MCP server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
        /// Server name
        #[arg(short, long, default_value = "Local MCP Server")]
        name: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ServerInfo {
    name: String,
    version: String,
    protocol: String,
    capabilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ServerState {
    info: ServerInfo,
    tools: HashMap<String, Tool>,
    stats: ServerStats,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ServerStats {
    uptime_seconds: u64,
    total_requests: u64,
    successful_requests: u64,
    failed_requests: u64,
    tool_calls: HashMap<String, u64>,
}

impl ServerStats {
    fn new() -> Self {
        ServerStats {
            uptime_seconds: 0,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            tool_calls: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ToolsResponse {
    tools: Vec<Tool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tool {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct ToolCallRequest {
    tool: String,
    arguments: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct ToolCallResponse {
    result: serde_json::Value,
    success: bool,
    error: Option<String>,
}

fn convert_mcp_to_https(url: &str) -> String {
    if url.starts_with("mcp://") {
        url.replace("mcp://", "https://")
    } else {
        url.to_string()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::Connect { url } => {
            let https_url = convert_mcp_to_https(&url);
            println!("Connecting to {}...", https_url);
            
            let response = client.get(&https_url).send().await?;
            
            if response.status().is_success() {
                let info: ServerInfo = response.json().await?;
                println!("Connected to: {}", info.name);
                println!("Version: {}", info.version);
                println!("Protocol: {}", info.protocol);
                println!("Capabilities: {:?}", info.capabilities);
            } else {
                println!("Failed to connect: {}", response.status());
            }
        }
        
        Commands::Discover { domain } => {
            println!("Discovering MCP servers for {}...", domain);
            println!("DNS TXT record: _mcp._tcp.{}", domain);
            // TODO: Implement actual DNS lookup
            println!("DNS discovery not yet implemented");
        }
        
        Commands::Register { name, endpoint, description } => {
            println!("Registering server: {}", name);
            println!("Endpoint: {}", endpoint);
            if let Some(desc) = description {
                println!("Description: {}", desc);
            }
            println!("Registration not yet implemented - requires MCP.city API");
        }
        
        Commands::ListTools { url } => {
            let https_url = convert_mcp_to_https(&url);
            let tools_url = format!("{}/tools/list", https_url);
            
            let response = client.get(&tools_url).send().await?;
            
            if response.status().is_success() {
                let tools: ToolsResponse = response.json().await?;
                println!("Available tools:");
                for tool in tools.tools {
                    println!("  - {}: {}", tool.name, tool.description);
                }
            } else {
                println!("Failed to list tools: {}", response.status());
            }
        }
        
        Commands::Call { url, tool, args } => {
            let https_url = convert_mcp_to_https(&url);
            let call_url = format!("{}/tools/call", https_url);
            
            let arguments = if let Some(args_str) = args {
                serde_json::from_str(&args_str)?
            } else {
                serde_json::json!({})
            };
            
            let request = ToolCallRequest {
                tool,
                arguments,
            };
            
            let response = client
                .post(&call_url)
                .json(&request)
                .send()
                .await?;
            
            if response.status().is_success() {
                let result: ToolCallResponse = response.json().await?;
                if result.success {
                    println!("Result: {}", serde_json::to_string_pretty(&result.result)?);
                } else {
                    println!("Error: {}", result.error.unwrap_or_else(|| "Unknown error".to_string()));
                }
            } else {
                println!("Failed to call tool: {}", response.status());
            }
        }
        
        Commands::Serve { port, name } => {
            start_server(port, name).await?;
        }
    }
    
    Ok(())
}

async fn start_server(port: u16, name: String) -> Result<()> {
    let server_info = ServerInfo {
        name: name.clone(),
        version: "0.1.0".to_string(),
        protocol: "mcp://".to_string(),
        capabilities: vec!["tools/list".to_string(), "tools/call".to_string()],
    };
    
    let mut tools = HashMap::new();
    tools.insert("echo".to_string(), Tool {
        name: "echo".to_string(),
        description: "Echo back the input".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "Message to echo"
                }
            },
            "required": ["message"]
        }),
    });
    
    let state = Arc::new(RwLock::new(ServerState {
        info: server_info.clone(),
        tools,
        stats: ServerStats::new(),
    }));
    
    let app = Router::new()
        .route("/", get(get_server_info))
        .route("/tools/list", get(list_tools))
        .route("/tools/call", post(call_tool))
        .route("/stats", get(get_stats))
        .with_state(state.clone());
    
    let addr = format!("0.0.0.0:{}", port);
    println!("🚀 Starting MCP server: {}", name);
    println!("📡 Listening on: http://{}", addr);
    println!("📊 Stats endpoint: http://{}/stats", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn get_server_info(State(state): State<Arc<RwLock<ServerState>>>) -> Json<ServerInfo> {
    let state = state.read().await;
    Json(state.info.clone())
}

async fn list_tools(State(state): State<Arc<RwLock<ServerState>>>) -> Json<ToolsResponse> {
    let state = state.read().await;
    let tools: Vec<Tool> = state.tools.values().cloned().collect();
    Json(ToolsResponse { tools })
}

async fn call_tool(
    State(state): State<Arc<RwLock<ServerState>>>,
    Json(request): Json<ToolCallRequest>,
) -> Json<ToolCallResponse> {
    let mut state = state.write().await;
    
    state.stats.total_requests += 1;
    
    let tool_name = request.tool.clone();
    
    if let Some(_tool) = state.tools.get(&tool_name) {
        *state.stats.tool_calls.entry(tool_name.clone()).or_insert(0) += 1;
        state.stats.successful_requests += 1;
        
        let result = if tool_name == "echo" {
            request.arguments
        } else {
            serde_json::json!({"error": "Tool not implemented"})
        };
        
        Json(ToolCallResponse {
            result,
            success: true,
            error: None,
        })
    } else {
        state.stats.failed_requests += 1;
        Json(ToolCallResponse {
            result: serde_json::json!({}),
            success: false,
            error: Some(format!("Tool '{}' not found", tool_name)),
        })
    }
}

async fn get_stats(State(state): State<Arc<RwLock<ServerState>>>) -> Json<ServerStats> {
    let state = state.read().await;
    Json(state.stats.clone())
}
