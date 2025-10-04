use std::fs::File;
use std::io::Read;
use crate::packet::Packet;
use crate::packet_buffer::PacketBuffer;

mod header;
mod packet;
mod packet_buffer;
mod question;
mod record;

fn main() {
    let mut f = File::open("dns_query.bin").unwrap();
    let mut buffer = PacketBuffer::new();
    f.read(&mut buffer.buffer).unwrap();

    let packet = Packet::from_buffer(&mut buffer).unwrap();
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }

    for rec in packet.answers {
        println!("{:#?}", rec);
    }

    for rec in packet.authorities {
        println!("{:#?}", rec);
    }

    for rec in packet.resources {
        println!("{:#?}", rec);
    }
}
