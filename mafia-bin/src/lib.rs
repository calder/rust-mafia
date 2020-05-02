#[macro_use]
extern crate log;

pub mod client;
pub mod init;
pub mod server;
pub mod util;

use structopt::StructOpt;

use crate::client::Client;
use crate::init::init;
use crate::server::Server;

/// Rust implementation of the classic party game Mafia.
#[derive(StructOpt)]
pub struct Mafia {
    /// Subcommand.
    #[structopt(subcommand)]
    pub cmd: Command,
}

/// Subcommand.
#[derive(StructOpt)]
pub enum Command {
    /// Join a game.
    Join {
        /// Render the UI once and exit.
        #[structopt(long)]
        smoketest: bool,
    },

    /// Host a game.
    Host {
        /// Address to bind to.
        #[structopt(long, default_value = "0.0.0.0:6666")]
        address: String,

        /// Storage directory.
        #[structopt(long, parse(from_os_str), default_value = ".")]
        path: std::path::PathBuf,

        /// Start the server then exit.
        #[structopt(long)]
        smoketest: bool,
    },

    /// Create a game directory for hosting.
    Init {
        /// Storage directory.
        #[structopt(long, parse(from_os_str), default_value = ".")]
        path: std::path::PathBuf,

        /// Starting random seed.
        #[structopt(long)]
        seed: Option<u64>,
    },

    /// Print version and exit.
    Version,
}

pub async fn main(args: Vec<String>) -> Result<(), std::io::Error> {
    // Parse command line args.
    let opt = Mafia::from_iter(args);

    // Initialize logging.
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    match env_logger::try_init_from_env(env) {
        Err(e) => {
            eprintln!("Error initializing logging: {:?}", e);
        }
        Ok(()) => {}
    }

    match opt.cmd {
        Command::Join { smoketest } => {
            let mut app = Client::new()?;

            if smoketest {
                app.draw()?;
                return Ok(());
            }

            app.run()?;
        }

        Command::Host {
            address,
            path,
            smoketest,
        } => {
            let mut server = Server::new(path, &address).await?;

            if smoketest {
                return Ok(());
            }

            server.run().await?;
        }

        Command::Init { path, seed } => {
            init(path, seed);
        }

        Command::Version => {
            println!("mafia {}", env!("CARGO_PKG_VERSION"));
        }
    };

    Ok(())
}
