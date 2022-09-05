#[allow(unused_imports)]
use self::super::root;
extern "C" {
    #[link_name = "\u{1}_ZN2nn4diag6detail7LogImplERKNS0_11LogMetaDataEPKcz"]
    pub fn LogImpl(
        arg1: *const root::nn::diag::LogMetaData,
        arg2: *const libc::c_char,
        ...
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4diag6detail9AbortImplEPKcS3_S3_i"]
    pub fn AbortImpl(
        arg1: *const libc::c_char,
        arg2: *const libc::c_char,
        arg3: *const libc::c_char,
        arg4: root::s32,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4diag6detail9AbortImplEPKcS3_S3_ij"]
    pub fn AbortImpl1(
        arg1: *const libc::c_char,
        arg2: *const libc::c_char,
        arg3: *const libc::c_char,
        arg4: libc::c_int,
        arg5: root::Result,
    );
}