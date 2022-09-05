#[allow(unused_imports)]
use self::super::root;
pub mod detail;

extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto18GenerateSha256HashEPvmPKvm"]
    pub fn GenerateSha256Hash(
        arg1: *mut libc::c_void,
        arg2: root::ulong,
        arg3: *const libc::c_void,
        arg4: root::ulong,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sha256Context {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto16DecryptAes128CbcEPvmPKvmS3_mS3_m"]
    pub fn DecryptAes128Cbc(
        arg1: *mut libc::c_void,
        arg2: u64,
        arg3: *const libc::c_void,
        arg4: u64,
        arg5: *const libc::c_void,
        arg6: u64,
        arg7: *const libc::c_void,
        arg8: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto16EncryptAes128CbcEPvmPKvmS3_mS3_m"]
    pub fn EncryptAes128Cbc(
        arg1: *mut libc::c_void,
        arg2: u64,
        arg3: *const libc::c_void,
        arg4: u64,
        arg5: *const libc::c_void,
        arg6: u64,
        arg7: *const libc::c_void,
        arg8: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto16DecryptAes128CcmEPvmS1_mPKvmS3_mS3_mS3_mm"]
    pub fn DecryptAes128Ccm(
        arg1: *mut libc::c_void,
        arg2: u64,
        arg3: *mut libc::c_void,
        arg4: u64,
        arg5: *const libc::c_void,
        arg6: u64,
        arg7: *const libc::c_void,
        arg8: u64,
        arg9: *const libc::c_void,
        arg10: u64,
        arg11: *const libc::c_void,
        arg12: u64,
        arg13: u64,
    );
}