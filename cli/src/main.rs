use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

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
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerInfo {
    name: String,
    version: String,
    protocol: String,
    capabilities: Vec<String>,
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
    }
    
    Ok(())
}
