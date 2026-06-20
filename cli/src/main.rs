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
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use chrono::{Utc, DateTime};
use trust_dns_resolver::{TokioAsyncResolver, config::{ResolverConfig, ResolverOpts}};

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
    /// View local stats
    Stats {
        /// Export stats to file
        #[arg(short, long)]
        export: Option<String>,
        /// Reset stats
        #[arg(short, long)]
        reset: bool,
        /// Sync stats to registry
        #[arg(short, long)]
        sync: bool,
    },
    /// Check or upgrade tier
    Tier {
        /// Upgrade to next tier
        #[arg(short, long)]
        upgrade: bool,
    },
    /// Pool management
    Pool {
        /// Join a pool
        #[arg(short, long)]
        join: Option<String>,
        /// Leave current pool
        #[arg(short, long)]
        leave: bool,
        /// Check pool status
        #[arg(short, long)]
        status: bool,
    },
    /// Domain management via shop.mcp.city
    Domain {
        /// Search for available mcp:// domains
        #[arg(short, long)]
        search: Option<String>,
        /// Purchase a domain
        #[arg(short, long)]
        purchase: Option<String>,
        /// Register mcp:// domain
        #[arg(short, long)]
        register: Option<String>,
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
    db: SqlitePool,
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
            discover_mcp_servers(&domain).await?;
        }
        
        Commands::Register { name, endpoint, description } => {
            register_server(&name, &endpoint, description.as_deref()).await?;
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
        
        Commands::Stats { export, reset, sync } => {
            view_stats(export, reset, sync).await?;
        }
        
        Commands::Tier { upgrade } => {
            manage_tier(upgrade).await?;
        }
        
        Commands::Pool { join, leave, status } => {
            manage_pool(join, leave, status).await?;
        }
        
        Commands::Domain { search, purchase, register } => {
            manage_domain(search, purchase, register).await?;
        }
    }
    
    Ok(())
}

async fn start_server(port: u16, name: String) -> Result<()> {
    let db_path = "mcp_stats.db";
    let db_options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);
    
    let db = SqlitePool::connect_with(db_options).await?;
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS stats (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            total_requests INTEGER NOT NULL DEFAULT 0,
            successful_requests INTEGER NOT NULL DEFAULT 0,
            failed_requests INTEGER NOT NULL DEFAULT 0,
            tool_calls TEXT NOT NULL DEFAULT '{}'
        )
        "#
    )
    .execute(&db)
    .await?;
    
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
        db,
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
    println!("💾 Stats database: {}", db_path);
    
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
        
        let tool_calls_json = serde_json::to_string(&state.stats.tool_calls).unwrap_or_else(|_| "{}".to_string());
        
        sqlx::query(
            r#"
            INSERT INTO stats (timestamp, total_requests, successful_requests, failed_requests, tool_calls)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(Utc::now().to_rfc3339())
        .bind(state.stats.total_requests as i64)
        .bind(state.stats.successful_requests as i64)
        .bind(state.stats.failed_requests as i64)
        .bind(&tool_calls_json)
        .execute(&state.db)
        .await
        .ok();
        
        Json(ToolCallResponse {
            result,
            success: true,
            error: None,
        })
    } else {
        state.stats.failed_requests += 1;
        
        let tool_calls_json = serde_json::to_string(&state.stats.tool_calls).unwrap_or_else(|_| "{}".to_string());
        
        sqlx::query(
            r#"
            INSERT INTO stats (timestamp, total_requests, successful_requests, failed_requests, tool_calls)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(Utc::now().to_rfc3339())
        .bind(state.stats.total_requests as i64)
        .bind(state.stats.successful_requests as i64)
        .bind(state.stats.failed_requests as i64)
        .bind(&tool_calls_json)
        .execute(&state.db)
        .await
        .ok();
        
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

async fn view_stats(export: Option<String>, reset: bool, sync: bool) -> Result<()> {
    let db_path = "mcp_stats.db";
    
    if !std::path::Path::new(db_path).exists() {
        println!("❌ Stats database not found. Start the server first with: mcp serve");
        return Ok(());
    }
    
    let db_options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(false);
    
    let db = SqlitePool::connect_with(db_options).await?;
    
    if reset {
        sqlx::query("DELETE FROM stats")
            .execute(&db)
            .await?;
        println!("✅ Stats reset successfully");
        return Ok(());
    }
    
    if sync {
        println!("🔄 Syncing stats to registry.mcp.city...");
        
        let rows = sqlx::query_as::<_, (i64, String, i64, i64, i64, String)>(
            "SELECT id, timestamp, total_requests, successful_requests, failed_requests, tool_calls FROM stats ORDER BY timestamp DESC LIMIT 100"
        )
        .fetch_all(&db)
        .await?;
        
        let sync_url = "https://registry.mcp.city/api/v1/stats/sync";
        
        let payload = serde_json::json!({
            "stats": rows
        });
        
        let response = client.post(sync_url).json(&payload).send().await;
        
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("✅ Stats synced successfully");
                } else {
                    println!("❌ Sync failed: {}", resp.status());
                    if let Ok(error) = resp.text().await {
                        println!("Error: {}", error);
                    }
                }
            }
            Err(e) => {
                println!("❌ Sync failed: {}", e);
                println!("💡 Note: Stats sync requires Registered tier or higher");
                println!("💡 Contact HYBRID IN. to upgrade: https://hybridin.io/");
            }
        }
        
        return Ok(());
    }
    
    let rows = sqlx::query_as::<_, (i64, String, i64, i64, i64, String)>(
        "SELECT id, timestamp, total_requests, successful_requests, failed_requests, tool_calls FROM stats ORDER BY timestamp DESC LIMIT 100"
    )
    .fetch_all(&db)
    .await?;
    
    println!("📊 MCP Server Stats");
    println!("====================");
    println!();
    
    for (id, timestamp, total, successful, failed, tool_calls) in rows {
        println!("ID: {}", id);
        println!("Timestamp: {}", timestamp);
        println!("Total Requests: {}", total);
        println!("Successful: {}", successful);
        println!("Failed: {}", failed);
        println!("Tool Calls: {}", tool_calls);
        println!();
    }
    
    if let Some(export_path) = export {
        let export_data = serde_json::to_string_pretty(&rows)?;
        std::fs::write(&export_path, export_data)?;
        println!("✅ Stats exported to: {}", export_path);
    }
    
    Ok(())
}

async fn discover_mcp_servers(domain: &str) -> Result<()> {
    println!("🔍 Discovering MCP servers for {}...", domain);
    println!("📡 DNS TXT record: _mcp._tcp.{}", domain);
    
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()).await?;
    let txt_record_name = format!("_mcp._tcp.{}", domain);
    
    match resolver.txt_lookup(txt_record_name.clone()).await {
        Ok(txt_records) => {
            println!("✅ Found {} DNS TXT record(s)", txt_records.len());
            for txt in txt_records {
                for record in txt.txt_data() {
                    println!("📝 Record: {}", record);
                    
                    if let Some(endpoint) = parse_txt_record(record) {
                        println!("🌐 MCP Endpoint: {}", endpoint);
                        
                        let response = client.get(&endpoint).send().await;
                        if let Ok(resp) = response {
                            if resp.status().is_success() {
                                if let Ok(info) = resp.json::<ServerInfo>().await {
                                    println!("📦 Server: {}", info.name);
                                    println!("🔧 Version: {}", info.version);
                                    println!("⚡ Protocol: {}", info.protocol);
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ DNS lookup failed: {}", e);
            println!("💡 Trying /.well-known/mcp endpoint...");
            
            let well_known_url = format!("https://{}/.well-known/mcp", domain);
            let response = client.get(&well_known_url).send().await;
            
            if let Ok(resp) = response {
                if resp.status().is_success() {
                    println!("✅ Found /.well-known/mcp endpoint");
                    if let Ok(info) = resp.json::<ServerInfo>().await {
                        println!("📦 Server: {}", info.name);
                        println!("🔧 Version: {}", info.version);
                        println!("⚡ Protocol: {}", info.protocol);
                    }
                } else {
                    println!("❌ /.well-known/mcp not found");
                }
            }
        }
    }
    
    Ok(())
}

fn parse_txt_record(record: &str) -> Option<String> {
    for part in record.split_whitespace() {
        if part.starts_with("endpoint=") {
            return Some(part.trim_start_matches("endpoint=").to_string());
        }
    }
    None
}

async fn register_server(name: &str, endpoint: &str, description: Option<&str>) -> Result<()> {
    println!("📝 Registering server: {}", name);
    println!("🌐 Endpoint: {}", endpoint);
    if let Some(desc) = description {
        println!("📄 Description: {}", desc);
    }
    
    let registry_url = "https://registry.mcp.city/api/v1/servers";
    
    let mut payload = serde_json::json!({
        "name": name,
        "endpoint": endpoint,
    });
    
    if let Some(desc) = description {
        payload["description"] = serde_json::Value::String(desc.to_string());
    }
    
    let response = client
        .post(registry_url)
        .json(&payload)
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("✅ Server registered successfully");
                if let Ok(result) = resp.json::<serde_json::Value>().await {
                    println!("📦 Server ID: {}", result.get("id").unwrap_or(&serde_json::Value::Null));
                }
            } else {
                println!("❌ Registration failed: {}", resp.status());
                if let Ok(error) = resp.text().await {
                    println!("Error: {}", error);
                }
            }
        }
        Err(e) => {
            println!("❌ Registration failed: {}", e);
            println!("💡 Note: MCP.city registry API is not yet available");
            println!("💡 This is a placeholder for future implementation");
        }
    }
    
    Ok(())
}

async fn manage_tier(upgrade: bool) -> Result<()> {
    println!("🏆 MCP.city Tier System");
    println!("========================");
    println!();
    
    let tiers = vec![
        ("Free", "100/min", "Community support", "Self-hosted"),
        ("Registered", "500/min", "Email support", "Registry listing"),
        ("Pool Member", "1000/min", "Priority support", "Geographic routing"),
        ("Paid Beta", "10000/min", "24/7 support", "COGNIT MESH access"),
    ];
    
    println!("Available Tiers:");
    for (i, (name, rate, support, features)) in tiers.iter().enumerate() {
        println!("{}. {} - Rate: {}, Support: {}, Features: {}", i + 1, name, rate, support, features);
    }
    println!();
    
    if upgrade {
        println!("⬆️  Tier Upgrade");
        println!("================");
        println!("💡 To upgrade your tier, contact HYBRID IN.:");
        println!("   Website: https://hybridin.io/");
        println!("   Email: contact@hybridin.io");
        println!();
        println!("💡 Note: Tier upgrades require HYBRID IN. hosting");
        println!("   - Pool Member: Join geographic compute pool");
        println!("   - Paid Beta: Access COGNIT MESH and WHYBLE NODEs");
    } else {
        println!("📊 Current Tier: Free (Open Source)");
        println!("💡 Upgrade to access advanced features:");
        println!("   - Higher rate limits");
        println!("   - Priority support");
        println!("   - Geographic routing");
        println!("   - COGNIT MESH integration");
        println!("   - WHYBLE NODEs access");
    }
    
    Ok(())
}

async fn manage_pool(join: Option<String>, leave: bool, status: bool) -> Result<()> {
    println!("🌊 MCP.city Pool Management");
    println!("===========================");
    println!();
    
    let pools = vec![
        ("EU", "Europe", "Frankfurt, London, Paris"),
        ("US", "United States", "Virginia, Oregon, California"),
        ("ASIA", "Asia Pacific", "Singapore, Tokyo, Sydney"),
    ];
    
    if let Some(pool_name) = join {
        println!("🔗 Joining pool: {}", pool_name);
        println!();
        
        let pool_url = format!("https://registry.mcp.city/api/v1/pools/{}", pool_name);
        
        let response = client.post(&pool_url).send().await;
        
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("✅ Successfully joined pool: {}", pool_name);
                    if let Ok(result) = resp.json::<serde_json::Value>().await {
                        println!("📦 Pool ID: {}", result.get("id").unwrap_or(&serde_json::Value::Null));
                    }
                } else {
                    println!("❌ Failed to join pool: {}", resp.status());
                    if let Ok(error) = resp.text().await {
                        println!("Error: {}", error);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to join pool: {}", e);
                println!("💡 Note: Pool joining requires Pool Member tier or higher");
                println!("💡 Contact HYBRID IN. to upgrade: https://hybridin.io/");
            }
        }
    } else if leave {
        println!("🚪 Leaving current pool");
        println!();
        
        let pool_url = "https://registry.mcp.city/api/v1/pools/leave";
        
        let response = client.post(pool_url).send().await;
        
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("✅ Successfully left pool");
                } else {
                    println!("❌ Failed to leave pool: {}", resp.status());
                }
            }
            Err(e) => {
                println!("❌ Failed to leave pool: {}", e);
            }
        }
    } else if status {
        println!("📊 Available Pools:");
        for (i, (code, region, locations)) in pools.iter().enumerate() {
            println!("{}. {} ({}) - {}", i + 1, code, region, locations);
        }
        println!();
        println!("💡 To join a pool: mcp pool --join <POOL_CODE>");
        println!("💡 Note: Pool joining requires Pool Member tier or higher");
    } else {
        println!("📊 Available Pools:");
        for (i, (code, region, locations)) in pools.iter().enumerate() {
            println!("{}. {} ({}) - {}", i + 1, code, region, locations);
        }
        println!();
        println!("💡 Current Status: Not in any pool");
        println!("💡 To join a pool: mcp pool --join <POOL_CODE>");
        println!("💡 To check status: mcp pool --status");
        println!("💡 To leave pool: mcp pool --leave");
    }
    
    Ok(())
}

async fn manage_domain(search: Option<String>, purchase: Option<String>, register: Option<String>) -> Result<()> {
    println!("🌐 Domain Management via shop.mcp.city");
    println!("=======================================");
    println!();
    
    if let Some(query) = search {
        println!("🔍 Searching for mcp:// domains: {}", query);
        println!();
        
        let search_url = format!("https://shop.mcp.city/api/v1/domains/search?q={}", query);
        
        let response = client.get(&search_url).send().await;
        
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    if let Ok(results) = resp.json::<serde_json::Value>().await {
                        println!("✅ Search results:");
                        if let Some(domains) = results.get("domains").and_then(|d| d.as_array()) {
                            for domain in domains {
                                println!("  - mcp://{} - €{}", 
                                    domain.get("domain").unwrap_or(&serde_json::Value::Null),
                                    domain.get("price").unwrap_or(&serde_json::Value::Null)
                                );
                            }
                        } else {
                            println!("  No domains found");
                        }
                    }
                } else {
                    println!("❌ Search failed: {}", resp.status());
                }
            }
            Err(e) => {
                println!("❌ Search failed: {}", e);
                println!("💡 Note: shop.mcp.city API is not yet available");
                println!("💡 This is a placeholder for future implementation");
            }
        }
    } else if let Some(domain) = purchase {
        println!("💳 Purchasing domain: {}", domain);
        println!();
        
        let purchase_url = "https://shop.mcp.city/api/v1/domains/purchase";
        
        let payload = serde_json::json!({
            "domain": domain
        });
        
        let response = client.post(purchase_url).json(&payload).send().await;
        
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("✅ Domain purchase initiated");
                    if let Ok(result) = resp.json::<serde_json::Value>().await {
                        println!("📦 Transaction ID: {}", result.get("transaction_id").unwrap_or(&serde_json::Value::Null));
                        println!("💰 Price: €{}", result.get("price").unwrap_or(&serde_json::Value::Null));
                        println!("💡 Complete purchase via escrow at shop.mcp.city");
                    }
                } else {
                    println!("❌ Purchase failed: {}", resp.status());
                }
            }
            Err(e) => {
                println!("❌ Purchase failed: {}", e);
                println!("💡 Note: shop.mcp.city API is not yet available");
                println!("💡 This is a placeholder for future implementation");
            }
        }
    } else if let Some(domain) = register {
        println!("📝 Registering mcp:// domain: {}", domain);
        println!();
        
        let register_url = "https://shop.mcp.city/api/v1/domains/register";
        
        let payload = serde_json::json!({
            "domain": domain
        });
        
        let response = client.post(register_url).json(&payload).send().await;
        
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("✅ Domain registered successfully");
                    if let Ok(result) = resp.json::<serde_json::Value>().await {
                        println!("📦 Domain ID: {}", result.get("id").unwrap_or(&serde_json::Value::Null));
                        println!("🌐 mcp://{} is now active", domain);
                        println!("💡 Your domain is accessible via the MCP network");
                    }
                } else {
                    println!("❌ Registration failed: {}", resp.status());
                }
            }
            Err(e) => {
                println!("❌ Registration failed: {}", e);
                println!("💡 Note: shop.mcp.city API is not yet available");
                println!("💡 This is a placeholder for future implementation");
            }
        }
    } else {
        println!("📊 Domain Management Commands:");
        println!("  mcp domain --search <query>  - Search for available mcp:// domains");
        println!("  mcp domain --purchase <domain> - Purchase a domain via shop.mcp.city");
        println!("  mcp domain --register <domain> - Register mcp:// domain");
        println!();
        println!("💡 Example: mcp domain --search green.cloud");
        println!("💡 Example: mcp domain --purchase mcp://green.cloud");
        println!("💡 Example: mcp domain --register mcp://green.cloud");
        println!();
        println!("🌐 Shop: https://shop.mcp.city");
        println!("🌐 API: https://shop.mcp.city/api/v1");
    }
    
    Ok(())
}
