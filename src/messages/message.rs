use super::{Error, Header, Open, Update};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufStream};

/// MessageType represents what the type of message is. These are encoded as
/// u8s when sending a message to another BGP Peer.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MessageType {
    Open = 1,
    Update = 2,
    Notification = 3,
    KeepAlive = 4,
}

#[derive(Debug, Clone)]
pub enum Message {
    Open(Open),
    Update(Update),
}

impl Message {
    pub async fn from_bytes<T: AsyncReadExt + Sized + Unpin>(input: &mut T) -> Result<Self, Error> {
        let header = Header::from_bytes(input).await?;

        match header.message {
            Some(m) => Ok(m),
            None => Err(Error::new("no message found")),
        }
    }
}
