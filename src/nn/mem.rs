#[allow(unused_imports)]
use self::super::root;
#[repr(C)]
pub struct StandardAllocator {
    pub mIsInitialized: bool,
    pub mIsEnabledThreadCache: bool,
    pub _2: u16,
    pub mAllocAddr: *mut u64,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator10InitializeEPvm"]
    pub fn StandardAllocator_Initialize(
        this: *mut StandardAllocator,
        address: *mut libc::c_void,
        size: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator8FinalizeEv"]
    pub fn StandardAllocator_Finalize(this: *mut StandardAllocator);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator10ReallocateEPvm"]
    pub fn StandardAllocator_Reallocate(
        this: *mut StandardAllocator,
        address: *mut libc::c_void,
        newSize: u64,
    ) -> *mut libc::c_void;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator8AllocateEm"]
    pub fn StandardAllocator_Allocate(
        this: *mut StandardAllocator,
        size: u64,
    ) -> *mut libc::c_void;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator4FreeEPv"]
    pub fn StandardAllocator_Free(
        this: *mut StandardAllocator,
        address: *mut libc::c_void,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator4DumpEv"]
    pub fn StandardAllocator_Dump(this: *mut StandardAllocator);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3mem17StandardAllocatorC1Ev"]
    pub fn StandardAllocator_StandardAllocator(
        this: *mut StandardAllocator,
    );
}
impl StandardAllocator {
    #[inline]
    pub unsafe fn Initialize(&mut self, address: *mut libc::c_void, size: u64) {
        StandardAllocator_Initialize(self, address, size)
    }
    #[inline]
    pub unsafe fn Finalize(&mut self) {
        StandardAllocator_Finalize(self)
    }
    #[inline]
    pub unsafe fn Reallocate(
        &mut self,
        address: *mut libc::c_void,
        newSize: u64,
    ) -> *mut libc::c_void {
        StandardAllocator_Reallocate(self, address, newSize)
    }
    #[inline]
    pub unsafe fn Allocate(&mut self, size: u64) -> *mut libc::c_void {
        StandardAllocator_Allocate(self, size)
    }
    #[inline]
    pub unsafe fn Free(&mut self, address: *mut libc::c_void) {
        StandardAllocator_Free(self, address)
    }
    #[inline]
    pub unsafe fn Dump(&mut self) {
        StandardAllocator_Dump(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::core::mem::MaybeUninit::uninit();
        StandardAllocator_StandardAllocator(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}