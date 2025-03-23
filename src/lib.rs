use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use mcp_rust_sdk::client::Client;
use serde_json::Value;

// Publicly export structs for testing
#[derive(Clone, Serialize, Deserialize)]
pub struct RadiologyImage {
    pub image_id: String,
    pub data: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RadiologyResult {
    pub image_id: String,
    pub findings: String,
    pub confidence_score: f32,
    pub analysis_date: String,
}

// Define our own simplified message types to use for the analysis
#[derive(Clone, Serialize, Deserialize)]
pub struct AnalysisMessage {
    pub role: String,
    pub content: String,
}

// The RadiologyCluster for managing radiology processing through MCP
pub struct RadiologyCluster {
    client: Arc<Client>,
    contexts: Mutex<HashMap<String, String>>, // Store context IDs
}

impl RadiologyCluster {
    pub fn new(client: Arc<Client>) -> Self {
        RadiologyCluster {
            client,
            contexts: Mutex::new(HashMap::new()),
        }
    }

    pub async fn initialize_context(&self, context_id: &str, model_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Store the mapping of our logical context ID to the model name
        self.contexts.lock().unwrap().insert(context_id.to_string(), model_name.to_string());
        println!("Initialized mapping for context '{}' to model '{}'", context_id, model_name);
        
        Ok(())
    }

    pub async fn submit_image(&self, context_id: &str, image: RadiologyImage) -> Result<String, Box<dyn std::error::Error>> {
        let contexts = self.contexts.lock().unwrap();
        let model_name = contexts.get(context_id).ok_or("Context not found")?;

        // Create a message to send via the client
        let prompt = format!(
            "You are a radiology analysis system. Analyze the following medical image:\n\n{}",
            serde_json::to_string(&image.metadata)?
        );
        
        // Create the message payload as a JSON string
        let message_data = serde_json::json!({
            "model": model_name,
            "prompt": prompt,
            "image_id": image.image_id
        });
        
        // Convert to string - the client.request expects a &str
        let message_str = message_data.to_string();
        
        // The request method requires an Option<Value> as second parameter
        let options: Option<Value> = None;
        
        // Pass the message string directly to request
        let response = self.client.request(&message_str, options).await?;
        
        // Convert response to string
        let response_str = response.to_string();
        println!("Processed image {}: {}", image.image_id, response_str);
        
        Ok(response_str)
    }

    pub async fn get_results(&self, context_id: &str) -> Result<Vec<RadiologyResult>, Box<dyn std::error::Error>> {
        // In a real implementation, you would retrieve stored results
        println!("Retrieving results for context '{}'", context_id);
        
        // Simulate retrieving results
        Ok(vec![])
    }
}
