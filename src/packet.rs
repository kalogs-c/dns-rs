use crate::header::Header;
use crate::packet_buffer::{PacketBuffer, PacketError};
use crate::question::Question;
use crate::record::Record;

pub struct Packet {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Record>,
    pub authorities: Vec<Record>,
    pub resources: Vec<Record>,
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            header: Header::new(),
            questions: vec![],
            answers: vec![],
            authorities: vec![],
            resources: vec![],
        }
    }

    pub fn from_buffer(buffer: &mut PacketBuffer) -> Result<Packet, PacketError> {
        let mut packet = Packet::new();
        packet.header = Header::from_buffer(buffer)?;

        for _ in 0..packet.header.questions {
            let question = Question::from_buffer(buffer)?;
            packet.questions.push(question);
        }

        for _ in 0..packet.header.answers {
            let record = Record::from_buffer(buffer)?;
            packet.answers.push(record);
        }

        for _ in 0..packet.header.authoritative_entries {
            let record = Record::from_buffer(buffer)?;
            packet.authorities.push(record);
        }

        for _ in 0..packet.header.resource_entries {
            let record = Record::from_buffer(buffer)?;
            packet.resources.push(record);
        }

        Ok(packet)
    }
}
