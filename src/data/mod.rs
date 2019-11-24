mod format;
mod serialization;

use serde::{Serialize};


type Channel = u16;


type PayloadSize = u32;


struct FrameProperties {
    channel: Channel,
    payload_size: PayloadSize,
}


impl FrameProperties {
    fn new(channel: Channel, payload_size: PayloadSize) -> Self {
        FrameProperties {
            channel,
            payload_size,
        }
    }
}


#[derive(Serialize)]
struct ProtocolVersion(char, char, char);


#[derive(Serialize)]
struct AmqpLiteral(char, char, char, char);


struct ProtocolHeader {
    protocol_id: char,
    protocol_version: ProtocolVersion,
    literal_amqp: AmqpLiteral,
}


impl ProtocolHeader {
    fn str_repr(&self) -> String {
        format!("{}{}{}", self.literal_amqp, self.protocol_id, self.protocol_version)
    }
}


impl Default for ProtocolHeader {
    fn default() -> Self {
        ProtocolHeader {
            protocol_id: '0',
            protocol_version: ProtocolVersion('0', '9', '1'),
            literal_amqp: AmqpLiteral('A', 'M', 'Q', 'P'),
        }
    }
}


#[cfg(test)]
mod tests;