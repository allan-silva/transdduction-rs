use super::*;

impl Default for ProtocolHeader {
    fn default() -> Self {
        ProtocolHeader {
            amqp_literal: String::from("AMQP"),
            id: 0,
            major: 0,
            minor: 9,
            revision: 1
        }
    }
}
