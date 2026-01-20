
//! Library for Parsing Artnet Packets

pub mod art_poll;
pub mod art_poll_reply;
pub mod art_dmx;

pub use crate::art_poll::*;
pub use crate::art_poll_reply::ArtPollReply;
pub use crate::art_dmx::ArtDmx;


#[derive(Clone, Copy)]
pub struct PortAddress(
    /// Legacy, please do not us the field directly this can cause unexpected behaviour
    pub u16
);

impl PortAddress {

    /// Creates a new PortAddress and checks if the Values are in range
    /// Valid Ranges are:
    /// net: 0 - 127
    /// subnet: 0 - 15
    /// universe: 0 - 15
    pub fn new(net: u8, subnet: u8, universe: u8) -> Result<Self, &'static str> {
        if net > 0xF0 || subnet > 0x0F || universe > 0x0F {
            return Err("A value exeeds limit");
        }
        Ok(Self::unsafe_new(net, subnet, universe))
    }

    /// Creates a new PortAddress without checking if the values exceed their limits. It just removes all unnecessary bits
    /// Using this function can cause unexpected behaviour
    pub fn unsafe_new(net: u8, subnet: u8, universe: u8) -> Self {
        Self( ( (net as u16) << 8) | ( ( (subnet as u16) & 0x0F) << 4 ) | ( (universe as u16) & 0x0F as u16) )
    }

    /// Creates a new PortAddress from a u16 value and checks if the value is in range
    /// The valid Range is:
    /// 0 - 0x7FFF
    pub fn from_u16(value: u16) -> Result<Self, &'static str> {
        if value > 0x7FFF {
            return Err("Value exeeds limit");
        }
        Ok(Self::unsafe_from_u16(value))
    }

    /// Creates a new PortAddress from a u16 value without checking if the value is in range
    /// Using this function can cause unexpected behaviour
    pub fn unsafe_from_u16(value: u16) -> Self {
        Self(value & 0x7FFF)
    }

    pub fn net(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn subnet(self) -> u8 {
        ( (self.0 & 0x00F0) >> 4 ) as u8
    }

    pub fn universe(self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    pub fn as_u16(self) -> u16 {
        self.0
    }
}

impl Default for PortAddress {
    fn default() -> Self {
        Self::unsafe_from_u16(0)
    }
}

#[derive(Clone, Copy)]
pub struct EstaManCode(pub u16);

#[derive(Clone, Copy)]
pub struct OemCode(pub u16);


/// Checks if the data is an Artnet Packet
pub fn is_artnet(data: &[u8]) -> bool {
    let art_net_bytes = b"Art-Net\0";
    return data.len() >= art_net_bytes.len() && data[..art_net_bytes.len()] == *art_net_bytes
}

#[repr(u16)]
pub enum OpCode {
    OpPoll = 0x2000,
    OpPollReply = 0x2100,
    OpDmx = 0x5000,
}

/// fetches the opcode of the packet
pub fn get_op_code(data: &[u8]) -> Result<OpCode, &'static str> {
    let op_code = u16::from_le_bytes(data[8..10].try_into().unwrap());

    match op_code {
        0x2000 => Ok(OpCode::OpPoll),
        0x2100 => Ok(OpCode::OpPollReply),
        0x5000 => Ok(OpCode::OpDmx),
        _ => Err("Unknown OpCode")
    }
}

/// Holds all artnet Packet types. This is the main entry point for this library.
pub enum ArtNetPacket {
    ArtPoll(ArtPoll),
    ArtPollReply(ArtPollReply),
    ArtDmx(ArtDmx),
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
            Ok(OpCode::OpDmx) => match ArtDmx::parse(data) {
                Ok(packet) => Ok(ArtNetPacket::ArtDmx(packet)),
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
            ArtNetPacket::ArtDmx(packet) => packet.serialize(),
        }
    }
}
