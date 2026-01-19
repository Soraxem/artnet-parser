use std::net::Ipv4Addr;

use crate::{EstaManCode, OemCode, PortAddress};


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
    ip_address: Ipv4Addr,
    /// the Nodes Artnet protocol version
    version_info: u16,
    oem: OemCode,
    /// This field contains the firmware version of the User Bios Extension Area (UBEA). If the UBEA is not programmed, this field contains zero.
    ubea_version: u8,
    inputs: [PortAddress; 4],
    outputs: [PortAddress; 4],
    status1: ArtPollReplyStatus1,
    esta_man: EstaManCode,
    port_name: [u8; 17],
    long_name: [u8; 63],
    node_report: [u8; 64],
    port_types: [ArtPollReplyPort; 4],
    good_output: [ArtPollReplyGoodOutput; 4],
    acn_priority: u8,
    sw_macro: ArtPollReplySwMacro,
    sw_remote: ArtPollReplySwRemote,
    style: StyleCode,
    mac_address: [u8; 6],
}

/// State of the Node
pub struct ArtPollReplyStatus1 {
    indicator_state: ArtPollIndicatorState,
    programming_authority: ArtPollProgrammingAuthority,
    booted_from_rom: bool,
    rdm_capable: bool,
    ubea_present: bool

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
#[derive(Clone)]
#[derive(Copy)]
pub struct ArtPollReplyPort {
    can_input_data: bool,
    can_output_data: bool,
    port_type: ArtPollReplyPortTypes,
}

/// Different port types on an artnet node
#[derive(Clone)]
#[derive(Copy)]
pub enum ArtPollReplyPortTypes {
    DMX512,
    MIDI,
    Avab,
    ColortranCMX,
    ADB62_5,
    ArtNet,
    DALI
}

#[derive(Clone)]
#[derive(Copy)]
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

#[derive(Clone)]
#[derive(Copy)]
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

#[derive(Clone)]
#[derive(Copy)]
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
        let parsed = ArtPollReply {
            ip_address: Ipv4Addr::new(data[10], data[11], data[12], data[13]),
            version_info: u16::from_le_bytes(data[16..17].try_into().or(Err("Malformed Packet"))?),
            oem: OemCode(0),
            ubea_version: data[6],
            inputs: [PortAddress(0); 4],
            outputs: [PortAddress(0); 4],
            status1: ArtPollReplyStatus1 {
                indicator_state: ArtPollIndicatorState::Unknown,
                programming_authority: ArtPollProgrammingAuthority::Unknown,
                booted_from_rom: false,
                rdm_capable: false,
                ubea_present: false,
            },
            esta_man: EstaManCode(0),
            port_name: [0; 17],
            long_name: [0; 63],
            node_report: [0; 64],
            port_types: [ArtPollReplyPort { can_input_data: false, can_output_data: false, port_type: ArtPollReplyPortTypes::DMX512 }; 4],
            good_output: [ArtPollReplyGoodOutput {
                dmx_is_ouput: false,
                includes_dmx_test_packets: false,
                includes_dmx_sips: false,
                includes_dmx_text_packets: false,
                is_merging_data: false,
                dmx_power_short: false,
                ltp_merge_mode: false,
                convert_from_sacn: false,
            }; 4],
            acn_priority: 0,
            sw_macro: ArtPollReplySwMacro {
                macro_1_active: false,
                macro_2_active: false,
                macro_3_active: false,
                macro_4_active: false,
                macro_5_active: false,
                macro_6_active: false,
                macro_7_active: false,
                macro_8_active: false,
            },
            sw_remote: ArtPollReplySwRemote {
                remote_1_active: false,
                remote_2_active: false,
                remote_3_active: false,
                remote_4_active: false,
                remote_5_active: false,
                remote_6_active: false,
                remote_7_active: false,
                remote_8_active: false,
            },
            style: StyleCode::StNode,
            mac_address: [0; 6],
        };

        Ok(parsed)
    }

    pub fn serialize(self) -> Vec<u8> {
        println!("ArtPollReply Serializing");
        todo!()
    }
}