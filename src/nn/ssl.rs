#[allow(unused_imports)]
use self::super::root;
pub const CertificateFormat_PEM: CertificateFormat = 1;
pub const CertificateFormat_DER: CertificateFormat = 2;
pub type CertificateFormat = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Context {
    pub _address: u8,
}
pub const Context_SslVersion_Auto: Context_SslVersion = 1;
pub const Context_SslVersion_v10: Context_SslVersion = 8;
pub const Context_SslVersion_v11: Context_SslVersion = 16;
pub const Context_SslVersion_v12: Context_SslVersion = 32;
pub type Context_SslVersion = u32;

extern "C" {
    #[link_name = "\u{1}_ZN2nn3ssl7Context6CreateENS1_10SslVersionE"]
    pub fn Context_Create(
        this: *mut Context,
        version: Context_SslVersion,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3ssl7Context15ImportServerPkiEPmPKcjNS0_17CertificateFormatE"]
    pub fn Context_ImportServerPki(
        this: *mut Context,
        arg1: *mut u64,
        certData: *const u8,
        certSize: u32,
        certFormat: CertificateFormat,
    ) -> root::Result;
}
impl Context {
    #[inline]
    pub unsafe fn Create(
        &mut self,
        version: Context_SslVersion,
    ) -> root::Result {
        Context_Create(self, version)
    }
    #[inline]
    pub unsafe fn ImportServerPki(
        &mut self,
        arg1: *mut u64,
        certData: *const u8,
        certSize: u32,
        certFormat: CertificateFormat,
    ) -> root::Result {
        Context_ImportServerPki(self, arg1, certData, certSize, certFormat)
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3ssl10InitializeEv"]
    pub fn Initialize() -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3ssl8FinalizeEv"]
    pub fn Finalize() -> root::Result;
}