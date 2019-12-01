use super::{
    AmqpLiteral, Channel, DecimalValue, FrameProperties, LongString, LongUInt, PayloadSize,
    ProtocolHeader, ProtocolVersion, Scale, ShortString,
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
