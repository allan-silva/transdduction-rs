use super::*;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer};

impl Default for ProtocolHeader {
    fn default() -> Self {
        ProtocolHeader {
            amqp_litetal: String::from("AMQP"),
            id: 0,
            major: 0,
            minor: 9,
            revision: 1
        }
    }
}

impl Serialize for ProtocolHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut ph = serializer.serialize_struct("ProtocolHeader", 8)?;

        for c in self.amqp_litetal.chars() {
            ph.serialize_field("", &c)?;
        }

        ph.serialize_field("4", &0u8)?;
        ph.serialize_field("5", &0u8)?;
        ph.serialize_field("6", &9u8)?;
        ph.serialize_field("7", &1u8)?;
        
        ph.end()
    }
}

impl<'de> Deserialize<'de> for ProtocolHeader
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        assert!(bytes.len() == 8);
        let amqp = bytes[0..4].to_string();
        print("{}", amqp);

        Ok(Default::default())
    }
}