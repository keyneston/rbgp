use crate::messages::{Error, Message, ASN};
use tokio::net::TcpStream;

#[derive(Debug)]
enum State {
    SessionEstablished,
    SessionError,
}

#[derive(Debug)]
pub struct Session {
    /// Remote is a copy of the remote IP Address and Port
    remote: std::net::SocketAddr,

    /// bgp_id is the session ID sent by the remote
    bgp_id: u32,

    remote_asn: ASN,

    current_state: State,
}

pub async fn handle(mut socket: TcpStream) -> Result<(), Error> {
    &print!("Accepted connection from {:?}", socket.peer_addr(),);

    let message = Message::from_bytes(&mut socket).await?;

    let open = match message {
        Message::Open(o) => o,
        _ => {
            println!("Received a non-open type");
            return Err(Error::new(&format!(
                "received a non-open type: {:?}",
                message
            )));
        }
    };

    let session = Session {
        remote: socket.peer_addr()?,
        bgp_id: open.bgp_id,
        remote_asn: open.asn,
        current_state: State::SessionEstablished,
    };

    println!("Got: {:?}", session);

    print!("Finished connection from {:?}", socket.peer_addr());

    Ok(())
}
