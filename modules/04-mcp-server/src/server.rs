use pmcp::server::Server;

pub struct ProductionServer {
    #[allow(dead_code)]
    server: Server,
}

impl ProductionServer {
    #[must_use]
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
