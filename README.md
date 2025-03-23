# RadiologyCluster System

A Rust application that implements a radiology analysis system using the Model Context Protocol (MCP) SDK.

## Overview

This project demonstrates how to build a system that can process and analyze radiological images through a Model Context Protocol service. It includes:

- Client for connecting to MCP servers
- Mock server for testing without a real backend
- Structure for organizing and processing radiology images
- Connection retry mechanism for resilient operation
- Comprehensive test suite for unit and integration testing

## Installation

1. Clone the repository
2. Install dependencies:

```bash
cargo build
```

## Usage

### Running the application

```bash
# Option 1: Run with default settings (connects to ws://localhost:8080)
cargo run

# Option 2: Specify a different MCP server
MCP_WEBSOCKET_URL=ws://your-server:9090 cargo run
```

### Testing with mock server

Start the mock server in one terminal:

```bash
cargo run --example mock_server
```

Then run the main application in another terminal:

```bash
cargo run
```

#### Mock Server Details

The mock server implements a simplified WebSocket server that:
- Listens on 127.0.0.1:8080 by default
- Handles WebSocket protocol handshakes automatically
- Responds to client messages with simulated analysis results
- Logs connections and message activity for debugging

If you encounter connection issues:
- Ensure no other application is using port 8080
- Check terminal output for detailed error messages
- Verify the client is using the correct WebSocket URL
- For WebSocket handshake errors, check that proper headers are being set

## Testing

The project includes a comprehensive test suite for ensuring functionality:

### Running all tests

```bash
cargo test
```

### Running specific test suites

```bash
# Run only the RadiologyCluster unit tests
cargo test --test radiology_cluster_tests

# Run only the integration tests
cargo test --test integration_test
```

### Test with visible output

To see output from tests (useful for debugging):

```bash
cargo test -- --nocapture
```

### Run tests sequentially

For tests that might interact with the same resources:

```bash
cargo test -- --test-threads=1
```

### Test architecture

- **Unit tests**: Tests individual components in isolation
- **Integration tests**: Tests the system end-to-end with the mock server
- **Connection retry tests**: Verifies resilience when servers are unavailable

## Project Structure

- `src/main.rs` - Main application code
- `src/lib.rs` - Reusable library components
- `examples/mock_server.rs` - WebSocket server for testing
- `tests/radiology_cluster_tests.rs` - Unit tests for RadiologyCluster
- `tests/integration_test.rs` - End-to-end integration tests
- `Cargo.toml` - Project dependencies

## How It Works

The RadiologyCluster system connects to an MCP server using WebSockets and sends radiology image metadata for analysis. The server processes the data and returns findings, which are then processed by the client application.

The system uses:
- Tokio for async runtime
- Serde for serialization/deserialization
- MCP Rust SDK for communication
- tokio-tungstenite for WebSocket functionality

### Connection Handling

The system includes robust connection handling:
- Automatic retry mechanism (configurable attempts and delay)
- Proper WebSocket protocol compliance
- Detailed error reporting for connection issues
- Graceful handling of server disconnections

## Development

### Adding new test cases

1. For unit tests, add functions to `tests/radiology_cluster_tests.rs`
2. For integration tests, mosdify or add to `tests/integration_test.rs`
3. Use the `#[test]` attribute for synchronous tests
4. Use the `#[tokio::test]` attribute for asynchronous tests

### Adding new features

1. Implement core functionality in `src/lib.rs` for reusability
2. Add CLI or application-specific code to `src/main.rs`
3. Update tests to verify new functionality

### Debugging WebSocket Connections

When troubleshooting WebSocket connectivity issues:
1. Run the mock server with visible output: `cargo run --example mock_server`
2. Check server logs for connection attempts and handshake errors
3. Verify the client is using the correct WebSocket URL (ws://localhost:8080)
4. Ensure proper WebSocket protocol handling in both client and server code

## Next Steps

- Implement image data transfer
- Add persistent storage for results
- Create a user interface
- Add authentication and permission controls
- Expand test coverage for edge cases
- Add benchmarking tests for performance analysis
