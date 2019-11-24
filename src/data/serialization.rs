use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::ser::SerializeStruct;

use super::{
    ProtocolHeader,
    FrameProperties,
};


impl Serialize for ProtocolHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
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
        S: Serializer
    {
        let mut fp = serializer.serialize_struct("FrameProperties", 2)?;
        fp.serialize_field("channel", &self.channel)?;
        fp.serialize_field("payload_size", &self.payload_size)?;
        fp.end()
    }
}