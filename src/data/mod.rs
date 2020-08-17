mod methods;
mod serialization;

pub struct ProtocolHeader {
    pub amqp_literal: String,
    pub id: u8,
    pub major: u8,
    pub minor: u8,
    pub revision: u8
}