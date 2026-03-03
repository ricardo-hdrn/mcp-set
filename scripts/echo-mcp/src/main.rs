use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
    transport::stdio,
    ServerHandler, ServiceExt,
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EchoParams {
    #[schemars(description = "The message to echo back")]
    pub message: String,
}

#[derive(Clone)]
pub struct EchoServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl EchoServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Echo back the provided message")]
    fn echo(&self, Parameters(EchoParams { message }): Parameters<EchoParams>) -> String {
        format!("echo: {message}")
    }

    #[tool(description = "Reverse the provided message")]
    fn reverse(&self, Parameters(EchoParams { message }): Parameters<EchoParams>) -> String {
        format!("reversed: {}", message.chars().rev().collect::<String>())
    }
}

#[tool_handler]
impl ServerHandler for EchoServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "echo-mcp".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                ..Default::default()
            },
            instructions: Some("A simple echo MCP server for testing mcp-set.".into()),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let service = EchoServer::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
