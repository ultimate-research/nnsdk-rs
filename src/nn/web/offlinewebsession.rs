use self::super::root;
use self::super::{OfflineHtmlPageReturnValue, ShowOfflineHtmlPageArg};
#[repr(C)]
#[derive(Debug)]
pub struct OfflineWebSession {
    pub inner_web_impl: *const u8,
}
impl OfflineWebSession {
    pub fn new() -> Self {
        let web_impl_size = unsafe { GetWorkBufferSize() };
        let inner_web_impl =
            alloc::vec![0u8; web_impl_size].leak().as_ptr() as *const u8;

        let session = Self { inner_web_impl };

        unsafe {
            Initialize(&session, inner_web_impl);
        }

        session
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web17OfflineWebSession17GetWorkBufferSizeEv"]
    pub fn GetWorkBufferSize() -> usize;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web17OfflineWebSession10InitializeEPvm"]
    pub fn Initialize(
        session: *const OfflineWebSession,
        offline_web_impl: *const u8,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web17OfflineWebSession5StartEPPNS_2os15SystemEventTypeERKNS0_22ShowOfflineHtmlPageArgE"]
    pub fn Start(
        session: &OfflineWebSession,
        system_evt: &&root::nn::os::SystemEventType,
        webpage_arg: &ShowOfflineHtmlPageArg,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web17OfflineWebSession6AppearEv"]
    pub fn Appear(session: &OfflineWebSession) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web17OfflineWebSession21TrySendContentMessageEPKcm"]
    pub fn TrySendContentMessage(
        session: &OfflineWebSession,
        buffer: *const u8,
        buf_len: usize,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web17OfflineWebSession24TryReceiveContentMessageEPmPcm"]
    pub fn TryReceiveContentMessage(
        session: &OfflineWebSession,
        out_size: *mut usize,
        buffer: *const u8,
        buf_len: usize,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web17OfflineWebSession11WaitForExitEPNS0_26OfflineHtmlPageReturnValueE"]
    pub fn WaitForExit(
        session: &OfflineWebSession,
        return_val: &OfflineHtmlPageReturnValue,
    ) -> root::Result;
}