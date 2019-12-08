use super::*;
use std::io::Cursor;
use tokio::io::{self, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWriteExt, BufStream};

/// # Open Message
///
/// An Open Message is sent when establishing a connection to a BGP peer.
///
/// ```text
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+
/// |    Version    |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |     My Autonomous System      |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |           Hold Time           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         BGP Identifier                        |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// | Opt Parm Len  |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |             Optional Parameters (variable)                    |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
/// [Source](https://tools.ietf.org/html/rfc4271#section-4.2)

pub struct Open {
    pub version: u8,
    pub my_as: ASN,
    pub hold_time: u16,
    pub bgp_identifier: u32,
    pub opt_parm_len: u8,
    pub optional_parameters: Option<Vec<u8>>,
}

impl Open {
    pub async fn from_bytes<T: AsyncBufReadExt + Sized + Unpin>(
        input: &mut T,
    ) -> Result<Open, Error> {
        let version = input.read_u8().await?;
        let my_asn = input.read_u16().await?;
        let hold_time = input.read_u16().await?;
        let bgp_identifier = input.read_u32().await?;
        let opt_parm_len = input.read_u8().await?;

        if opt_parm_len != 0 {
            panic!("optional parameters are not yet supported");
        }

        Ok(Open {
            version: version,
            my_as: my_asn as ASN,
            hold_time: hold_time,
            bgp_identifier: bgp_identifier,
            opt_parm_len: opt_parm_len,
            optional_parameters: None,
        })
    }

    fn write_packet(&self) {
        let h = Header::new(MessageType::Open, std::mem::size_of::<Self>() as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::*;

    const DOCUMENTATION_ASN: ASN = 64511;

    #[test]
    fn test_from_bytes() {
        let mut input = Cursor::new(b"\x04\xFB\xFF\x01\x00\x00\x00\x00\x16\x00\x00");
        let open = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(Open::from_bytes(&mut input))
            .unwrap();

        assert_eq!(open.version, 4);
        assert_eq!(open.my_as, DOCUMENTATION_ASN);
        assert_eq!(open.hold_time, 256);
        assert_eq!(open.bgp_identifier, 22);
        assert_eq!(open.opt_parm_len, 0);
        // TODO: add optional parms
    }
}
