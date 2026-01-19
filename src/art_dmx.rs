
use crate::{EstaManCode, OemCode, PortAddress, OpCode};

/// Packet containing dmx data.
pub struct ArtDmx {
    /// Version of the Artnet Protocol used
    pub protocol_version: u16,
    /// Incrementing number for correct packet ordering
    pub sequence: u8,
    /// Physical input port from wicht DMX data was input.
    pub physical: u8,
    /// Port address to wich this packet is destined
    pub port_address: PortAddress,
    /// Length of the DMX data
    pub length: u16,
    /// DMX data
    pub data: Vec<u8>,
}

impl ArtDmx {

    pub fn parse(data: &[u8]) -> Result<ArtDmx, &'static str> {
        let mut parsed = ArtDmx {
            protocol_version: u16::from_le_bytes(data[10..12].try_into().or(Err("Malformed Packet"))?),
            sequence: data[12],
            physical: data[13],
            port_address: PortAddress(0),
            length: u16::from_le_bytes(data[16..18].try_into().or(Err("Malformed Packet"))?),
            data: data[18..].to_vec(),
        };
        return Ok(parsed);
    }

    /// Serializes an ArtDmx Packet to raw data for sending
    pub fn serialize(self) -> Vec<u8> {
        todo!()
    }
}