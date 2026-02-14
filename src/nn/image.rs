#[allow(unused_imports)]
use self::super::root;
pub const JpegStatus_OK: JpegStatus = 0;
pub const JpegStatus_INVALID_FORMAT: JpegStatus = -32;
pub const JpegStatus_UNSUPPORTED_FORMAT: JpegStatus = -33;
pub const JpegStatus_OUT_OF_MEMORY: JpegStatus = -64;
pub type JpegStatus = i32;
pub const PixelFormat_RGBA32: PixelFormat = 0;
pub const PixelFormat_RGB24: PixelFormat = 1;
pub type PixelFormat = u32;
pub const ProcessStage_UNREGISTERED: ProcessStage = 0;
pub const ProcessStage_REGISTERED: ProcessStage = 1;
pub const ProcessStage_ANALYZED: ProcessStage = 2;
pub type ProcessStage = u32;
#[repr(C)]
pub struct Dimension {
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
pub struct JpegDecoder__bindgen_vtable(libc::c_void);
#[repr(C)]
pub struct JpegDecoder {
    pub vtable_: *const JpegDecoder__bindgen_vtable,
    pub mProcessStage: ProcessStage,
    pub mData: *mut u8,
    pub mSize: root::s64,
    pub _18: root::s32,
    pub mFormat: PixelFormat,
    pub mImgDimensions: Dimension,
    pub _28: root::s64,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn5image11JpegDecoder12SetImageDataEPKvm"]
    pub fn JpegDecoder_SetImageData(
        this: *mut JpegDecoder,
        source: *const u8,
        size: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5image11JpegDecoder7AnalyzeEv"]
    pub fn JpegDecoder_Analyze(
        this: *mut JpegDecoder,
    ) -> JpegStatus;
}
extern "C" {
    #[link_name = "\u{1}_ZNK2nn5image11JpegDecoder20GetAnalyzedDimensionEv"]
    pub fn JpegDecoder_GetAnalyzedDimension(
        this: *const JpegDecoder,
    ) -> Dimension;
}
extern "C" {
    #[link_name = "\u{1}_ZNK2nn5image11JpegDecoder25GetAnalyzedWorkBufferSizeEv"]
    pub fn JpegDecoder_GetAnalyzedWorkBufferSize(
        this: *const JpegDecoder,
    ) -> root::s64;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5image11JpegDecoder6DecodeEPvliS2_l"]
    pub fn JpegDecoder_Decode(
        this: *mut JpegDecoder,
        out: *mut u8,
        arg1: root::s64,
        alignment: root::s32,
        arg2: *mut u8,
        arg3: root::s64,
    ) -> JpegStatus;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5image11JpegDecoderC1Ev"]
    pub fn JpegDecoder_JpegDecoder(this: *mut JpegDecoder);
}
impl JpegDecoder {
    #[inline]
    pub unsafe fn SetImageData(&mut self, source: *const u8, size: u64) {
        JpegDecoder_SetImageData(self, source, size)
    }
    #[inline]
    pub unsafe fn Analyze(&mut self) -> JpegStatus {
        JpegDecoder_Analyze(self)
    }
    #[inline]
    pub unsafe fn GetAnalyzedDimension(&self) -> Dimension {
        JpegDecoder_GetAnalyzedDimension(self)
    }
    #[inline]
    pub unsafe fn GetAnalyzedWorkBufferSize(&self) -> root::s64 {
        JpegDecoder_GetAnalyzedWorkBufferSize(self)
    }
    #[inline]
    pub unsafe fn Decode(
        &mut self,
        out: *mut u8,
        arg1: root::s64,
        alignment: root::s32,
        arg2: *mut u8,
        arg3: root::s64,
    ) -> JpegStatus {
        JpegDecoder_Decode(self, out, arg1, alignment, arg2, arg3)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        JpegDecoder_JpegDecoder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5image11JpegDecoderD1Ev"]
    pub fn JpegDecoder_JpegDecoder_destructor(this: *mut JpegDecoder);
}
