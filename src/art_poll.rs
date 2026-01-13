
use crate::{EstaManCode, OemCode, PortAddress};

/// Packet for Polling Nodes in the Network. The Nodes shall resond with a ArtPollReply packet.
pub struct ArtPoll {
    /// Version of the Artnet Protocol used
    pub protocol_version: u16,
    /// Flags to tell the Node what responses to send
    pub flags: ArtPollFlags,
    pub diag_priority: u8,
    /// Port address range top for Polling
    pub target_port_address_top: PortAddress,
    /// Port address range bottom for Polling
    pub target_port_address_bottom: PortAddress,
    /// Manufacturer Code
    pub esta_man: EstaManCode,
    /// Oem Code
    pub oem: OemCode
}

/// The controller sends these flags when polling the nodes in the network. These flags tell the node what responses the controller wants and doesn't want.
pub struct ArtPollFlags {
    /// Send an ArtPollReply instantly when conditions change
    pub send_on_change: bool,
    /// Send diagnostic messages
    pub send_diagnostics: bool,
    /// Send unicast diagnostic messages
    pub unicast_diagnostics: bool,
    pub disable_vlc_transmission: bool,
    pub enable_targeted_mode: bool,
}

impl ArtPoll {
    /// Parse raw data into an ArtPoll Packet. Doesn't check if data even is artnet data.
    pub fn parse(data: &[u8]) -> Result<ArtPoll, &'static str> {

        let mut parsed = ArtPoll {
            protocol_version: u16::from_le_bytes(data[10..12].try_into().or(Err("Malformed Packet"))?),
            flags: ArtPollFlags {
                send_on_change: data[12] & 0x01 == 0x01,
                send_diagnostics: data[12] & 0x02 == 0x02,
                unicast_diagnostics: data[12] & 0x04 == 0x04,
                disable_vlc_transmission: data[12] & 0x08 == 0x08,
                enable_targeted_mode: data[12] & 0x10 == 0x10,
            },
            diag_priority: data[13],

            target_port_address_top: PortAddress(0),
            target_port_address_bottom: PortAddress(0),
            esta_man: EstaManCode(0),
            oem: OemCode(0),
        };

        // TODO: implement optional fields

        Ok(parsed)
    }

    pub fn serialize(self) -> Vec<u8> {
        todo!()
    }
}