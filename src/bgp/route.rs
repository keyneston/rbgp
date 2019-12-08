use super::*;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};

/// # Route
/// ```text
/// Withdrawn Routes:
///
/// This is a variable-length field that contains a list of IP
/// address prefixes for the routes that are being withdrawn from
/// service.  Each IP address prefix is encoded as a 2-tuple of the
/// form <length, prefix>, whose fields are described below:
///
///          +---------------------------+
///          |   Length (1 octet)        |
///          +---------------------------+
///          |   Prefix (variable)       |
///          +---------------------------+
///
/// The use and the meaning of these fields are as follows:
///
/// a) Length:
///
///    The Length field indicates the length in bits of the IP
///    address prefix.  A length of zero indicates a prefix that
///    matches all IP addresses (with prefix, itself, of zero
///    octets).
///
/// b) Prefix:
///
///    The Prefix field contains an IP address prefix, followed by
///    the minimum number of trailing bits needed to make the end
///    of the field fall on an octet boundary.  Note that the value
///    of trailing bits is irrelevant.
/// ```
/// [Source](https://tools.ietf.org/html/rfc4271#section-4.3)
pub struct Route {
    // TODO: turn this prefix into a native type
    pub prefix: Vec<u8>,
}

impl Route {
    #[allow(dead_code)]
    pub async fn from_bytes<T: AsyncBufReadExt + Sized + Unpin>(
        input: &mut T,
    ) -> Result<Route, Error> {
        let length = input.read_u8().await?;
        let mut buf: Vec<u8> = Vec::with_capacity(length as usize);

        input.read(&mut buf).await?;

        Ok(Route { prefix: buf })
    }
}
