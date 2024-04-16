mod handler;
mod smart_socket;

use crate::handler::{Request, RequestHandler};
use crate::smart_socket::SmartSocket;
use std::error::Error;
use std::sync::Arc;
use std::thread;
use stp::server::{StpConnection, StpServer};

fn main() -> Result<(), Box<dyn Error>> {
    let addr = String::from("127.0.0.1:55331");
    let server = StpServer::bind(addr)?;
    let socket = Arc::new(SmartSocket::new(
        false,
        16,
        "Socket_1".to_string(),
        "Kitchen".to_string(),
    ));

    for connection in server.incoming() {
        let connection = match connection {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Can't establish connection: {}", e);
                continue;
            }
        };

        let addr = match connection.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        println!("New client connected: {}", addr);

        let socket = socket.clone();
        thread::spawn(move || {
            if handle_connection(connection, socket).is_err() {
                println!("Client disconnected: {}", addr);
            }
        });
    }
    Ok(())
}

fn handle_connection(
    mut connection: StpConnection,
    socket: Arc<SmartSocket>,
) -> Result<(), anyhow::Error> {
    let mut handler = RequestHandler::new(socket);
    loop {
        let req_str = connection.recv_request()?;
        let req = Request::new(&req_str);
        connection.send_response(handler.handle(req))?;
    }
}
