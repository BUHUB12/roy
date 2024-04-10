use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug)]
struct MboHeader {
    frame_length: u16,
    header_length: u16,
    feed_id: u16,
    session_id: u16,
    sequence_number: u32,
    timestamp: u64, // Assuming Time_t is equivalent to u64 for simplicity
    message_count: u8,
}

impl MboHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut rdr = Cursor::new(bytes);

        Ok(MboHeader {
            frame_length: rdr.read_u16::<LittleEndian>()?,
            header_length: rdr.read_u16::<LittleEndian>()?,
            feed_id: rdr.read_u16::<LittleEndian>()?,
            session_id: rdr.read_u16::<LittleEndian>()?,
            sequence_number: rdr.read_u32::<LittleEndian>()?,
            timestamp: rdr.read_u64::<LittleEndian>()?,
            message_count: rdr.read_u8()?,
        })
    }
}
