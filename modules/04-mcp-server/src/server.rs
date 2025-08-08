use pmcp::server::Server;

pub struct ProductionServer {
    server: Server,
}

impl ProductionServer {
    pub fn new() -> Self {
        Self {
            server: pmcp::server::ServerBuilder::new().build(),
        }
    }
}

impl Default for ProductionServer {
    fn default() -> Self {
        Self::new()
    }
}