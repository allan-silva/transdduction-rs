mod format;
mod methods;
mod serialization;
#[cfg(test)]
mod tests;

use serde::Serialize;

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

type StringChar = str;

type Channel = ShortUInt;

type Scale = u8;

type PayloadSize = LongUInt;

struct DecimalValue {
    scale: Scale,
    value: LongUInt,
}

struct FrameProperties {
    channel: Channel,
    payload_size: PayloadSize,
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

#[derive(Debug)]
struct ShortString<'a> {
    length: u8,
    content: &'a StringChar
}
