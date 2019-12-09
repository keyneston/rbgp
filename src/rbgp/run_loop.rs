use super::session;
use crate::messages::Error;
use futures::StreamExt;
use tokio::net::TcpListener;

/// run_loop is the main loop for the program.
pub async fn run_loop(mut listener: TcpListener) -> Result<(), Error> {
    let mut incoming = listener.incoming();

    while let Some(socket_res) = incoming.next().await {
        match socket_res {
            Ok(socket) => {
                tokio::spawn(async move {
                    session::handle(socket).await;
                });
            }
            Err(err) => {
                println!("accept error = {:?}", err);
            }
        }
    }

    Ok(())
}
