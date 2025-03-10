use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await?;
    println!("Mock MCP server listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut ws_stream = accept_async(stream).await?;
    
    println!("New WebSocket connection established");
    
    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
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
            ws_stream.send(Message::Text(response.to_string())).await?;
        }
    }
    
    Ok(())
}
