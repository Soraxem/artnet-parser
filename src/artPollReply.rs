use std::net::Ipv4Addr;

use crate::{EstaManCode, OemCode, PortAddress};



enum StyleCode {
    StNode,
    StController,
    StMedia,
    StRoute,
    StBackup,
    StConfig,
    StVisual,
}



struct ArtPollReply {
    ip_address: Ipv4Addr,
    version_info: u16,
    oem: OemCode,
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

struct ArtPollReplyStatus1 {
    indicator_state: ArtPollIndicatorState,
    programming_authority: ArtPollProgrammingAuthority,
    booted_from_rom: bool,
    rdm_capable: bool,
    ubea_present: bool

}

enum ArtPollIndicatorState {
    Unknown,
    Locate,
    Mute,
    Normal,
}

enum ArtPollProgrammingAuthority {
    Unknown,
    Manual,
    Programmable,
}

struct ArtPollReplyPort {
    can_input_data: bool,
    can_output_data: bool,
    port_type: ArtPollReplyPortTypes,
}

enum ArtPollReplyPortTypes {
    DMX512,
    MIDI,
    Avab,
    ColortranCMX,
    ADB62_5,
    ArtNet,
    DALI
}

struct ArtPollReplyGoodOutput {
    dmx_is_ouput: bool,
    includes_dmx_test_packets: bool,
    includes_dmx_sips: bool,
    includes_dmx_text_packets: bool,
    is_merging_data: bool,
    dmx_power_short: bool,
    ltp_merge_mode: bool,
    convert_from_sacn: bool,
}

struct ArtPollReplySwMacro {
    macro_1_active: bool,
    macro_2_active: bool,
    macro_3_active: bool,
    macro_4_active: bool,
    macro_5_active: bool,
    macro_6_active: bool,
    macro_7_active: bool,
    macro_8_active: bool,
}

struct ArtPollReplySwRemote {
    remote_1_active: bool,
    remote_2_active: bool,
    remote_3_active: bool,
    remote_4_active: bool,
    remote_5_active: bool,
    remote_6_active: bool,
    remote_7_active: bool,
    remote_8_active: bool,
}