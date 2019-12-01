use bincode;

use std::mem;

use super::{
    AmqpField, Channel, DecimalValue, FrameProperties, LongString, LongUInt, PayloadSize,
    ProtocolHeader, Scale, ShortString,
};

fn assert_len<T>(v: &Vec<T>, size: usize) {
    assert_eq!(v.len(), size);
}

fn get_byte_array<T>(v: usize) -> Vec<u8> {
    let mut byte_array: Vec<u8> = Vec::new();

    for i in 0..mem::size_of::<T>() {
        let b = ((v >> i * 8) & 0xFF) as u8;
        byte_array.push(b);
    }

    byte_array
}

fn assert_vec_data(left: &[u8], right: &[u8]) {
    assert_eq!(left.len(), right.len());
    for i in 0..left.len() {
        assert_eq!(left[i], right[i]);
    }
}

#[test]
fn test_protocol_header() {
    let protocol_header: ProtocolHeader = Default::default();

    assert_eq!("AMQP", format!("{}", protocol_header.literal_amqp));
    assert_eq!('0', protocol_header.protocol_id);
    assert_eq!("091", format!("{}", protocol_header.protocol_version));
    assert_eq!("AMQP0091", protocol_header.str_repr());
}

#[test]
fn test_protocol_header_serialization() {
    let protocol_header: ProtocolHeader = Default::default();
    let serialized_ph_repr = bincode::serialize(&protocol_header).unwrap();
    let ph_bytes = vec![65u8, 77u8, 81u8, 80u8, 48u8, 48u8, 57u8, 49u8];

    assert_len(&serialized_ph_repr, 8);

    assert_vec_data(&ph_bytes, &serialized_ph_repr);
}

#[test]
fn test_frame_properties_serialization() {
    let frame_properties = FrameProperties::new(std::u16::MIN, std::u32::MAX);
    let serialized_fp = bincode::serialize(&frame_properties).unwrap();

    let mem_size = 6;

    assert_len(&serialized_fp, mem_size);

    let channel_bytes = get_byte_array::<Channel>(frame_properties.channel as usize);

    assert_vec_data(&channel_bytes, &serialized_fp[..2]);

    let payload_size_bytes = get_byte_array::<PayloadSize>(frame_properties.payload_size as usize);

    assert_vec_data(&payload_size_bytes, &serialized_fp[2..]);
}

#[test]
fn test_decimal_value_serialization() {
    let decimal_value = DecimalValue::new(std::u8::MAX, 420000024u32);
    let serialized_dv = bincode::serialize(&decimal_value).unwrap();

    let mem_size = 5;

    assert_len(&serialized_dv, mem_size);

    let scale_bytes = get_byte_array::<Scale>(decimal_value.scale as usize);

    assert_vec_data(&scale_bytes, &serialized_dv[..1]);

    let value_bytes = get_byte_array::<LongUInt>(decimal_value.value as usize);

    assert_vec_data(&value_bytes, &serialized_dv[1..]);
}

#[test]
fn test_short_string_serialization() {
    let s = "Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Univers";
    let short_str = ShortString::new(s.len() as u8, &s).unwrap();
    let serialized_short_string = bincode::serialize(&short_str).unwrap();

    let mem_size = 1 + short_str.content.len();

    assert_len(&serialized_short_string, mem_size);

    let length_bytes = get_byte_array::<u8>(short_str.length as usize);
    assert_vec_data(&length_bytes, &serialized_short_string[..1]);

    assert_vec_data(short_str.content.as_bytes(), &serialized_short_string[1..]);
}

#[test]
fn test_short_string_serialization_fail() {
    let s = "Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe";
    assert_eq!(
        ShortString::new(s.len() as u8, &s).unwrap_err(),
        "Invalid content size"
    );
}

#[test]
fn test_long_string_serialization() {
    let s = "Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe";
    let long_string = LongString::new(s.len() as LongUInt, &s).unwrap();
    let serialized_long_string = bincode::serialize(&long_string).unwrap();
    let length_field_mem_size = mem::size_of::<LongUInt>();
    let mem_size = length_field_mem_size + long_string.content.len();

    assert_len(&serialized_long_string, mem_size);

    let length_bytes = get_byte_array::<LongUInt>(long_string.length as usize);
    assert_vec_data(
        &length_bytes,
        &serialized_long_string[..length_field_mem_size],
    );

    assert_vec_data(
        long_string.content.as_bytes(),
        &serialized_long_string[length_field_mem_size..],
    );
}

#[test]
fn test_long_string_serialization_fail() {
    let s = "Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe and Everything Life, the Universe";
    assert_eq!(
        LongString::new(1 + s.len() as LongUInt, &s).unwrap_err(),
        "Invalid content size"
    );
}

#[test]
fn test_amqp_field() {
    let v = vec![
        AmqpField::Bit(u8::max_value()),
        AmqpField::Octect(u8::max_value()),
        AmqpField::ShortUInt(u16::max_value()),
        AmqpField::LongUInt(u32::max_value()),
        AmqpField::ShortString(ShortString::new(7, "moz://a").unwrap()),
        AmqpField::LongString(LongString::new(7, "moz://a").unwrap()),
        AmqpField::Timestamp(u64::max_value()),
    ];
}
