use std::mem;

use serde;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{DecimalValue, FrameProperties, LongString, LongUInt, ProtocolHeader, ShortString};

impl Serialize for ProtocolHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ph = serializer.serialize_struct("ProtocolHeader", 3)?;
        ph.serialize_field("literal_amqp", &self.literal_amqp)?;
        ph.serialize_field("protocol_id", &self.protocol_id)?;
        ph.serialize_field("protocol_version", &self.protocol_version)?;
        ph.end()
    }
}

impl Serialize for FrameProperties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut fp = serializer.serialize_struct("FrameProperties", 2)?;
        fp.serialize_field("channel", &self.channel)?;
        fp.serialize_field("payload_size", &self.payload_size)?;
        fp.end()
    }
}

impl Serialize for DecimalValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut dv = serializer.serialize_struct("DecimalValue", 2)?;
        dv.serialize_field("scale", &self.scale)?;
        dv.serialize_field("value", &self.value)?;
        dv.end()
    }
}

impl<'a> Serialize for ShortString<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ss = serializer.serialize_struct(
            "ShortString",
            mem::size_of_val(&self.length) + self.length as usize,
        )?;
        ss.serialize_field("length", &self.length)?;
        for b in self.content.as_bytes() {
            ss.serialize_field("content", &b)?;
        }

        ss.end()
    }
}

impl<'a> Serialize for LongString<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ls = serializer.serialize_struct(
            "LongString",
            mem::size_of_val(&self.length) + self.length as usize,
        )?;
        ls.serialize_field("length", &self.length)?;
        for b in self.content.as_bytes() {
            ls.serialize_field("content", &b)?;
        }

        ls.end()
    }
}
