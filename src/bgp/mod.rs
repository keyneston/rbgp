pub mod error;
pub mod header;
pub mod open;

pub use error::*;
pub use header::*;
pub use open::*;

///////////
// Types //
///////////

/// ASN is an autonomous system number.
type ASN = u16;

/// MessageType represents what the type of message is. These are encoded as
/// u8s when sending a message to another BGP Peer.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum MessageType {
    Open = 1,
    Update = 2,
    Notification = 3,
    KeepAlive = 4,
}
