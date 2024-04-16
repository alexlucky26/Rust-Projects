use std::net::{ToSocketAddrs, UdpSocket};


use std::{io, thread};


struct UdpClient {
    socket: UdpSocket,
}

impl UdpClient {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self { socket })
    }

    /// Читает данные из сокета.
    pub fn receive(&self) -> io::Result<(Vec<u8>, std::net::SocketAddr)> {
        let mut buf = [0; 4]; // Буфер для хранения входящих данных.
        let (amt, src) = self.socket.recv_from(&mut buf)?;
        Ok((buf[..amt].to_vec(), src))
    }

    /// Отправляет данные на указанный адрес.
    pub fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], target: A) -> io::Result<()> {
        self.socket.send_to(buf, target)?;
        Ok(())
    }
}

fn main() {
    let client = UdpClient::bind("127.0.0.1:34255").unwrap_or_else(|e| {
        println!("Ошибка при попытке привязки к адресу: {}", e);
        std::process::exit(1); // Завершаем программу с кодом ошибки 1
    });
    let server_addr = "127.0.0.1:34254";
    thread::spawn(move || loop {
        client.send_to(b"temp", server_addr).unwrap_or_else(|e| println!("Ошибка при получении данных: {}", e));
        match client.receive() {
            Ok((data, src)) => {
                println!("Текущая температура {:?} ({})", data, src);
            }
            Err(e) => println!("Ошибка при получении данных: {}", e),
        }
    });
}
