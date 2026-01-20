use std::net::Ipv4Addr;

use crate::{EstaManCode, OemCode, PortAddress, OpCode};


/// Style of the Node
pub enum StyleCode {
    StNode,
    StController,
    StMedia,
    StRoute,
    StBackup,
    StConfig,
    StVisual,
}

/// Reply to a ArtPoll Packet
pub struct ArtPollReply {
    /// IP Address of the replying Node
    pub ip_address: Ipv4Addr,
    /// the Nodes Artnet protocol version
    pub version_info: u16,
    /// the Nodes Oem Code
    pub oem: OemCode,
    /// This field contains the firmware version of the User Bios Extension Area (UBEA). If the UBEA is not programmed, this field contains zero.
    pub ubea_version: u8,
    /// Address of the Input ports
    pub inputs: [PortAddress; 4],
    /// Address of the Output ports
    pub outputs: [PortAddress; 4],
    /// Status of the Node
    pub status1: ArtPollReplyStatus1,
    /// This field contains the firmware version of the Esta Man extension area.
    pub esta_man: EstaManCode,
    /// Short name of the Node
    pub port_name: [u8; 17],
    /// Long name of the Node
    pub long_name: [u8; 63],
    /// The Node Report is a 64 byte field that contains information about the Node.
    pub node_report: [u8; 64],
    /// The Port Types field contains information about the type of the Nodes ports.
    pub port_types: [ArtPollReplyPort; 4],
    /// The Good Output field contains information about the quality of the output of the Nodes ports.
    pub good_output: [ArtPollReplyGoodOutput; 4],
    /// The ACN Priority field contains the priority of the Nodes ACN output.
    pub acn_priority: u8,
    /// The SW Macro field contains information about the macro keys of the Node.
    pub sw_macro: ArtPollReplySwMacro,
    /// The SW Remote field contains information about the remote keys of the Node.
    pub sw_remote: ArtPollReplySwRemote,
    /// The Style field contains information about the style of the Node.
    pub style: StyleCode,
    /// The MAC Address field contains the MAC address of the Node.
    pub mac_address: [u8; 6],
}

impl Default for ArtPollReply {
    fn default() -> Self {
        Self {
            ip_address: Ipv4Addr::new(0, 0, 0, 0),
            version_info: 0,
            oem: OemCode(0),
            ubea_version: 0,
            inputs: [PortAddress(0); 4],
            outputs: [PortAddress(0); 4],
            status1: ArtPollReplyStatus1::default(),
            esta_man: EstaManCode(0),
            port_name: [0; 17],
            long_name: [0; 63],
            node_report: [0; 64],
            port_types: [ArtPollReplyPort::default(); 4],
            good_output: [ArtPollReplyGoodOutput::default(); 4],
            acn_priority: 0,
            sw_macro: ArtPollReplySwMacro::default(),
            sw_remote: ArtPollReplySwRemote::default(),
            style: StyleCode::StNode,
            mac_address: [0; 6],
        }
    }
}

/// State of the Node
pub struct ArtPollReplyStatus1 {
    indicator_state: ArtPollIndicatorState,
    programming_authority: ArtPollProgrammingAuthority,
    booted_from_rom: bool,
    rdm_capable: bool,
    ubea_present: bool
}

impl Default for ArtPollReplyStatus1 {
    fn default() -> Self {
        Self {
            indicator_state: ArtPollIndicatorState::Unknown,
            programming_authority: ArtPollProgrammingAuthority::Unknown,
            booted_from_rom: false,
            rdm_capable: false,
            ubea_present: false
        }
    }
}

/// State of the Indicator Light
pub enum ArtPollIndicatorState {
    Unknown,
    Locate,
    Mute,
    Normal,
}

/// Describes how the nodes artnet port addresses are programmable
pub enum ArtPollProgrammingAuthority {
    Unknown,
    Manual,
    Programmable,
}

/// Describes a single port of a node
#[derive(Clone, Copy)]
pub struct ArtPollReplyPort {
    can_input_data: bool,
    can_output_data: bool,
    port_type: ArtPollReplyPortTypes,
}

impl Default for ArtPollReplyPort {
    fn default() -> Self {
        Self {
            can_input_data: true,
            can_output_data: true,
            port_type: ArtPollReplyPortTypes::DMX512,
        }
    }
}

/// Different port types on an artnet node
#[derive(Clone, Copy)]
pub enum ArtPollReplyPortTypes {
    DMX512,
    MIDI,
    Avab,
    ColortranCMX,
    ADB62_5,
    ArtNet,
    DALI
}

#[derive(Clone, Copy, Default)]
pub struct ArtPollReplyGoodOutput {
    dmx_is_ouput: bool,
    includes_dmx_test_packets: bool,
    includes_dmx_sips: bool,
    includes_dmx_text_packets: bool,
    is_merging_data: bool,
    dmx_power_short: bool,
    ltp_merge_mode: bool,
    convert_from_sacn: bool,
}

#[derive(Clone, Copy, Default)]
pub struct ArtPollReplySwMacro {
    macro_1_active: bool,
    macro_2_active: bool,
    macro_3_active: bool,
    macro_4_active: bool,
    macro_5_active: bool,
    macro_6_active: bool,
    macro_7_active: bool,
    macro_8_active: bool,
}

#[derive(Clone, Copy, Default)]
pub struct ArtPollReplySwRemote {
    remote_1_active: bool,
    remote_2_active: bool,
    remote_3_active: bool,
    remote_4_active: bool,
    remote_5_active: bool,
    remote_6_active: bool,
    remote_7_active: bool,
    remote_8_active: bool,
}


impl ArtPollReply {
    pub fn parse(data: &[u8]) -> Result<ArtPollReply, &'static str> {

        // Iterating through the swin fields
        let mut inputs = [PortAddress::default(); 4];
        for i in 0..4 {
            // using unsafe because the values are assumed to be in range when recieved
            inputs[i] = PortAddress::unsafe_new(data[18], data[19], data[186 + i]);
        }

        // iterating through the swout fields
        let mut outputs = [PortAddress::default(); 4];
        for i in 0..4 {
            // using unsafe because the values are assumed to be in range when recieved
            outputs[i] = PortAddress::unsafe_new(data[18], data[19], data[190 + i]);
        }

        // Copy the binary texts
        let mut port_name = [0u8; 17];
        port_name.copy_from_slice(&data[26..43]);

        let mut long_name = [0u8; 63];
        long_name.copy_from_slice(&data[44..106]);

        let mut node_report = [0u8; 64];
        node_report.copy_from_slice(&data[108..171]);

        let parsed = ArtPollReply {
            ip_address: Ipv4Addr::new(data[10], data[11], data[12], data[13]),
            version_info: u16::from_be_bytes([data[16], data[17]]),
            oem: OemCode( u16::from_be_bytes([data[20], data[21]]) ),
            ubea_version: data[22],
            inputs: inputs,
            outputs: outputs,
            status1: ArtPollReplyStatus1 {
                indicator_state:
                    match data[23] & 0xC0 >> 6 {
                        1 => ArtPollIndicatorState::Locate,
                        2 => ArtPollIndicatorState::Mute,
                        3 => ArtPollIndicatorState::Normal,
                        _ => ArtPollIndicatorState::Unknown
                    },
                programming_authority: 
                    match data[23] & 0x30 >> 4 {
                        1 => ArtPollProgrammingAuthority::Manual,
                        2 => ArtPollProgrammingAuthority::Programmable,
                        _ => ArtPollProgrammingAuthority::Unknown
                    },
                booted_from_rom: data[23] & 0x04 == 0x04,
                rdm_capable: data[23] & 0x02 == 0x02,
                ubea_present: data[23] & 0x01 == 0x01
            },
            esta_man: EstaManCode( u16::from_be_bytes([data[24], data[25]]) ),
            port_name: port_name,
            long_name: long_name,
            node_report: node_report,

            // Implementation incomplete
            port_types: [ArtPollReplyPort::default(); 4],
            good_output: [ArtPollReplyGoodOutput::default(); 4],
            acn_priority: 0,
            sw_macro: ArtPollReplySwMacro::default(),
            sw_remote: ArtPollReplySwRemote::default(),
            style: StyleCode::StNode,
            mac_address: [0; 6],
        };

        Ok(parsed)
    }

    /// Serializes an ArtPollReply Packet to raw data for sending
    pub fn serialize(self) -> [u8; 207] {
        let mut data = [0_u8; 207];

        // Copying the structs data to the correct positions in the byte array

        data[0..8].copy_from_slice(b"Art-Net\0");
        data[8..10].copy_from_slice(&(OpCode::OpPollReply as u16).to_le_bytes());
        data[10..14].copy_from_slice(&self.ip_address.octets());
        data[14..16].copy_from_slice(&6454_u16.to_be_bytes());
        data[16..18].copy_from_slice(&self.version_info.to_be_bytes());

        // Using net and Subnet of the first input -> not safe, a problem for another day ;-)
        data[18] = self.inputs[0].net();
        data[19] = self.inputs[0].subnet();

        data[20..22].copy_from_slice(&self.oem.0.to_be_bytes());

        data[22] = self.ubea_version;

        // Building the Status1 byte
        match self.status1.indicator_state {
            ArtPollIndicatorState::Locate => data[23] |= 0x40,
            ArtPollIndicatorState::Mute => data[23] |= 0x80,
            ArtPollIndicatorState::Normal => data[23] |= 0xC0,
            _ => {}
        }
        
        match self.status1.programming_authority {
            ArtPollProgrammingAuthority::Manual => data[23] |= 0x10,
            ArtPollProgrammingAuthority::Programmable => data[23] |= 0x20,
            _ => {}
        }

        if self.status1.booted_from_rom {
            data[23] |= 0x04;
        }
        if self.status1.rdm_capable {
            data[23] |= 0x02;
        }
        if self.status1.ubea_present {
            data[23] |= 0x01;
        }

        data[24..26].copy_from_slice(&self.esta_man.0.to_be_bytes());

        data[26..44].copy_from_slice(&self.port_name);
        data[44..108].copy_from_slice(&self.long_name);
        data[108..172].copy_from_slice(&self.node_report);

        // Iterate Swin and Swout
        for i in 0..4 {
            data[186 + i] = self.inputs[i].universe();
        }
        for i in 0..4 {
            data[190 + i] = self.outputs[i].universe();
        }

        println!("ArtPollReply Serializing");

        return data
    }
}