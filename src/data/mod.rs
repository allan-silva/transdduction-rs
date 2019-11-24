mod format;
mod serialization;

use serde::{Serialize};


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