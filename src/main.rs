mod messages;
mod rbgp;



use tokio::net::{TcpListener};

#[tokio::main]
async fn main() {
    if false {
        println!("Starting server...");

        let addr = "127.0.0.1:6142";
        let listener = TcpListener::bind(addr).await.unwrap();

        let server = async move {
            // Wait for connections. On receiving a connection spawn it off into a new "green thread"
            // and run the handler.
            rbgp::run_loop::run_loop(listener).await;
        };

        println!("Server is running on {0}", addr);
        server.await;
        println!("Server is shutting down");
    } else {
        let addr = "localhost:2179";

        println!("Making connection to: {}", addr);
        let _connection = rbgp::connection::Connection::new(addr).await.unwrap();
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
