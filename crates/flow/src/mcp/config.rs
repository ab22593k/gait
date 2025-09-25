//! Configuration for the MCP server

use serde::{Deserialize, Serialize};

/// Configuration options for the MCP server
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MCPServerConfig {
    /// Whether to enable development mode with more verbose logging
    pub dev_mode: bool,
    /// The transport type to use (stdio, sse)
    pub transport: MCPTransportType,
    /// Port to use for network transports
    pub port: Option<u16>,
    /// Listen address for network transports (e.g., "127.0.0.1", "0.0.0.0")
    pub listen_address: Option<String>,
}

/// Types of transports supported by the MCP server
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum MCPTransportType {
    /// Standard input/output transport
    StdIO,
    /// Server-Sent Events transport
    SSE,
}

impl Default for MCPServerConfig {
    fn default() -> Self {
        Self {
            dev_mode: false,
            transport: MCPTransportType::StdIO,
            port: None,
            listen_address: None,
        }
    }
}

impl MCPServerConfig {
    /// Create a new configuration with development mode enabled
    #[must_use]
    pub fn with_dev_mode(mut self) -> Self {
        self.dev_mode = true;
        self
    }

    /// Create a new configuration with the specified transport
    #[must_use]
    pub fn with_transport(mut self, transport: MCPTransportType) -> Self {
        self.transport = transport;
        self
    }

    /// Create a new configuration with the specified port
    #[must_use]
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Create a new configuration with the specified listen address
    #[must_use]
    pub fn with_listen_address(mut self, listen_address: impl Into<String>) -> Self {
        self.listen_address = Some(listen_address.into());
        self
    }
}
