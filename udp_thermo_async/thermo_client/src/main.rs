use std::io;
use std::time::Duration;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::sync::Mutex;

struct UdpClient {
    socket: Mutex<UdpSocket>,
}

impl UdpClient {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = Mutex::new(UdpSocket::bind(addr).await?);
        Ok(Self { socket })
    }

    /// Читает данные из сокета.
    pub async fn receive(&self) -> io::Result<(Vec<u8>, std::net::SocketAddr)> {
        let mut buf = [0; 4]; // Буфер для хранения входящих данных.
        let (amt, src) = self.socket.lock().await.recv_from(&mut buf).await?;
        Ok((buf[..amt].to_vec(), src))
    }

    /// Отправляет данные на указанный адрес.
    pub async fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], target: A) -> io::Result<()> {
        self.socket.lock().await.send_to(buf, target).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let client = UdpClient::bind("127.0.0.1:00000")
        .await
        .unwrap_or_else(|e| {
            println!("Ошибка при попытке привязки к адресу: {}", e);
            std::process::exit(1); // Завершаем программу с кодом ошибки 1
        });
    let server_addr = "127.0.0.1:34254";
    let clone_client = client;
    tokio::spawn(async move {
        loop {
            clone_client
                .send_to(b"temp", server_addr)
                .await
                .unwrap_or_else(|e| println!("Ошибка при получении данных: {}", e));
            match clone_client.receive().await {
                Ok((data, src)) => {
                    println!("Текущая температура {} ({})", data.last().unwrap(), src);
                }
                Err(e) => println!("Ошибка при получении данных: {}", e),
            }
            tokio::time::sleep(Duration::from_secs_f32(1.0)).await;
        }
    });
    tokio::time::sleep(Duration::from_secs_f32(10.0)).await;
}
