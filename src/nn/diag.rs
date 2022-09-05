#[allow(unused_imports)]
use self::super::root;
pub mod detail;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LogMetaData {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct ModuleInfo {
    pub mPath: *mut u8,
    pub mBaseAddr: u64,
    pub mSize: u64,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn4diag12GetBacktraceEPmi"]
    pub fn GetBacktrace(out_array: *mut *const u8, array_len: i32) -> usize;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4diag12GetBacktraceEPmimmm"]
    pub fn GetBacktrace1(
        out_array: *mut *const u8,
        array_len: i32,
        fp: *const u64,
        sp: *const u64,
        pc: *const u64,
    ) -> usize;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4diag13GetSymbolNameEPcmm"]
    pub fn GetSymbolName(name: *mut u8, nameSize: u64, addr: u64)
        -> *mut u32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4diag40GetRequiredBufferSizeForGetAllModuleInfoEv"]
    pub fn GetRequiredBufferSizeForGetAllModuleInfo() -> u64;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4diag16GetAllModuleInfoEPPNS0_10ModuleInfoEPvm"]
    pub fn GetAllModuleInfo(
        out: *mut *mut root::nn::diag::ModuleInfo,
        buffer: *mut libc::c_void,
        bufferSize: u64,
    ) -> root::s32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4diag13GetSymbolSizeEm"]
    pub fn GetSymbolSize(addr: u64) -> u64;
}