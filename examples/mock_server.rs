use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;

// Define a custom error type that implements Send
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await?;
    println!("Mock MCP server listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from: {}", addr);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
    Ok(())
}

async fn handle_connection(stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    println!("WebSocket connection established");
    
    let (mut write, mut read) = ws_stream.split();
    
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    println!("Received message: {}", msg);
                    
                    // Parse the message and create a mock response
                    let response = json!({
                        "status": "success",
                        "message": "Analysis completed successfully",
                        "results": {
                            "findings": "Mock radiology findings: No abnormalities detected",
                            "confidence": 0.92
                        }
                    });
                    
                    // Send back the response
                    write.send(Message::Text(response.to_string())).await?;
                }
            },
            Err(e) => {
                println!("Error receiving message: {}", e);
                break;
            }
        }
    }
    
    println!("WebSocket connection closed");
    Ok(())
}
