#[allow(unused_imports)]
use self::super::root;
#[repr(C)]
pub struct Md5Impl {
    pub _x0: u32,
    pub _x4: u32,
    pub _x8: u32,
    pub _xC: u32,
    pub _x10: [u8; 64usize],
    pub _x50: u64,
    pub _x58: u32,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail7Md5Impl10InitializeEv"]
    pub fn Md5Impl_Initialize(this: *mut Md5Impl);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail7Md5Impl6UpdateEPKvm"]
    pub fn Md5Impl_Update(
        this: *mut Md5Impl,
        arg1: *const u8,
        dataSize: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail7Md5Impl12ProcessBlockEv"]
    pub fn Md5Impl_ProcessBlock(this: *mut Md5Impl);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail7Md5Impl7GetHashEPvm"]
    pub fn Md5Impl_GetHash(
        this: *mut Md5Impl,
        arg1: *mut u8,
        hashSize: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail7Md5Impl16ProcessLastBlockEv"]
    pub fn Md5Impl_ProcessLastBlock(this: *mut Md5Impl);
}
impl Md5Impl {
    #[inline]
    pub unsafe fn Initialize(&mut self) {
        Md5Impl_Initialize(self)
    }
    #[inline]
    pub unsafe fn Update(&mut self, arg1: *const u8, dataSize: u64) {
        Md5Impl_Update(self, arg1, dataSize)
    }
    #[inline]
    pub unsafe fn ProcessBlock(&mut self) {
        Md5Impl_ProcessBlock(self)
    }
    #[inline]
    pub unsafe fn GetHash(&mut self, arg1: *mut u8, hashSize: u64) {
        Md5Impl_GetHash(self, arg1, hashSize)
    }
    #[inline]
    pub unsafe fn ProcessLastBlock(&mut self) {
        Md5Impl_ProcessLastBlock(self)
    }
}
#[repr(C)]
#[repr(align(16))]
pub struct Sha1Impl {
    pub _x0: u64,
    pub _x8: u64,
    pub _x10: u32,
    pub __bindgen_padding_0: u64,
    pub _x14: u128,
    pub _x24: u128,
    pub _x34: u128,
    pub _x44: u32,
    pub _x48: u64,
    pub _x50: u64,
    pub _x58: u64,
    pub _x60: u64,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail8Sha1Impl10InitializeEv"]
    #[allow(improper_ctypes)]
    pub fn Sha1Impl_Initialize(this: *mut Sha1Impl);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail8Sha1Impl6UpdateEPKvm"]
    #[allow(improper_ctypes)]
    pub fn Sha1Impl_Update(
        this: *mut Sha1Impl,
        arg1: *const u8,
        arg2: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail8Sha1Impl12ProcessBlockEPKv"]
    #[allow(improper_ctypes)]
    pub fn Sha1Impl_ProcessBlock(
        this: *mut Sha1Impl,
        arg1: *const u8,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail8Sha1Impl7GetHashEPvm"]
    #[allow(improper_ctypes)]
    pub fn Sha1Impl_GetHash(
        this: *mut Sha1Impl,
        destHash: *mut u8,
        arg1: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail8Sha1Impl16ProcessLastBlockEv"]
    #[allow(improper_ctypes)]
    pub fn Sha1Impl_ProcessLastBlock(this: *mut Sha1Impl);
}
impl Sha1Impl {
    #[inline]
    pub unsafe fn Initialize(&mut self) {
        Sha1Impl_Initialize(self)
    }
    #[inline]
    pub unsafe fn Update(&mut self, arg1: *const u8, arg2: u64) {
        Sha1Impl_Update(self, arg1, arg2)
    }
    #[inline]
    pub unsafe fn ProcessBlock(&mut self, arg1: *const u8) {
        Sha1Impl_ProcessBlock(self, arg1)
    }
    #[inline]
    pub unsafe fn GetHash(&mut self, destHash: *mut u8, arg1: u64) {
        Sha1Impl_GetHash(self, destHash, arg1)
    }
    #[inline]
    pub unsafe fn ProcessLastBlock(&mut self) {
        Sha1Impl_ProcessLastBlock(self)
    }
}
#[repr(C)]
#[repr(align(16))]
pub struct Sha256Impl {
    pub _x0: u64,
    pub _x8: u64,
    pub _x10: u32,
    pub __bindgen_padding_0: u64,
    pub _x14: u128,
    pub _x24: u128,
    pub _x34: u128,
    pub _x44: u32,
    pub _x48: u64,
    pub _x50: u64,
    pub _x58: u64,
    pub _x60: u64,
    pub _x68: u64,
    pub _x70: u32,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail10Sha256Impl10InitializeEv"]
    #[allow(improper_ctypes)]
    pub fn Sha256Impl_Initialize(this: *mut Sha256Impl);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail10Sha256Impl6UpdateEPKvm"]
    #[allow(improper_ctypes)]
    pub fn Sha256Impl_Update(
        this: *mut Sha256Impl,
        arg1: *const u8,
        arg2: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail10Sha256Impl13ProcessBlocksEPKhm"]
    #[allow(improper_ctypes)]
    pub fn Sha256Impl_ProcessBlocks(
        this: *mut Sha256Impl,
        arg1: *const u8,
        arg2: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail10Sha256Impl7GetHashEPvm"]
    #[allow(improper_ctypes)]
    pub fn Sha256Impl_GetHash(
        this: *mut Sha256Impl,
        destHash: *mut u8,
        arg1: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail10Sha256Impl16ProcessLastBlockEv"]
    #[allow(improper_ctypes)]
    pub fn Sha256Impl_ProcessLastBlock(
        this: *mut Sha256Impl,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto6detail10Sha256Impl21InitializeWithContextEPKNS0_13Sha256ContextE"]
    #[allow(improper_ctypes)]
    pub fn Sha256Impl_InitializeWithContext(
        this: *mut Sha256Impl,
        arg1: *const root::nn::crypto::Sha256Context,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNK2nn6crypto6detail10Sha256Impl10GetContextEPNS0_13Sha256ContextE"]
    #[allow(improper_ctypes)]
    pub fn Sha256Impl_GetContext(
        this: *const Sha256Impl,
        arg1: *mut root::nn::crypto::Sha256Context,
    );
}
impl Sha256Impl {
    #[inline]
    pub unsafe fn Initialize(&mut self) {
        Sha256Impl_Initialize(self)
    }
    #[inline]
    pub unsafe fn Update(&mut self, arg1: *const u8, arg2: u64) {
        Sha256Impl_Update(self, arg1, arg2)
    }
    #[inline]
    pub unsafe fn ProcessBlocks(&mut self, arg1: *const u8, arg2: u64) {
        Sha256Impl_ProcessBlocks(self, arg1, arg2)
    }
    #[inline]
    pub unsafe fn GetHash(&mut self, destHash: *mut u8, arg1: u64) {
        Sha256Impl_GetHash(self, destHash, arg1)
    }
    #[inline]
    pub unsafe fn ProcessLastBlock(&mut self) {
        Sha256Impl_ProcessLastBlock(self)
    }
    #[inline]
    pub unsafe fn InitializeWithContext(
        &mut self,
        arg1: *const root::nn::crypto::Sha256Context,
    ) {
        Sha256Impl_InitializeWithContext(self, arg1)
    }
    #[inline]
    pub unsafe fn GetContext(&self, arg1: *mut root::nn::crypto::Sha256Context) {
        Sha256Impl_GetContext(self, arg1)
    }
}