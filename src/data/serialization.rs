use std::io::Write;

use serde;
use serde::ser::{SerializeSeq, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{DecimalValue, FrameProperties, ProtocolHeader, ShortString};

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
        let content_bytes = self.content.as_bytes();
        let mut ss = serializer.serialize_struct("ShortString", 1 + content_bytes.len())?;
        
        ss.serialize_field("length", &self.length)?;
        
        for b in content_bytes {
            ss.serialize_field("content", &b)?;
        }

        ss.end()
    }
}
