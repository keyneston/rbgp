use super::*;
use std::io::Cursor;
use tokio::io::AsyncReadExt;

/// # Message Header
///
/// ```text
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                           Marker                              |
/// +                                                               +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |          Length               |      Type     |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
/// [Source](https://tools.ietf.org/html/rfc4271#section-4.1)

// TODO: Add message_types enum which then encodes to a u8

/// MessageType represents what the type of message is. These are encoded as
/// u8s when sending a message to another BGP Peer.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MessageType {
    Open = 1,
    Update = 2,
    Notification = 3,
    KeepAlive = 4,
}

#[repr(C)]
#[derive(Debug, Clone)]
/// Header represents a header for a BGP message.
pub struct Header {
    // Marker must have all bits set 1.
    marker: [u8; 16],
    message_type: MessageType,
    pub message: Option<Message>,
}

impl Header {
    pub fn new(message_type: MessageType, length: u16) -> Self {
        Header {
            marker: [std::u8::MAX; 16],
            message_type: message_type,
            message: None,
        }
    }

    pub async fn from_bytes<T: AsyncReadExt + Sized + Unpin>(input: &mut T) -> Result<Self, Error> {
        let mut marker: [u8; 16] = [0; 16];

        input.read(&mut marker);
        for i in marker.iter() {
            if *i != std::u8::MAX {
                return Err(Error::new("invalid marker in header"));
            }
        }

        let length = input.read_u16().await?;
        let message_type = input.read_u8().await?;

        // Build a buffer to read the entire message for later parsing.
        let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);

        input.read(&mut buffer).await?;

        let message = match message_type {
            1 => {
                let open = Open::from_bytes(&mut Cursor::new(buffer)).await?;
                Message::Open(open)
            }
            2 => {
                let update = Update::from_bytes(&mut Cursor::new(buffer)).await?;
                Message::Update(update)
            }
            unknown => return Err(Error::new(&format!("unknown message type: {}", unknown))),
        };

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let h = Header::new(MessageType::Open, 16);

        assert_eq!(h.message_type, MessageType::Open);

        println!("{:?}", h.marker);
        // Ensure that all header bits are 1s
        for i in &h.marker {
            assert_eq!(*i, 255 as u8);
        }
    }
}
