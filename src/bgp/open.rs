extern crate nom;
use super::*;
use nom::{
    bytes::streaming::{tag, take},
    combinator::map_res,
    number::streaming::be_u8,
    sequence::tuple,
    IResult,
};

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
    pub fn from_bytes(input: &[u8]) -> Result<Open, Error> {
        let (input, version) = be_u8(input)?;

        unimplemented!()
    }

    fn write_packet(&self) {
        let h = Header::new(MessageType::Open, std::mem::size_of::<Self>() as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOCUMENTATION_ASN: ASN = 64511;

    #[test]
    fn test_from_bytes() {
        let open = Open::from_bytes(b"").unwrap();
        // TODO
        assert_eq!(open.version, 4);
        assert_eq!(open.my_as, DOCUMENTATION_ASN);
    }
}
