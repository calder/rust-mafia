use structopt::StructOpt;
use tokio::net::TcpListener;
use tokio::prelude::*;

use mafia_bin::ui;

/// Rust implementation of the classic party game Mafia.
#[derive(StructOpt)]
struct Mafia {
    /// Subcommand.
    #[structopt(subcommand)]
    cmd: Command,
}

/// Subcommand.
#[derive(StructOpt)]
enum Command {
    /// Join a game.
    Join {
        /// Render the UI once and exit.
        #[structopt(long)]
        smoketest: bool,
    },

    /// Host a game.
    Host {
        /// Start the server then exit.
        #[structopt(long)]
        smoketest: bool,
    },

    /// Print version and exit.
    Version {},
}

#[tokio::main]
async fn main() {
    let opt = Mafia::from_args();

    match opt.cmd {
        Command::Join { smoketest } => {
            let mut app = ui::Client::new().unwrap();

            if smoketest {
                app.draw().unwrap();
            } else {
                app.run().unwrap();
            }
        }

        Command::Host { smoketest } => {
            let mut listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

            if smoketest {
                return;
            }

            loop {
                let (mut socket, _) = listener.accept().await.unwrap();

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

        Command::Version {} => {
            println!("mafia {}", env!("CARGO_PKG_VERSION"));
        }
    }
}
