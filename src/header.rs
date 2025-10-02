use crate::packet_buffer::{PacketBuffer, PacketError};

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
            0 | _ => ResponseCode::FORMERR,
        }
    }
}

pub struct Header {
    pub id: u16,
    pub query_response: bool,
    pub operation_code: u8,
    pub authoritative_answer: bool,
    pub truncated_message: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub reserved: bool,
    pub response_code: ResponseCode,
    pub questions: u16,
    pub answers: u16,
    pub authoritive_entries: u16,
    pub resource_entries: u16,
}

impl Header {
    fn new() -> Header {
        Header {
            id: 0,
            query_response: false,
            operation_code: 0,
            authoritative_answer: false,
            truncated_message: false,
            recursion_desired: false,
            recursion_available: false,
            reserved: false,
            response_code: ResponseCode::NOERROR,
            questions: 0,
            answers: 0,
            authoritive_entries: 0,
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
        header.reserved = (last_flags & 0b0111_0000) > 0;
        header.response_code = ResponseCode::from_byte(last_flags & 0b0000_1111);

        header.questions = buffer.read_u16()?;
        header.answers = buffer.read_u16()?;
        header.authoritive_entries = buffer.read_u16()?;
        header.resource_entries = buffer.read_u16()?;

        Ok(header)
    }
}
