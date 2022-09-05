#[allow(unused_imports)]
use self::super::root;

extern "C" {
    #[link_name = "\u{1}_ZN2nn4init6detail30DefaultAllocatorForThreadLocalEmm"]
    pub fn DefaultAllocatorForThreadLocal(
        arg1: u64,
        arg2: u64,
    ) -> *mut u8;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4init6detail32DefaultDeallocatorForThreadLocalEPvm"]
    pub fn DefaultDeallocatorForThreadLocal(
        arg1: *mut u8,
        arg2: u64,
    ) -> *mut u8;
}