#[allow(unused_imports)]
use self::super::root;
pub type ApplicationId = u64;

pub mod time;
pub mod err;
pub mod os;
pub mod settings;
pub mod oe;
pub mod account;
pub mod fs;
pub mod ro;
pub mod crypto;
pub mod prepo;
pub mod vi;
pub mod web;
pub mod image;
pub mod friends;
pub mod diag;
pub mod ssl;
pub mod mem;
pub mod init;
pub mod util;
pub mod hid;
pub mod audio;
pub mod svc;
pub mod nifm;
pub mod ldn;

#[repr(C)]
pub struct TimeSpan {
    pub nanoseconds: u64,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn11ReferSymbolEPKv"]
    pub fn ReferSymbol(arg1: *const libc::c_void);
}

