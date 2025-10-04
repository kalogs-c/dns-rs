use crate::packet_buffer::{PacketBuffer, PacketError};
use crate::question::QueryType;
use std::net::Ipv4Addr;

pub enum Record {
    Unknown {
        domain: String,
        query_type: u16,
        data_length: u16,
        ttl: u32,
    },
    A {
        domain: String,
        address: Ipv4Addr,
        ttl: u32,
    },
}

impl Record {
    pub fn from_buffer(buffer: &mut PacketBuffer) -> Result<Record, PacketError> {
        let domain = buffer.qname()?;

        let query_type_u16 = buffer.read_u16()?;
        let query_type = QueryType::from_u16(query_type_u16);
        let _ = buffer.read_u16()?;
        let ttl = buffer.read_u32()?;
        let data_length = buffer.read_u16()?;

        let record = match query_type {
            QueryType::A => {
                let raw_addr = buffer.read_u32()?;
                let address = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    (raw_addr & 0xFF) as u8,
                );

                Record::A {
                    domain,
                    address,
                    ttl,
                }
            }
            QueryType::Unknown(_) => {
                buffer.walk(data_length as usize);
                
                Record::Unknown {
                    domain,
                    query_type: query_type_u16,
                    data_length,
                    ttl,
                }
            }
        };

        Ok(record)
    }
}
