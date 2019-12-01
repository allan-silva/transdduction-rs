mod format;
mod methods;
mod serialization;
#[cfg(test)]
mod tests;

use serde::Serialize;

static FRAME_END: u8 = 0xCE;

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

type ClassId = u16;

type MethodId = u16;

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
    content: &'a str,
}

#[derive(Debug)]
struct LongString<'a> {
    length: LongUInt,
    content: &'a str,
}

enum AmqpField<'a> {
    Bit(u8),
    Octect(u8),
    ShortUInt(ShortUInt),
    LongUInt(LongUInt),
    LongLongUInt(LongLongUInt),
    ShortString(ShortString<'a>),
    LongString(LongString<'a>),
    Timestamp(Timestamp),
    FieldTable(FieldTable<'a>),

    Boolean(Boolean),
    ShortShortInt(ShortShortInt),
    ShortShortUInt(ShortShortUInt),
    ShortInt(ShortInt),
    LongInt(LongInt),
    LongLongInt(LongLongInt),
    Float(Float),
    Double(Double),
    DecimalValue(DecimalValue),
    FieldArray(FieldArray<'a>),
}

type FieldName<'a> = ShortString<'a>;

struct FieldValue<'a> {
    id: char,
    value: AmqpField<'a>,
}

struct Field<'a> {
    name: &'a FieldName<'a>,
    value: FieldValue<'a>,
}

struct FieldTable<'a> {
    fields: Vec<Field<'a>>,
}

struct FieldArray<'a> {
    values: Vec<FieldValue<'a>>,
}
