extern crate byteorder;
use super::*;

use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWriteExt, BufStream};

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

#[derive(Debug, Clone)]
pub struct Open {
    pub version: u8,
    pub asn: ASN,
    pub hold_time: u16,
    pub bgp_id: u32,
    pub optional_parameters: Option<Vec<u8>>,
}

impl Open {
    pub async fn from_bytes<T: AsyncReadExt + Sized + Unpin>(input: &mut T) -> Result<Open, Error> {
        let version = input.read_u8().await?;
        let asn = input.read_u16().await?;
        let hold_time = input.read_u16().await?;
        let bgp_id = input.read_u32().await?;
        let opt_parm_len = input.read_u8().await?;

        if opt_parm_len != 0 {
            panic!("optional parameters are not yet supported");
        }

        Ok(Open {
            version: version,
            asn: asn as ASN,
            hold_time: hold_time,
            bgp_id: bgp_id,
            optional_parameters: None,
        })
    }

    pub fn size(&self) -> u16 {
        let version_size = 8;
        let asn_size = 16;
        let hold_time_size = 16;
        let bgp_id_size = 32;
        let opt_parm_len_size = 8;

        // TODO: figure out the length of the actual optional params.
        let _opt_params_size = 0;

        return version_size + asn_size + hold_time_size + bgp_id_size + opt_parm_len_size;
    }

    pub async fn write_bytes<T: AsyncWriteExt + Sized + Unpin>(
        &self,
        stream: &mut T,
    ) -> Result<(), Error> {
        // create a buffer with an estimated size
        let mut open_msg: Vec<u8> = Vec::with_capacity(self.size() as usize);

        open_msg.write_u8(self.version).await?;
        open_msg.write_u16(self.asn).await?;
        open_msg.write_u16(self.hold_time).await?;
        open_msg.write_u32(self.bgp_id).await?;
        match self.optional_parameters {
            None => open_msg.write_u8(0).await?,
            Some(_) => unimplemented!(),
        };

        let h = Header::new(MessageType::Open, (open_msg.len() * 8) as u16);

        h.write_bytes(stream).await?;
        stream.write(&open_msg).await?;

        Ok(())
    }
}

impl Default for Open {
    fn default() -> Self {
        Open {
            version: 4,
            asn: 0,
            hold_time: 60,
            bgp_id: 0,
            optional_parameters: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    

    const DOCUMENTATION_ASN: ASN = 64511;

    #[test]
    fn test_from_bytes() {
        let mut input = Cursor::new(b"\x04\xFB\xFF\x01\x00\x00\x00\x00\x16\x00\x00");
        let open = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(Open::from_bytes(&mut input))
            .unwrap();

        assert_eq!(open.version, 4);
        assert_eq!(open.asn, DOCUMENTATION_ASN);
        assert_eq!(open.hold_time, 256);
        assert_eq!(open.bgp_id, 22);
        // TODO: add optional parms
    }

    #[test]
    fn test_size_no_optionals() {
        let o = Open {
            ..Default::default()
        };
        let size = o.size();

        assert_eq!(size, 80);
    }
}
