use crate::messages::{Error, Open};
use std::io::{stdout, Write};
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::time::delay_for;

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

        loop {
            delay_for(Duration::from_secs(60)).await;

            let mut buf: [u8; 1024] = [0; 1024];
            stream.read(&mut buf).await?;

            stdout().write(&buf)?;
        }

        Ok(Connection {
            remote: addr.to_string(),
            stream: stream,
        })
    }
}
