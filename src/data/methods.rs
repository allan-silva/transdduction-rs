use super::{
    AmqpField, AmqpLiteral, Boolean, Channel, DecimalValue, Double, FieldArray, FieldTable,
    FieldValue, Float, FrameProperties, LongInt, LongLongInt, LongLongUInt, LongString, LongUInt,
    PayloadSize, ProtocolHeader, ProtocolVersion, Scale, ShortInt, ShortShortInt, ShortShortUInt,
    ShortString, ShortUInt, Timestamp,
};

impl DecimalValue {
    pub fn new(scale: Scale, value: LongUInt) -> Self {
        DecimalValue { scale, value }
    }
}

impl FrameProperties {
    pub fn new(channel: Channel, payload_size: PayloadSize) -> Self {
        FrameProperties {
            channel,
            payload_size,
        }
    }
}

impl ProtocolHeader {
    pub fn str_repr(&self) -> String {
        format!(
            "{}{}{}",
            self.literal_amqp, self.protocol_id, self.protocol_version
        )
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

impl<'a> ShortString<'a> {
    pub fn new(length: u8, content: &'a str) -> Result<Self, String> {
        match content.len() {
            l if l == length as usize => Ok(ShortString { length, content }),
            _ => Err(format!("Invalid content size")),
        }
    }
}

impl<'a> LongString<'a> {
    pub fn new(length: LongUInt, content: &'a str) -> Result<Self, String> {
        match content.len() {
            l if l == length as usize => Ok(LongString { length, content }),
            _ => Err(format!("Invalid content size")),
        }
    }
}

impl<'a> FieldValue<'a> {
    pub fn new(id: char, value: AmqpField<'a>) -> Self {
        FieldValue { id, value }
    }

    pub fn from_bool(value: Boolean) -> Self {
        FieldValue::new('t', AmqpField::Boolean(value))
    }

    pub fn short_short_uint(value: ShortShortUInt) -> Self {
        FieldValue::new('B', AmqpField::ShortShortUInt(value))
    }

    pub fn from_long_long_uint(value: LongLongUInt) -> Self {
        FieldValue::new('l', AmqpField::LongLongUInt(value))
    }

    pub fn from_timestamp(value: Timestamp) -> Self {
        FieldValue::new('T', AmqpField::Timestamp(value))
    }
}

impl<'a> From<FieldTable<'a>> for FieldValue<'a> {
    fn from(value: FieldTable<'a>) -> Self {
        FieldValue::new('F', AmqpField::FieldTable(value))
    }
}

impl<'a> From<FieldArray<'a>> for FieldValue<'a> {
    fn from(value: FieldArray<'a>) -> Self {
        FieldValue::new('A', AmqpField::FieldArray(value))
    }
}

impl<'a> From<LongString<'a>> for FieldValue<'a> {
    fn from(value: LongString<'a>) -> Self {
        FieldValue::new('S', AmqpField::LongString(value))
    }
}

impl<'a> From<ShortString<'a>> for FieldValue<'a> {
    fn from(value: ShortString<'a>) -> Self {
        FieldValue::new('s', AmqpField::ShortString(value))
    }
}

impl From<DecimalValue> for FieldValue<'_> {
    fn from(value: DecimalValue) -> Self {
        FieldValue::new('D', AmqpField::DecimalValue(value))
    }
}

impl From<Double> for FieldValue<'_> {
    fn from(value: Double) -> Self {
        FieldValue::new('d', AmqpField::Double(value))
    }
}

impl From<Float> for FieldValue<'_> {
    fn from(value: Float) -> Self {
        FieldValue::new('f', AmqpField::Float(value))
    }
}

impl From<LongLongInt> for FieldValue<'_> {
    fn from(value: LongLongInt) -> Self {
        FieldValue::new('L', AmqpField::LongLongInt(value))
    }
}

impl From<LongUInt> for FieldValue<'_> {
    fn from(value: LongUInt) -> Self {
        FieldValue::new('i', AmqpField::LongUInt(value))
    }
}

impl From<LongInt> for FieldValue<'_> {
    fn from(value: LongInt) -> Self {
        FieldValue::new('I', AmqpField::LongInt(value))
    }
}

impl From<ShortUInt> for FieldValue<'_> {
    fn from(value: ShortUInt) -> Self {
        FieldValue::new('u', AmqpField::ShortUInt(value))
    }
}

impl From<ShortInt> for FieldValue<'_> {
    fn from(value: ShortInt) -> Self {
        FieldValue::new('U', AmqpField::ShortInt(value))
    }
}

impl From<ShortShortInt> for FieldValue<'_> {
    fn from(value: ShortShortInt) -> Self {
        FieldValue::new('b', AmqpField::ShortShortInt(value))
    }
}
