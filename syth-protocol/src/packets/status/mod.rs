use crate::{reader::Reader, writer::Writer};

// Player Identification
pub struct Request;

// Server Identification
pub struct Response {}

pub struct Ping;

impl Response {
    pub fn new(server_name: String, motd: String, user_type: u8) -> Self {
        Self {
            server_name,
            motd,
            user_type,
        }
    }
}

impl Packet for Response {
    fn encode(&self, writer: &mut Writer) -> std::io::Result<()> {
        writer.write_byte(0x00);
        writer.write_byte(7);
        writer.write_string(Self::server_name);
        writer.write_string(Self::motd);
        writer.write_byte(Self::user_type);
        Ok(())
    }
}
