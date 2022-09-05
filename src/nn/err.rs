#[allow(unused_imports)]
use self::super::root;
type ErrorCodeCategoryType = u32;

#[repr(C)]
pub struct ApplicationErrorArg {
    unk: u64,
    error_code: u32,
    language_code: *const root::nn::settings::LanguageCode,
    dialog_message: [u8; 2048usize],
    fullscreen_message: [u8; 2048usize],
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3err19ApplicationErrorArg29SetApplicationErrorCodeNumberEj"]
    pub fn ApplicationErrorArg_SetApplicationErrorCodeNumber(
        this: *mut ApplicationErrorArg,
        error_code: u32,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3err19ApplicationErrorArg16SetDialogMessageEPKc"]
    pub fn ApplicationErrorArg_SetDialogMessage(
        this: *mut ApplicationErrorArg,
        message: *const u8,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3err19ApplicationErrorArg20SetFullScreenMessageEPKc"]
    pub fn ApplicationErrorArg_SetFullScreenMessage(
        this: *mut ApplicationErrorArg,
        message: *const u8,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3err19ApplicationErrorArgC1Ev"]
    pub fn ApplicationErrorArg_ApplicationErrorArg(
        this: *mut ApplicationErrorArg,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3err19ApplicationErrorArgC2EjPKcS3_RKNS_8settings12LanguageCodeE"]
    pub fn ApplicationErrorArg_ApplicationErrorArg1(
        this: *mut ApplicationErrorArg,
        error_code: u32,
        dialog_message: *const u8,
        fullscreen_message: *const u8,
        languageCode: *const root::nn::settings::LanguageCode,
    );
}
impl ApplicationErrorArg {
    #[inline]
    pub unsafe fn SetApplicationErrorCodeNumber(&mut self, error_code: u32) {
        ApplicationErrorArg_SetApplicationErrorCodeNumber(self, error_code)
    }
    #[inline]
    pub unsafe fn SetDialogMessage(&mut self, message: *const u8) {
        ApplicationErrorArg_SetDialogMessage(self, message)
    }
    #[inline]
    pub unsafe fn SetFullScreenMessage(&mut self, message: *const u8) {
        ApplicationErrorArg_SetFullScreenMessage(self, message)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut temp = ::core::mem::MaybeUninit::uninit();
        ApplicationErrorArg_ApplicationErrorArg(temp.as_mut_ptr());
        temp.assume_init()
    }
    #[inline]
    pub unsafe fn new_with_messages(
        error_code: u32,
        dialog_message: *const u8,
        fullscreen_message: *const u8,
        languageCode: *const root::nn::settings::LanguageCode,
    ) -> Self {
        let mut temp = ::core::mem::MaybeUninit::uninit();
        ApplicationErrorArg_ApplicationErrorArg1(
            temp.as_mut_ptr(),
            error_code,
            dialog_message,
            fullscreen_message,
            languageCode,
        );
        temp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3err13MakeErrorCodeENS0_21ErrorCodeCategoryTypeEj"]
    pub fn MakeErrorCode(
        err_category_type: ErrorCodeCategoryType,
        errorCodeNumber: u32,
    ) -> u32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3err20ShowApplicationErrorERKNS0_19ApplicationErrorArgE"]
    pub fn ShowApplicationError(arg: *const root::nn::err::ApplicationErrorArg);
}