mod format;
mod serialization;

use serde::{Serialize};


type ShortUInt = u16;


type LongUInt = u32;


type LongLongUInt = u64;


type Timestamp = LongLongUInt;


type Boolean = u8;


type ShortShortUInt = u8;


type ShortShortInt = i8;


type ShortInt = i16;


type LongInt = i32;


type LongLongInt = i64;


type Float = f32;


type Double = f64;


type Channel = ShortUInt;


type Scale = u8;


type PayloadSize = LongUInt;


struct DecimalValue {
    scale: Scale,
    value: LongUInt,
}


impl DecimalValue {
    fn new(scale: Scale, value: LongUInt) -> Self {
        DecimalValue {
            scale,
            value,
        }
    }
}


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