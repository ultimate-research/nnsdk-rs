use alloc::string::{String, ToString};

#[allow(unused_imports)]
use self::super::root;
pub type FocusHandlingMode = i32;
pub type PerformanceMode = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DisplayVersion {
    pub name: [u8; 16usize],
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe10InitializeEv"]
    pub fn Initialize();
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe27SetPerformanceConfigurationENS0_15PerformanceModeEi"]
    pub fn SetPerformanceConfiguration(arg1: root::nn::oe::PerformanceMode, arg2: u32);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe27GetPerformanceConfigurationENS0_15PerformanceModeE"]
    pub fn GetPerformanceConfiguration(arg1: root::nn::oe::PerformanceMode) -> u32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe16GetOperationModeEv"]
    pub fn GetOperationMode() -> root::s32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe18GetPerformanceModeEv"]
    pub fn GetPerformanceMode() -> PerformanceMode;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe28SetResumeNotificationEnabledEb"]
    pub fn SetResumeNotificationEnabled(arg1: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe42SetOperationModeChangedNotificationEnabledEb"]
    pub fn SetOperationModeChangedNotificationEnabled(arg1: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe44SetPerformanceModeChangedNotificationEnabledEb"]
    pub fn SetPerformanceModeChangedNotificationEnabled(arg1: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe20SetFocusHandlingModeEi"]
    pub fn SetFocusHandlingMode(arg1: root::nn::oe::FocusHandlingMode);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe25TryPopNotificationMessageEPj"]
    pub fn TryPopNotificationMessage(arg1: *mut u32) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe20GetCurrentFocusStateEv"]
    pub fn GetCurrentFocusState() -> root::s32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe23EnableGamePlayRecordingEPvm"]
    pub fn EnableGamePlayRecording(arg1: *mut u8, arg2: u64);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe37IsUserInactivityDetectionTimeExtendedEv"]
    pub fn IsUserInactivityDetectionTimeExtended() -> bool;
}

pub fn is_user_inactivity_detection_time_extended() -> bool {
    let result = unsafe { IsUserInactivityDetectionTimeExtended() };

    return result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe38SetUserInactivityDetectionTimeExtendedEb"]
    pub fn SetUserInactivityDetectionTimeExtended(arg1: bool);
}

pub fn set_user_inactivity_detection_time_extended(do_extend: bool) {
    unsafe { SetUserInactivityDetectionTimeExtended(do_extend); }
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe17FinishStartupLogoEv"]
    pub fn FinishStartupLogo();
}

pub fn finish_startup_logo() {
    unsafe { FinishStartupLogo(); }
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe18ReportUserIsActiveEv"]
    pub fn ReportUserIsActive();
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe18GetDesiredLanguageEv"]
    pub fn GetDesiredLanguage() -> root::nn::settings::LanguageCode;
}

pub fn get_desired_language() -> String {
    let mut result = "";

    let lang_code = unsafe { GetDesiredLanguage() };

    result = core::str::from_utf8(&lang_code.code).unwrap();

    return result.to_string();
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe17GetDisplayVersionEPNS0_14DisplayVersionE"]
    pub fn GetDisplayVersion(arg1: *mut root::nn::oe::DisplayVersion);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe21IsCpuOverclockEnabledEv"]
    pub fn IsCpuOverclockEnabled() -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe22SetCpuOverclockEnabledEb"]
    pub fn SetCpuOverclockEnabled(enabled: bool);
}

pub fn get_display_version() -> String {
    let mut ver = DisplayVersion { name: [0; 16] };

    unsafe {
        GetDisplayVersion(&mut ver);
    }
    let string = core::str::from_utf8(&ver.name).unwrap();

    return string.to_string();
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum CpuBoostMode {
    Disabled = 0,
    Boost = 1,
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe15SetCpuBoostModeENS0_12CpuBoostModeE"]
    pub fn SetCpuBoostMode(mode: CpuBoostMode);
}

pub fn set_cpu_boost_mode(mode: CpuBoostMode) {
    unsafe {
        SetCpuBoostMode(mode);
    }
}

extern "C" {
    #[link_name = "_ZN2nn2oe14RestartProgramEPKvm"]
    pub fn RestartProgram(argv: *const u8, argc: u32) -> !;
}
pub fn restart_program_no_args() -> ! {
    unsafe { RestartProgram("".as_ptr() as _, 0) }
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe28RequestToRelaunchApplicationEv"]
    pub fn RequestToRelaunchApplication() -> !;
}

pub fn request_to_relaunch_application() -> ! {
    unsafe { RequestToRelaunchApplication(); }
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe15ExitApplicationEv"]
    pub fn ExitApplication() -> !;
}
pub fn exit_application() -> ! {
    unsafe { ExitApplication(); }
}
