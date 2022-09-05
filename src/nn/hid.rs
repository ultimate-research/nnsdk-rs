#[allow(unused_imports)]
use self::super::root;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct NpadHandheldState {
    pub updateCount: i64,
    pub Buttons: u64,
    pub LStickX: i32,
    pub LStickY: i32,
    pub RStickX: i32,
    pub RStickY: i32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct NpadGcState {
    pub updateCount: i64,
    pub Buttons: u64,
    pub LStickX: i32,
    pub LStickY: i32,
    pub RStickX: i32,
    pub RStickY: i32,
    pub Flags: u32,
    pub LTrigger: u32,
    pub RTrigger: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NpadStyleTag {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NpadStyleSet {
    pub flags: u32,
}

// pub u32 NpadStyleFullKey = 0x1;
// pub u32 NpadStyleHandheld = 0x2;
// pub u32 NpadStyleJoyDual = 0x4;
// pub u32 NpadStyleJoyLeft = 0x8;
// pub u32 NpadStyleJoyRight = 0x10;

#[repr(C)]
pub struct ControllerSupportArg {
    pub mMinPlayerCount: u8,
    pub mMaxPlayerCount: u8,
    pub mTakeOverConnection: u8,
    pub mLeftJustify: bool,
    pub mPermitJoyconDual: bool,
    pub mSingleMode: bool,
    pub mUseColors: bool,
    pub mColors: [root::nn::util::Color4u8; 4usize],
    pub mUsingControllerNames: u8,
    pub mControllerNames: [[libc::c_char; 129usize]; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ControllerSupportResultInfo {
    pub mPlayerCount: libc::c_int,
    pub mSelectedId: libc::c_int,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn3hid14InitializeNpadEv"]
    pub fn InitializeNpad();
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3hid22SetSupportedNpadIdTypeEPKjm"]
    pub fn SetSupportedNpadIdType(arg1: *const u32, arg2: u64);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3hid24SetSupportedNpadStyleSetENS_4util10BitFlagSetILi32ENS0_12NpadStyleTagEEE"]
    pub fn SetSupportedNpadStyleSet(arg1: u8);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3hid15GetNpadStyleSetERKj"]
    pub fn GetNpadStyleSet(arg1: *const u32) -> NpadStyleSet;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3hid19GetPlayerLedPatternERKj"]
    pub fn GetPlayerLedPattern(arg1: *const u32);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3hid13GetNpadStatesEPNS0_17NpadHandheldStateEiRKj"]
    pub fn GetNpadStates(
        arg1: *mut root::nn::hid::NpadHandheldState,
        arg2: root::s32,
        arg3: *const u32,
    );
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_17NpadHandheldStateERKj"]
    pub fn GetNpadHandheldState(
        arg1: *mut root::nn::hid::NpadHandheldState,
        arg2: *const u32,
    );

    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_16NpadFullKeyStateERKj"]
    pub fn GetNpadFullKeyState(
        arg1: *mut root::nn::hid::NpadHandheldState,
        arg2: *const u32,
    );

    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_11NpadGcStateERKj"]
    pub fn GetNpadGcState(
        arg1: *mut root::nn::hid::NpadGcState,
        arg2: *const u32,
    );

    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_16NpadJoyDualStateERKj"]
    pub fn GetNpadJoyDualState(
        arg1: *mut root::nn::hid::NpadHandheldState,
        arg2: *const u32,
    );

    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_16NpadJoyLeftStateERKj"]
    pub fn GetNpadJoyLeftState(
        arg1: *mut root::nn::hid::NpadHandheldState,
        arg2: *const u32,
    );

    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_17NpadJoyRightStateERKj"]
    pub fn GetNpadJoyRightState(
        arg1: *mut root::nn::hid::NpadHandheldState,
        arg2: *const u32,
    );
}