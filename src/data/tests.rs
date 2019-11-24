use bincode;

use std::mem;

use super::{
    ProtocolHeader,
    FrameProperties,
};


fn assert_len<T>(v: &Vec<T>, size: usize) {
    assert_eq!(size, v.len());
}


fn get_byte_array(v: usize, size: usize) -> Vec<u8> {
    let mut byte_array: Vec<u8> = Vec::new();

    for i in 0..size {
        let b = ((v >> i * 8) & 0xFF) as u8;
        byte_array.push(b);
    }

    byte_array
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

    assert_len(&serialized_ph_repr, 8);

    assert_eq!(65u8, serialized_ph_repr[0]);
    assert_eq!(77u8, serialized_ph_repr[1]);
    assert_eq!(81u8, serialized_ph_repr[2]);
    assert_eq!(80u8, serialized_ph_repr[3]);

    assert_eq!(48u8, serialized_ph_repr[4]);

    assert_eq!(48u8, serialized_ph_repr[5]);
    assert_eq!(57u8, serialized_ph_repr[6]);
    assert_eq!(49u8, serialized_ph_repr[7]);
}


#[test]
fn test_frame_properties_serialization() {
    let frame_properties = FrameProperties::new(42, 42000024);
    let serialized_fp = bincode::serialize(&frame_properties).unwrap();

    assert_len(&serialized_fp, 6);

    let channel_bytes = get_byte_array(
        frame_properties.channel as usize,
        mem::size_of_val(&frame_properties.channel)
    );

    assert_eq!(channel_bytes[0], serialized_fp[0]);
    assert_eq!(channel_bytes[1], serialized_fp[1]);

    let payload_size_bytes = get_byte_array(
        frame_properties.payload_size as usize,
        mem::size_of_val(&frame_properties.payload_size)
    );

    assert_eq!(payload_size_bytes[0], serialized_fp[2]);
    assert_eq!(payload_size_bytes[1], serialized_fp[3]);
    assert_eq!(payload_size_bytes[2], serialized_fp[4]);
    assert_eq!(payload_size_bytes[3], serialized_fp[5]);
}
