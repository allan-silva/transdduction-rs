use super::*;

impl From<ProtocolHeader> for Vec<u8> {
    fn from(protocol_header: ProtocolHeader) -> Vec<u8> {
        let mut bytes = vec![];

        for b in protocol_header.amqp_literal.bytes() {
            bytes.push(b);
        }

        bytes.extend(&protocol_header.id.to_be_bytes());
        bytes.extend(&protocol_header.major.to_be_bytes());
        bytes.extend(&protocol_header.minor.to_be_bytes());
        bytes.extend(&protocol_header.revision.to_be_bytes());

        bytes
    }
}

impl From<Vec<u8>> for ProtocolHeader {
    fn from(bytes: Vec<u8>) -> ProtocolHeader {
        let amqp_literal = String::from_utf8(bytes[0..4].to_vec()).unwrap();

        let mut arr: [u8; 1] = [0];
        arr.copy_from_slice(&bytes[4..5]);

        let id = u8::from_be_bytes(arr);

        let mut arr: [u8; 1] = [0];
        arr.copy_from_slice(&bytes[5..6]);

        let major = u8::from_be_bytes(arr);

        let mut arr: [u8; 1] = [0];
        arr.copy_from_slice(&bytes[6..7]);

        let minor = u8::from_be_bytes(arr);

        let mut arr: [u8; 1] = [0];
        arr.copy_from_slice(&bytes[7..8]);

        let revision = u8::from_be_bytes(arr);

        ProtocolHeader {
            amqp_literal,
            id,
            major,
            minor,
            revision
        }
    }
}