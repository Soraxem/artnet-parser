
//! Library for Parsing Artnet Packets

mod art_poll;
mod art_poll_reply;

pub use crate::art_poll::*;
pub use crate::art_poll_reply::ArtPollReply;

pub struct PortAddress(u16);

pub struct EstaManCode(u16);

pub struct OemCode(u16);


/// Checks if the data is an Artnet Packet
pub fn is_artnet(data: &[u8]) -> bool {
    let art_net_bytes = b"Art-Net\0";
    return data.len() >= art_net_bytes.len() && data[..art_net_bytes.len()] == *art_net_bytes
}

#[repr(u16)]
pub enum OpCode {
    OpPoll = 0x2000,
    OpPollReply = 0x2100,
}

/// fetches the opcode of the packet
pub fn get_op_code(data: &[u8]) -> Result<OpCode, &'static str> {
    let op_code = u16::from_le_bytes(data[8..10].try_into().unwrap());

    match op_code {
        0x2000 => Ok(OpCode::OpPoll),
        0x2100 => Ok(OpCode::OpPollReply),
        _ => Err("Unknown OpCode")
    }
}

/// Holds all artnet Packet types. This is the main entry point for this library.
pub enum ArtNetPacket {
    ArtPoll(ArtPoll),
    ArtPollReply(ArtPollReply),
}


impl ArtNetPacket {

    /// Parses raw data into an ArtNetPacket
    pub fn parse(data: &[u8]) -> Result<ArtNetPacket, &'static str> {
        // Check if data contains an Artnet Packet
        if !is_artnet(data) {
            return Err("Not an Artnet Packet");
        }
        // Fetch OpCode
        let op_code = get_op_code(data);

        match op_code {
            Ok(OpCode::OpPoll) => match ArtPoll::parse(data) {
                Ok(packet) => Ok(ArtNetPacket::ArtPoll(packet)),
                Err(err) => Err(err),
            },
            Ok(OpCode::OpPollReply) => match ArtPollReply::parse(data) {
                Ok(packet) => Ok(ArtNetPacket::ArtPollReply(packet)),
                Err(err) => Err(err),
            },
            // Throw an Error if the OpCode exists but is not implemented
            Ok(_) => Err("Packet not implemented"),
            // Throw an Error if the OpCode is not implemented
            Err(err) => Err(err),
        }
    }

    pub fn serialize(self) -> Vec<u8> {
        match self {
            ArtNetPacket::ArtPoll(packet) => packet.serialize(),
            ArtNetPacket::ArtPollReply(packet) => packet.serialize(),
        }
    }
}
