use crate::packet_buffer::{PacketBuffer, PacketError};

#[derive(Debug, PartialEq)]
pub enum ResponseCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMPLEMENTED = 4,
    REFUSED = 5,
}

impl ResponseCode {
    pub fn from_byte(b: u8) -> ResponseCode {
        match b {
            1 => ResponseCode::FORMERR,
            2 => ResponseCode::SERVFAIL,
            3 => ResponseCode::NXDOMAIN,
            4 => ResponseCode::NOTIMPLEMENTED,
            5 => ResponseCode::REFUSED,
            0 | _ => ResponseCode::NOERROR,
        }
    }
}

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub query_response: bool,
    pub operation_code: u8,
    pub authoritative_answer: bool,
    pub truncated_message: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub checking_disabled: bool,
    pub authed_data: bool,
    pub z: bool,
    pub response_code: ResponseCode,
    pub questions: u16,
    pub answers: u16,
    pub authoritative_entries: u16,
    pub resource_entries: u16,
}

impl Header {
    pub fn new() -> Header {
        Header {
            id: 0,
            query_response: false,
            operation_code: 0,
            authoritative_answer: false,
            truncated_message: false,
            recursion_desired: false,
            recursion_available: false,
            checking_disabled: false,
            authed_data: false,
            z: false,
            response_code: ResponseCode::NOERROR,
            questions: 0,
            answers: 0,
            authoritative_entries: 0,
            resource_entries: 0,
        }
    }

    pub fn from_buffer(buffer: &mut PacketBuffer) -> Result<Header, PacketError> {
        let mut header = Header::new();
        header.id = buffer.read_u16()?;

        let flags = buffer.read_byte()?;
        header.query_response = (flags & 0b1000_0000) > 0;
        header.operation_code = (flags & 0b0111_1000) >> 3;
        header.authoritative_answer = (flags & 0b0000_0100) > 0;
        header.truncated_message = (flags & 0b0000_0010) > 0;
        header.recursion_desired = (flags & 0b0000_0001) > 0;

        let last_flags = buffer.read_byte()?;
        header.recursion_available = (last_flags & 0b1000_0000) > 0;
        header.checking_disabled = (last_flags & 0b0100_0000) > 0;
        header.authed_data = (last_flags & 0b0010_0000) > 0;
        header.z = (last_flags & 0b0001_0000) > 0;
        header.response_code = ResponseCode::from_byte(last_flags & 0b0000_1111);

        header.questions = buffer.read_u16()?;
        header.answers = buffer.read_u16()?;
        header.authoritative_entries = buffer.read_u16()?;
        header.resource_entries = buffer.read_u16()?;

        Ok(header)
    }
}

#[test]
fn test_header_from_buffer_basic() {
    let data = vec![
        0x1A, 0x2B, 0x85, 0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
    ];

    let mut buffer = PacketBuffer::new();
    buffer.write(&data).unwrap();
    buffer.seek(0);

    let header = Header::from_buffer(&mut buffer).expect("Header parsing failed");

    assert_eq!(header.id, 0x1A2B);
    assert!(header.query_response);
    assert_eq!(header.operation_code, 0);
    assert!(header.authoritative_answer);
    assert!(!header.truncated_message);
    assert!(header.recursion_desired);
    assert!(header.recursion_available);
    assert!(!header.checking_disabled);
    assert!(!header.authed_data);
    assert!(!header.z);
    assert_eq!(header.response_code, ResponseCode::NOERROR);
    assert_eq!(header.questions, 1);
    assert_eq!(header.answers, 1);
    assert_eq!(header.authoritative_entries, 0);
    assert_eq!(header.resource_entries, 0);
}
