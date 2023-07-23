#[allow(unused_imports)]
use self::super::root;

#[repr(C)]
pub struct MultiWaitObjectList {
    inner: [*mut (); 2],
}

#[repr(C)]
pub struct InternalCriticalSection {
    pub Image: u32,
}

#[repr(C)]
pub struct InternalConditionVariable {
    pub Image: u32,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn2os6detail22g_CommandLineParameterE"]
    pub static mut g_CommandLineParameter: root::s32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os6detail26g_CommandLineParameterArgvE"]
    pub static mut g_CommandLineParameterArgv: *mut *mut u8;
}