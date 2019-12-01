use std::mem;

use bincode;
use serde;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{
    AmqpField, DecimalValue, Field, FieldArray, FieldTable, FieldValue, FrameProperties,
    LongString, LongUInt, ProtocolHeader, ShortString,
};

fn write_field_bytes<S>(serialize_struct: &mut S, name: &'static str, v: &[u8])
where
    S: SerializeStruct,
{
    for b in v {
        serialize_struct
            .serialize_field(name, &b)
            .expect("Error on serialize bytes field");
    }
}

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

        write_field_bytes(&mut ss, "content", &self.content.as_bytes());

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

        write_field_bytes(&mut ls, "content", &self.content.as_bytes());

        ls.end()
    }
}

impl<'a> Serialize for AmqpField<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value_bytes = match self {
            AmqpField::Bit(value) => bincode::serialize(value).unwrap(),
            AmqpField::Octect(value) => bincode::serialize(value).unwrap(),
            AmqpField::ShortUInt(value) => bincode::serialize(value).unwrap(),
            AmqpField::LongUInt(value) => bincode::serialize(value).unwrap(),
            AmqpField::LongLongUInt(value) => bincode::serialize(value).unwrap(),
            AmqpField::ShortString(value) => bincode::serialize(value).unwrap(),
            AmqpField::LongString(value) => bincode::serialize(value).unwrap(),
            AmqpField::Timestamp(value) => bincode::serialize(value).unwrap(),
            AmqpField::FieldTable(value) => bincode::serialize(value).unwrap(),

            AmqpField::Boolean(value) => bincode::serialize(value).unwrap(),
            AmqpField::ShortShortInt(value) => bincode::serialize(value).unwrap(),
            AmqpField::ShortShortUInt(value) => bincode::serialize(value).unwrap(),
            AmqpField::ShortInt(value) => bincode::serialize(value).unwrap(),
            AmqpField::LongInt(value) => bincode::serialize(value).unwrap(),
            AmqpField::LongLongInt(value) => bincode::serialize(value).unwrap(),
            AmqpField::Float(value) => bincode::serialize(value).unwrap(),
            AmqpField::Double(value) => bincode::serialize(value).unwrap(),
            AmqpField::DecimalValue(value) => bincode::serialize(value).unwrap(),
            AmqpField::FieldArray(value) => bincode::serialize(value).unwrap(),
        };

        let mut af = serializer.serialize_struct("AmqpField", value_bytes.len())?;
        write_field_bytes(&mut af, "AmqpField.0", &value_bytes);

        af.end()
    }
}

impl<'a> Serialize for FieldValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let amqp_field_bytes = bincode::serialize(&self.value).unwrap();

        let mut fv = serializer.serialize_struct(
            "FieldValue",
            mem::size_of_val(&self.id) + amqp_field_bytes.len(),
        )?;

        fv.serialize_field("id", &self.id)?;

        write_field_bytes(&mut fv, "value", &amqp_field_bytes);

        fv.end()
    }
}

impl<'a> Serialize for Field<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let name_bytes = bincode::serialize(&self.name).unwrap();
        let field_value_bytes = bincode::serialize(&self.value).unwrap();
        let mut field =
            serializer.serialize_struct("Field", name_bytes.len() + field_value_bytes.len())?;

        write_field_bytes(&mut field, "name", &name_bytes);
        write_field_bytes(&mut field, "value", &field_value_bytes);

        field.end()
    }
}

impl<'a> Serialize for FieldTable<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized_fields: Vec<Vec<u8>> = self
            .fields
            .iter()
            .map(|field| bincode::serialize(field).unwrap())
            .collect();

        let fields_bytes_size: usize = serialized_fields.iter().map(|field| field.len()).sum();

        let mut ft = serializer.serialize_struct(
            "FieldTable",
            mem::size_of_val(&self.size()) + fields_bytes_size,
        )?;

        ft.serialize_field("size", &self.size())?;

        for serialized_field in serialized_fields {
            write_field_bytes(&mut ft, "fields", &serialized_field)
        }

        ft.end()
    }
}

impl<'a> Serialize for FieldArray<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized_fields: Vec<Vec<u8>> = self
            .values
            .iter()
            .map(|field| bincode::serialize(field).unwrap())
            .collect();

        let fields_bytes_size: usize = serialized_fields.iter().map(|field| field.len()).sum();

        let mut ft = serializer.serialize_struct(
            "FieldArray",
            mem::size_of_val(&self.size()) + fields_bytes_size,
        )?;

        ft.serialize_field("size", &self.size())?;

        for serialized_field in serialized_fields {
            write_field_bytes(&mut ft, "fields", &serialized_field)
        }

        ft.end()
    }
}
