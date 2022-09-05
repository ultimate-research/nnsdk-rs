#[allow(unused_imports)]
use self::super::root;
pub mod detail;

extern "C" {
    #[link_name = "\u{1}_ZN2nn4init19InitializeAllocatorEPvm"]
    pub fn InitializeAllocator(addr: *mut libc::c_void, size: u64);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4init12GetAllocatorEv"]
    pub fn GetAllocator() -> *mut root::nn::mem::StandardAllocator;
}