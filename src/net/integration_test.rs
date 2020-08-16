use std::collections::HashMap;

use bincode::{serialize, deserialize};

use crate::configuration::Config;
use super::ConnectionManager;
use crate::data::ProtocolHeader;

struct ConfigMock {
    configs: HashMap<String, String>
}

impl ConfigMock {
    fn new() -> ConfigMock {
        let mut configs = HashMap::new();
        configs.insert(
            String::from("remote_server"),
            String::from("127.0.0.1:5672")
        );
        ConfigMock {
            configs
        }
    }
}

impl Config for ConfigMock {
    fn get(&self, key: &str) -> Option<String> {
        match self.configs.get(key) {
            Some(value) => Some(String::from(value)),
            None => None
        }
    }
}


#[test]
fn send_protocol_header() {
    let config = box ConfigMock::new();
    let connection_manager = ConnectionManager::new(config);

    let protocol_header = serialize(&ProtocolHeader::default()).unwrap();

    let server_response = connection_manager.send(protocol_header).unwrap();
}

#[test]
fn send_wrong_protocol_header() {
    let config = box ConfigMock::new();
    let connection_manager = ConnectionManager::new(config);
    let protocol_header = ProtocolHeader {
        amqp_litetal: String::from("ZMQP"),
        ..Default::default()
    };
    let protocol_header = serialize(&protocol_header).unwrap();

    let server_response = connection_manager.send(protocol_header).unwrap();

    let supported_protocol: ProtocolHeader = deserialize(&server_response).unwrap();

    assert_eq!(0u8, supported_protocol.id);
    assert_eq!(0u8, supported_protocol.major);
    assert_eq!(9u8, supported_protocol.minor);
    assert_eq!(1u8, supported_protocol.revision);
}