
use crate::{EstaManCode, OemCode, PortAddress};


struct ArtPoll {
    protocol_version: u16,
    flags: ArtPollFlags,
    diag_priority: u8,
    target_port_address_top: PortAddress,
    target_port_address_bottom: PortAddress,
    esta_man: EstaManCode,
    oem: OemCode
}

struct ArtPollFlags {
    send_on_change: bool,
    send_diagnostics: bool,
    unicast_diagnostics: bool,
    disable_vlc_transmission: bool,
    enable_targeted_mode: bool,
}