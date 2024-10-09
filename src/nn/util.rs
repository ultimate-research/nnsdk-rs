#[repr(simd)]
pub struct Vector3f {
    pub value: [f32;3]
}

#[repr(simd)]
pub struct Vector4f {
    pub value: [f32;4]
}

#[allow(unused_imports)]
use self::super::root;

#[repr(C)]
pub struct Unorm8x4 {
    pub elements: [u8; 4usize],
}

#[repr(C)]
pub struct Color4u8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub const CharacterEncodingResult_Success: CharacterEncodingResult = 0;
pub const CharacterEncodingResult_BadLength: CharacterEncodingResult =
    1;
pub const CharacterEncodingResult_InvalidFormat:
    CharacterEncodingResult = 2;
pub type CharacterEncodingResult = u32;
extern "C" {
    #[link_name = "\u{1}_ZN2nn4util30PickOutCharacterFromUtf8StringEPcPPKc"]
    pub fn PickOutCharacterFromUtf8String(
        arg1: *mut u8,
        str: *mut *const u8,
    ) -> CharacterEncodingResult;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4util27ConvertCharacterUtf8ToUtf32EPjPKc"]
    pub fn ConvertCharacterUtf8ToUtf32(
        dest: *mut u32,
        src: *const u8,
    ) -> CharacterEncodingResult;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4util30ConvertStringUtf16NativeToUtf8EPciPKti"]
    pub fn ConvertStringUtf16NativeToUtf8(
        arg1: *mut u8,
        arg2: root::s32,
        arg3: *const u16,
        arg4: root::s32,
    ) -> CharacterEncodingResult;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4util30ConvertStringUtf8ToUtf16NativeEPtiPKci"]
    pub fn ConvertStringUtf8ToUtf16Native(
        arg1: *mut u16,
        arg2: root::s32,
        arg3: *const u8,
        arg4: root::s32,
    ) -> CharacterEncodingResult;
}

#[repr(C)]
pub struct RelocationTable {
    pub mMagic: root::s32,
    pub mPosition: u32,
    pub mSectionCount: root::s32,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn4util15RelocationTable8RelocateEv"]
    pub fn RelocationTable_Relocate(this: *mut RelocationTable);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4util15RelocationTable10UnrelocateEv"]
    pub fn RelocationTable_Unrelocate(this: *mut RelocationTable);
}
impl RelocationTable {
    #[inline]
    pub unsafe fn Relocate(&mut self) {
        RelocationTable_Relocate(self)
    }
    #[inline]
    pub unsafe fn Unrelocate(&mut self) {
        RelocationTable_Unrelocate(self)
    }
}
#[repr(C)]
pub struct BinaryFileHeader {
    pub mMagic: root::s32,
    pub mSig: u32,
    pub mVerMicro: u8,
    pub mVerMinor: u8,
    pub mVerMajor: u16,
    pub mBOM: u16,
    pub mAlignment: u8,
    pub mTargetAddrSize: u8,
    pub mFileNameOffset: u32,
    pub mFlag: u16,
    pub mFirstBlockOffs: u16,
    pub mRelocationTableOffs: u32,
    pub mSize: u32,
}

extern "C" {
    #[link_name = "\u{1}_ZNK2nn4util16BinaryFileHeader7IsValidEliii"]
    pub fn BinaryFileHeader_IsValid(
        this: *const BinaryFileHeader,
        packedSig: root::s64,
        majorVer: root::s32,
        minorVer: root::s32,
        microVer: root::s32,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNK2nn4util16BinaryFileHeader11IsRelocatedEv"]
    pub fn BinaryFileHeader_IsRelocated(
        this: *const BinaryFileHeader,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNK2nn4util16BinaryFileHeader15IsEndianReverseEv"]
    pub fn BinaryFileHeader_IsEndianReverse(
        this: *const BinaryFileHeader,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4util16BinaryFileHeader18GetRelocationTableEv"]
    pub fn BinaryFileHeader_GetRelocationTable(
        this: *mut BinaryFileHeader,
    ) -> *mut RelocationTable;
}

impl BinaryFileHeader {
    #[inline]
    pub unsafe fn IsValid(
        &self,
        packedSig: root::s64,
        majorVer: root::s32,
        minorVer: root::s32,
        microVer: root::s32,
    ) -> bool {
        BinaryFileHeader_IsValid(self, packedSig, majorVer, minorVer, microVer)
    }
    #[inline]
    pub unsafe fn IsRelocated(&self) -> bool {
        BinaryFileHeader_IsRelocated(self)
    }
    #[inline]
    pub unsafe fn IsEndianReverse(&self) -> bool {
        BinaryFileHeader_IsEndianReverse(self)
    }
    #[inline]
    pub unsafe fn GetRelocationTable(
        &mut self,
    ) -> *mut RelocationTable {
        BinaryFileHeader_GetRelocationTable(self)
    }
}
