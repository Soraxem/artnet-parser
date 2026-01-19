
use crate::{EstaManCode, OemCode, PortAddress, OpCode};

/// Packet for Polling Nodes in the Network. The Nodes shall resend with a ArtPollReply packet.
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

/// Default values for an ArtPoll Packet
impl Default for ArtPoll {
    fn default() -> Self {
        Self {
            protocol_version: 14,
            flags: ArtPollFlags {
                send_on_change: false,
                send_diagnostics: false,
                unicast_diagnostics: false,
                disable_vlc_transmission: false,
                enable_targeted_mode: false,
            },
            diag_priority: 0,
            target_port_address_top: PortAddress(0),
            target_port_address_bottom: PortAddress(0),
            esta_man: EstaManCode(0),
            oem: OemCode(0),
        }
    }
}

impl ArtPoll {
    /// Parse raw data into an ArtPoll Packet. Doesn't check if data even is artnet data.
    pub fn parse(data: &[u8]) -> Result<ArtPoll, &'static str> {

        let parsed = ArtPoll {
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

    /// Serializes an ArtPoll Packet to raw data for sending
    pub fn serialize(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        data.extend_from_slice(b"Art-Net\0");
        data.extend_from_slice(&(OpCode::OpPoll as u16).to_be_bytes());
        data.extend_from_slice(&(self.protocol_version as u16).to_le_bytes());


        let mut flags: u8 = 0;
        if self.flags.send_on_change {
            flags |= 0x01;
        }
        if self.flags.send_diagnostics {
            flags |= 0x02;
        }
        if self.flags.unicast_diagnostics {
            flags |= 0x04;
        }
        if self.flags.disable_vlc_transmission {
            flags |= 0x08;
        }
        if self.flags.enable_targeted_mode {
            flags |= 0x10;
        }

        data.push(flags);
        data.push(self.diag_priority);
        data.extend_from_slice(&self.target_port_address_top.0.to_le_bytes());
        data.extend_from_slice(&self.target_port_address_bottom.0.to_le_bytes());
        data.extend_from_slice(&self.esta_man.0.to_le_bytes());
        data.extend_from_slice(&self.oem.0.to_le_bytes());
        
        return data;
    }

    /// Creates a new ArtPoll Packet with optional values. If values are not provided, default values are used.
    pub fn new(
        flags: ArtPollFlags,
        diag_priority: Option<u8>,
        target_port_address_top: Option<PortAddress>,
        target_port_address_bottom: Option<PortAddress>,
        esta_man: Option<EstaManCode>,
        oem: Option<OemCode>,
    ) -> ArtPoll {
        ArtPoll {
            protocol_version: 14,
            flags: flags,
            diag_priority: diag_priority.unwrap_or(0),
            target_port_address_top: target_port_address_top.unwrap_or(PortAddress(0)),
            target_port_address_bottom: target_port_address_bottom.unwrap_or(PortAddress(0)),
            esta_man: esta_man.unwrap_or(EstaManCode(0)),
            oem: oem.unwrap_or(OemCode(0)),
        }
    }
}