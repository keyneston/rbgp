use super::session::Session;
use crate::messages::Error;
use std::collections::HashMap;

/// ConnectionManager collects and controls the various outgoing connections.
/// The id field represents the id it calls itself when talking to external BGP servers.
pub struct ConnectionManager {
    sessions: HashMap<String, Session>,
    pub id: String,
}

impl ConnectionManager {
    /// run is the runloop for the connection manager.
    pub async fn run(&mut self) {
        unimplemented!()
    }

    /// create_session will launch a session and add it to the connection manager.
    pub fn create_session(&mut self, peer: &str) -> Result<(), Error> {
        unimplemented!()
    }
}
