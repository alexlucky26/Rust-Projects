mod handler;
mod smart_socket;

use crate::handler::{Request, RequestHandler};
use crate::smart_socket::SmartSocket;
use std::error::Error;
use std::sync::Arc;

use stp::server::{StpConnection, StpServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = String::from("127.0.0.1:55331");
    let server = StpServer::bind(addr).await?;
    let socket = Arc::new(SmartSocket::new(
        false,
        16,
        "Socket_1".to_string(),
        "Kitchen".to_string(),
    ));
    println!("Server started...");
    loop {
        let connection = server.accept().await?;

        let addr = connection.peer_addr().await?;

        let socket = socket.clone();

        tokio::spawn(async move {
            match handle_connection(connection, socket).await {
                Ok(_) => {
                    println!("New client connected: {}", addr);
                }
                Err(_) => {
                    println!("Client disconnected: {}", addr);
                }
            }
        });
    }
}

async fn handle_connection(
    mut connection: StpConnection,
    socket: Arc<SmartSocket>,
) -> Result<(), anyhow::Error> {
    let mut handler = RequestHandler::new(socket);
    loop {
        let req_str = connection.recv_request().await?;
        let req = Request::new(&req_str);
        connection.send_response(handler.handle(req)).await?;
    }
}
