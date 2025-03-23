use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use std::sync::Once;

// Used to ensure the mock server is only started once
static START_MOCK_SERVER: Once = Once::new();

// Integration test that uses the actual binary
#[test]
fn test_mcp_client_with_mock_server() {
    // Start the mock server
    START_MOCK_SERVER.call_once(|| {
        // Start the mock server as a separate process
        let server_process = Command::new("cargo")
            .args(&["run", "--example", "mock_server"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start mock server");
        
        // Give the server time to start
        sleep(Duration::from_secs(2));
        
        // Register a shutdown function to kill the server process when tests complete
        let pid = server_process.id();
        std::mem::forget(server_process); // Don't kill process when variable goes out of scope
        
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            let _ = Command::new("kill")
                .args(&[pid.to_string()])
                .status();
        }
        
        #[cfg(windows)]
        {
            let _ = Command::new("taskkill")
                .args(&["/PID", &pid.to_string(), "/F"])
                .status();
        }
    });
    
    // Wait for server to be ready
    sleep(Duration::from_secs(1));
    
    // Run the main client application
    let output = Command::new("cargo")
        .args(&["run"])
        .env("MCP_WEBSOCKET_URL", "ws://localhost:8080")
        .output()
        .expect("Failed to run client");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    println!("STDOUT: {}", stdout);
    println!("STDERR: {}", stderr);
    
    assert!(output.status.success(), "Client application failed");
    assert!(stdout.contains("Successfully connected"), "Client didn't connect successfully");
}
