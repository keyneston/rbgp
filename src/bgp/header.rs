use super::*;

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
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum MessageType {
    Open = 1,
    Update = 2,
    Notification = 3,
    KeepAlive = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// Header represents a header for a BGP message.
pub struct Header {
    // Marker must have all bits set 1.
    marker: [u8; 16],
    length: u16,
    message_type: MessageType,
}

impl Header {
    pub fn new(message_type: MessageType, length: u16) -> Self {
        Header {
            marker: [std::u8::MAX; 16],
            length: length,
            message_type: message_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let h = Header::new(MessageType::Open, 16);

        assert_eq!(h.message_type, MessageType::Open);
        assert_eq!(h.length, 16);

        println!("{:?}", h.marker);
        // Ensure that all header bits are 1s
        for i in &h.marker {
            assert_eq!(*i, 255 as u8);
        }
    }
}
