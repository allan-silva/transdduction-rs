use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

use super::*;
use crate::configuration::Config;

impl ConnectionManager {
    pub fn new(config: Box<dyn Config>) -> ConnectionManager {
        ConnectionManager {
            config
        }
    }

    pub fn send(&self, data: Vec<u8>) -> io::Result<Vec<u8>> {
        let mut stream = match self.config.get("remote_server") {
            Some(remote_server) => TcpStream::connect(remote_server)?,
            _ => panic!("Remote server not configured")
        };

        stream.write(&data)?;

        let mut buffer: [u8; 1024] = [0; 1024];
        let mut response_data = vec![];

        while let Ok(returned_bytes) = stream.read(&mut buffer) {
            if returned_bytes <= 0 {
                break;
            }
            response_data.extend_from_slice(&buffer[0..returned_bytes]);
        }

        Ok(response_data)
    }
}