#[allow(unused_imports)]
use self::super::root;

extern "C" {
    #[link_name = "\u{1}_ZN2nn4nifm10InitializeEv"]
    pub fn Initialize() -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4nifm19SetLocalNetworkModeEb"]
    pub fn SetLocalNetworkMode(arg1: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4nifm27SubmitNetworkRequestAndWaitEv"]
    pub fn SubmitNetworkRequestAndWait();
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4nifm18IsNetworkAvailableEv"]
    pub fn IsNetworkAvailable() -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4nifm26HandleNetworkRequestResultEv"]
    pub fn HandleNetworkRequestResult() -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4nifm20SubmitNetworkRequestEv"]
    pub fn SubmitNetworkRequest();
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4nifm22IsNetworkRequestOnHoldEv"]
    pub fn IsNetworkRequestOnHold() -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4nifm26GetCurrentPrimaryIpAddressEPm"]
    pub fn GetCurrentPrimaryIpAddress(inAddr: *mut u64) -> root::Result;
}