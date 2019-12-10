use crate::messages::{Error, Open};
use tokio::net::TcpStream;

pub struct Connection {
    remote: String,
    pub stream: TcpStream,
}

impl Connection {
    pub async fn new(addr: &str) -> Result<Self, Error> {
        let mut stream = TcpStream::connect(addr).await.unwrap();

        println!("Writing open packet");
        // TODO: write open packet to stream
        let o = Open {
            ..Default::default()
        };
        o.write_bytes(&mut stream).await?;

        Ok(Connection {
            remote: addr.to_string(),
            stream: stream,
        })
    }
}
