const JUMPS_ALLOWED: usize = 8;

#[derive(Debug)]
pub enum PacketError {
    BufferSizeExceeded(usize),
    JumpsExceeded(usize),
}

pub struct PacketBuffer {
    buffer: [u8; 512],
    position: usize,
}

impl PacketBuffer {
    pub fn new() -> PacketBuffer {
        PacketBuffer {
            buffer: [0; 512],
            position: 0,
        }
    }

    fn walk(&mut self, steps: usize) {
        self.position += steps;
    }

    fn seek(&mut self, offset: usize) {
        self.position = offset;
    }

    // Read byte value and increment position
    fn read_byte(&mut self) -> Result<u8, PacketError> {
        if self.position >= self.buffer.len() {
            return Err(PacketError::BufferSizeExceeded(self.position));
        }

        let byte_read = self.buffer[self.position];
        self.position += 1;

        Ok(byte_read)
    }

    // Return byte value without changing position
    fn peek_byte(&self, position: usize) -> Result<u8, PacketError> {
        if position >= self.buffer.len() {
            return Err(PacketError::BufferSizeExceeded(position));
        }

        let byte_read = self.buffer[position];
        Ok(byte_read)
    }

    fn read_range(&self, start_position: usize, length: usize) -> Result<&[u8], PacketError> {
        let end_position = start_position + length;
        if end_position > self.buffer.len() {
            return Err(PacketError::BufferSizeExceeded(end_position));
        }

        let slice = &self.buffer[start_position..end_position];
        Ok(slice)
    }

    fn read_u16(&mut self) -> Result<u16, PacketError> {
        let read = ((self.read_byte()? as u16) << 8) | (self.read_byte()? as u16);
        Ok(read)
    }

    fn read_u32(&mut self) -> Result<u32, PacketError> {
        let read = ((self.read_byte()? as u32) << 24)
            | ((self.read_byte()? as u32) << 16)
            | ((self.read_byte()? as u32) << 8)
            | (self.read_byte()? as u32);

        Ok(read)
    }

    fn qname(&mut self) -> Result<String, PacketError> {
        let mut labels: Vec<String> = Vec::new();
        let mut position = self.position;

        // jump tracker
        let mut jumped = false;
        let mut jump_count = 0;

        loop {
            if jump_count > JUMPS_ALLOWED {
                return Err(PacketError::JumpsExceeded(position));
            }

            let length = self.peek_byte(position)?;
            if length == 0 {
                break;
            }

            // if the two most significant bits are 1, next 14 bits it's an offset
            if (length & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(position + 2);
                }

                let next_byte = self.peek_byte(position + 1)? as u16;
                let offset = (((length as u16) & 0x3F) << 8) | next_byte;
                position = offset as usize;

                jumped = true;
                jump_count += 1;
                continue;
            }

            position += 1;
            let bytes = self.read_range(position, length as usize)?;
            let label = String::from_utf8_lossy(bytes).to_lowercase();
            labels.push(label);
            position += length as usize;
        }

        if !jumped {
            self.seek(position);
        }

        let qname = labels.join(".");
        Ok(qname)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qname_simple() {
        let mut buffer = PacketBuffer::new();

        // www.example.com
        buffer.buffer[0..17].copy_from_slice(&[
            3, b'w', b'w', b'w', // label "www"
            7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', // label "example"
            3, b'c', b'o', b'm', // label "com"
            0,    // terminator
        ]);

        let name = buffer.qname().unwrap();
        assert_eq!(name, "www.example.com");
        assert_eq!(buffer.position, 16);
    }

    #[test]
    fn test_qname_jumps_exceeded() {
        let mut buffer = PacketBuffer::new();

        // Loop jump
        buffer.buffer[0] = 0xC0;
        buffer.buffer[1] = 0;

        buffer.position = 0;
        let result = buffer.qname();
        assert!(matches!(result, Err(PacketError::JumpsExceeded(_))));
    }
}
