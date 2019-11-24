use bincode;

use super::ProtocolHeader;


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

    assert_eq!(8, serialized_ph_repr.len());

    assert_eq!(65u8, serialized_ph_repr[0]);
    assert_eq!(77u8, serialized_ph_repr[1]);
    assert_eq!(81u8, serialized_ph_repr[2]);
    assert_eq!(80u8, serialized_ph_repr[3]);

    assert_eq!(48u8, serialized_ph_repr[4]);

    assert_eq!(48u8, serialized_ph_repr[5]);
    assert_eq!(57u8, serialized_ph_repr[6]);
    assert_eq!(49u8, serialized_ph_repr[7]);
}