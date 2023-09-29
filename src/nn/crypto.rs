use alloc::{vec, vec::Vec};

#[allow(unused_imports)]
use self::super::root;
pub mod detail;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sha256Context {
    _unused: [u8; 0],
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto18GenerateSha256HashEPvmPKvm"]
    pub fn GenerateSha256Hash(
        out_hash: *mut u8,
        out_len: root::ulong,
        data: *const u8,
        data_len: root::ulong,
    );
}

pub fn generate_sha256_hash(data: &mut [u8]) -> Vec<u8> {
    let out_len = 32;
    let mut out_hash = vec![0; out_len];
    unsafe {
        GenerateSha256Hash(out_hash.as_mut_ptr(), out_len as u64, data.as_mut_ptr(), data.len() as u64);
    }
    return out_hash;
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto16DecryptAes128CbcEPvmPKvmS3_mS3_m"]
    pub fn DecryptAes128Cbc(
        out_data: *mut u8,
        out_len: u64,
        key: *const u8,
        key_len: u64,
        iv: *const u8,
        iv_len: u64,
        encrypted_data: *const u8,
        encrypted_data_len: u64,
    );
}

pub fn decrypt_aes_128_cbc(key: Vec<u8>, iv: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let mut decrypted_data = vec![0; data.len()];
    unsafe {
        DecryptAes128Cbc(decrypted_data.as_mut_ptr(), decrypted_data.len() as u64, key.as_ptr(), key.len() as u64, iv.as_ptr(), iv.len() as u64, data.as_ptr(), data.len() as u64)
    }
    return decrypted_data;
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto16EncryptAes128CbcEPvmPKvmS3_mS3_m"]
    pub fn EncryptAes128Cbc(
        out_data: *mut u8,
        out_len: u64,
        key: *const u8,
        key_len: u64,
        iv: *const u8,
        iv_len: u64,
        decrypted_data: *const u8,
        decrypted_data_len: u64,
    );
}

pub fn encrypt_aes_128_cbc(key: Vec<u8>, iv: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let mut encrypted_data = vec![0; data.len()];
    unsafe {
        EncryptAes128Cbc(encrypted_data.as_mut_ptr(), encrypted_data.len() as u64, key.as_ptr(), key.len() as u64, iv.as_ptr(), iv.len() as u64, data.as_ptr(), data.len() as u64)
    }
    return encrypted_data;
}


extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto16DecryptAes128CcmEPvmS1_mPKvmS3_mS3_mS3_mm"]
    pub fn DecryptAes128Ccm(
        arg1: *mut u8,
        arg2: u64,
        arg3: *mut u8,
        arg4: u64,
        arg5: *const u8,
        arg6: u64,
        arg7: *const u8,
        arg8: u64,
        arg9: *const u8,
        arg10: u64,
        arg11: *const u8,
        arg12: u64,
        arg13: u64,
    );
}
