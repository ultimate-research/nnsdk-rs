#[allow(unused_imports)]
use self::super::root;
#[repr(C)]
pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
    pub padding1: u8,
    pub revision_major: u8,
    pub revision_minor: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub platform: [u8; 32usize],
    pub version_hash: [u8; 64usize],
    pub display_version: [u8; 24usize],
    pub display_title: [u8; 128usize],
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn8settings6system18GetFirmwareVersionEPNS1_15FirmwareVersionE"]
    pub fn GetFirmwareVersion(
        arg1: *mut root::nn::settings::system::FirmwareVersion,
    ) -> root::Result;
}