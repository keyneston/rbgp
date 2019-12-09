use super::*;
use route::Route;
use tokio::io::AsyncReadExt;
/// # Update Message
///
/// ```text
/// +-----------------------------------------------------+
/// |   Withdrawn Routes Length (2 octets)                |
/// +-----------------------------------------------------+
/// |   Withdrawn Routes (variable)                       |
/// +-----------------------------------------------------+
/// |   Total Path Attribute Length (2 octets)            |
/// +-----------------------------------------------------+
/// |   Path Attributes (variable)                        |
/// +-----------------------------------------------------+
/// |   Network Layer Reachability Information (variable) |
/// +-----------------------------------------------------+
/// ```
/// [Source](https://tools.ietf.org/html/rfc4271#section-4.3)

#[derive(Debug, Clone)]
pub struct Update {
    withdrawn_routes: Vec<Route>,
    path_attributes: Vec<PathAttributes>,
}

#[derive(Debug, Clone)]
pub struct PathAttributes {}

impl Update {
    #[allow(dead_code)]
    pub async fn from_bytes<T: AsyncReadExt + Sized + Unpin>(
        input: &mut T,
    ) -> Result<Update, Error> {
        let withdrawn_count = input.read_u8().await?;
        let mut routes: Vec<Route> = Vec::with_capacity(withdrawn_count as usize);

        for _ in 0..withdrawn_count {
            let route = Route::from_bytes(input).await?;
            routes.push(route);
        }

        Ok(Update {
            withdrawn_routes: routes,
            ..Default::default()
        })
    }
}

impl Default for Update {
    fn default() -> Self {
        Update {
            withdrawn_routes: Vec::with_capacity(0),
            path_attributes: Vec::with_capacity(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use tokio::io::*;

    const DOCUMENTATION_ASN: ASN = 64511;

    #[test]
    fn test_from_bytes() {
        let mut input = Cursor::new(b"\x04\xFB\xFF\x01\x00\x00\x00\x00\x16\x00\x00");
        let update = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(Update::from_bytes(&mut input))
            .unwrap();

        assert_eq!(update.withdrawn_routes.len(), 4);
    }
}
