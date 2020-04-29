use tokio::net::TcpListener;
use tokio::prelude::*;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(address: &str) -> Result<Server, io::Error> {
        let listener = TcpListener::bind(address).await?;
        Ok(Server { listener: listener })
    }

    pub async fn run(self: &mut Self) -> Result<Server, io::Error> {
        loop {
            let (mut socket, _) = self.listener.accept().await.unwrap();

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    match socket.read(&mut buf).await {
                        Ok(0) => return,
                        Ok(n) => {
                            eprintln!("{:?}", std::str::from_utf8(&buf[0..n]));
                        }
                        Err(e) => {
                            eprintln!("Error reading from socket: {:?}", e);
                            return;
                        }
                    };
                }
            });
        }
    }
}
