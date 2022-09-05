#[allow(unused_imports)]
use self::super::root;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioRendererConfig {
    pub _0: *mut u64,
    pub _8: *mut u64,
    pub _10: *mut u64,
    pub _18: *mut u64,
    pub _20: *mut u64,
    pub _28: *mut u64,
    pub _30: *mut u64,
    pub _38: *mut u64,
    pub _40: *mut u64,
    pub _48: *mut u64,
    pub _50: *mut u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DelayType {
    pub _0: *mut u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FinalMixType {
    pub _0: *mut u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SubMixType {
    pub _0: *mut u64,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn5audio19SetDelayInputOutputEPNS0_9DelayTypeEPKaS4_i"]
    pub fn SetDelayInputOutput(
        arg1: *mut root::nn::audio::DelayType,
        arg2: *const root::s8,
        arg3: *const root::s8,
        arg4: root::s32,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5audio11RemoveDelayEPNS0_19AudioRendererConfigEPNS0_9DelayTypeEPNS0_12FinalMixTypeE"]
    pub fn RemoveDelay(
        arg1: *mut root::nn::audio::AudioRendererConfig,
        arg2: *mut root::nn::audio::DelayType,
        arg3: *mut root::nn::audio::FinalMixType,
    ) -> *mut libc::c_void;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5audio11RemoveDelayEPNS0_19AudioRendererConfigEPNS0_9DelayTypeEPNS0_10SubMixTypeE"]
    pub fn RemoveDelay1(
        arg1: *mut root::nn::audio::AudioRendererConfig,
        arg2: *mut root::nn::audio::DelayType,
        arg3: *mut root::nn::audio::SubMixType,
    ) -> *mut libc::c_void;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5audio17IsDelayRemoveableEPNS0_9DelayTypeE"]
    pub fn IsDelayRemoveable(arg1: *mut root::nn::audio::DelayType) -> bool;
}