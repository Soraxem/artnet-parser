
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
    /// DMX data
    pub data: Vec<u8>,
}


impl ArtDmx {

    pub fn parse(data: &[u8]) -> Result<ArtDmx, &'static str> {
        let mut parsed = ArtDmx {
            protocol_version: u16::from_le_bytes( [data[10], data[11]] ),
            sequence: data[12],
            physical: data[13],
            port_address: PortAddress::unsafe_from_u16(u16::from_le_bytes( [data[14], data[15]] )),
            data: data[18..].to_vec(),
        };
        return Ok(parsed);
    }

    /// Serializes an ArtDmx Packet to raw data for sending
    pub fn serialize(&self) -> Vec<u8> {
        let mut fixed_data = [0u8; 18];

        fixed_data[0..8].copy_from_slice(b"Art-Net\0");
        fixed_data[8..10].copy_from_slice(&(OpCode::OpDmx as u16).to_le_bytes());
        fixed_data[10..12].copy_from_slice(&(self.protocol_version as u16).to_le_bytes());
        fixed_data[12] = self.sequence;
        fixed_data[13] = self.physical;
        fixed_data[14..16].copy_from_slice(&(self.port_address.0 as u16).to_le_bytes());
        fixed_data[16..18].copy_from_slice(&(self.data.len() as u16).to_le_bytes());

        let mut data = fixed_data.to_vec();
        data.extend_from_slice(&self.data);

        return data;
    }
}