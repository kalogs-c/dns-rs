use crate::packet_buffer::{PacketBuffer, PacketError};

#[derive(Debug)]
pub enum QueryType {
    Unknown(u16),
    A,
}

impl QueryType {
    pub fn from_u16(value: u16) -> QueryType {
        match value {
            1 => QueryType::A,
            _ => QueryType::Unknown(value),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            QueryType::A => 1,
            QueryType::Unknown(value) => *value,
        }
    }
}

#[derive(Debug)]
pub struct Question {
    pub name: String,
    pub query_type: QueryType,
}

impl Question {
    pub fn new(name: String, query_type: QueryType) -> Question {
        Question { name, query_type }
    }

    pub fn from_buffer(buffer: &mut PacketBuffer) -> Result<Question, PacketError> {
        let qname = buffer.qname()?;
        let query_type = QueryType::from_u16(buffer.read_u16()?);
        let question = Question::new(qname, query_type);
        Ok(question)
    }
}
