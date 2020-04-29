// Each test file is compiled as its own crate. Not all test files use every
// symbol so Rust complains about unused types and functions. Suppress it.
#![allow(dead_code)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;

use lazy_static::lazy_static;
use log::debug;
use regex::Regex;

use tokio::prelude::*;

pub type Map<K, V> = std::collections::BTreeMap<K, V>;

pub struct ServerTestHelper {
    clients: Map<String, tokio::net::TcpStream>,
    mint: goldenfile::Mint,
    temp_dir: tempfile::TempDir,
    test_dir: std::path::PathBuf,
}

impl ServerTestHelper {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Self {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_dir = std::path::Path::new("tests").join(path);
        let mint = goldenfile::Mint::new(test_dir.clone());

        ServerTestHelper {
            clients: Map::new(),
            mint: mint,
            temp_dir: temp_dir,
            test_dir: test_dir.to_path_buf(),
        }
    }

    pub async fn run(self: &mut Self) {
        // Start server and get the port it's running on.
        let metadata_path = self.temp_dir.path().join("meta.yaml");
        let metadata_str = metadata_path.to_str().unwrap().to_string();
        let _server = mafia(&["host", "--metadata", &metadata_str]);
        while !metadata_path.exists() {
            tokio::time::delay_for(Duration::from_millis(1)).await;
        }
        let metadata_file = File::open(metadata_path).unwrap();
        let metadata: mafia_bin::server::Metadata = serde_yaml::from_reader(metadata_file).unwrap();
        let addr = format!("127.0.0.1:{}", metadata.port);

        // Run clients.
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"^([^:]+):\s+(.+)$").unwrap();
        }
        let input_path = self.test_dir.join("in.actions.txt");
        let input = File::open(input_path.clone()).unwrap();
        for (line_num, line) in BufReader::new(input).lines().enumerate() {
            let line = line.unwrap();
            debug!(">>> {}", line);

            // Parse line.
            let groups = LINE_RE.captures(&line).expect(&format!(
                "{}:{}: Malformed line",
                input_path.to_str().unwrap(),
                line_num + 1
            ));
            let client = groups.get(1).unwrap().as_str();
            let message = groups.get(2).unwrap().as_str().to_string() + "\n";

            // Create new client connection if necessary.
            if !self.clients.contains_key(client) {
                let conn = tokio::net::TcpStream::connect(addr.clone()).await.unwrap();
                self.clients.insert(client.to_string(), conn);
            }

            // Send the message.
            let conn = self.clients.get_mut(client).unwrap();
            conn.write(message.as_bytes()).await.unwrap();
        }

        // TODO: Fix this.
        tokio::time::delay_for(Duration::from_millis(1)).await;
    }
}

pub async fn run_server_test<P: AsRef<std::path::Path>>(path: P) {
    let mut t = ServerTestHelper::new(path);
    t.run().await;
}

pub fn mafia(args: &[&str]) -> tokio::task::JoinHandle<()> {
    let args = [["mafia"].to_vec(), args.to_vec()]
        .concat()
        .iter()
        .map(|a| a.to_string())
        .collect();

    tokio::spawn(mafia_bin::main(args))
}
