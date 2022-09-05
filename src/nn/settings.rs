#[allow(unused_imports)]
use self::super::root;

pub mod system;

pub const Language_Language_Japanese: root::nn::settings::Language = 0;
pub const Language_Language_English: root::nn::settings::Language = 1;
pub const Language_Language_French: root::nn::settings::Language = 2;
pub const Language_Language_German: root::nn::settings::Language = 3;
pub const Language_Language_Italian: root::nn::settings::Language = 4;
pub const Language_Language_Spanish: root::nn::settings::Language = 5;
pub const Language_Language_Chinese: root::nn::settings::Language = 6;
pub const Language_Language_Korean: root::nn::settings::Language = 7;
pub const Language_Language_Dutch: root::nn::settings::Language = 8;
pub const Language_Language_Portuguese: root::nn::settings::Language = 9;
pub const Language_Language_Russian: root::nn::settings::Language = 10;
pub const Language_Language_Taiwanese: root::nn::settings::Language = 11;
pub const Language_Language_BritishEnglish: root::nn::settings::Language = 12;
pub const Language_Language_CanadianFrench: root::nn::settings::Language = 13;
pub const Language_Language_LatinAmericanSpanish: root::nn::settings::Language = 14;
pub type Language = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LanguageCode {
    pub code: [libc::c_char; 8usize],
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn8settings12LanguageCode4MakeENS0_8LanguageE"]
    pub fn LanguageCode_Make(
        arg1: root::nn::settings::Language,
    ) -> root::nn::settings::LanguageCode;
}
impl LanguageCode {
    #[inline]
    pub unsafe fn Make(
        arg1: root::nn::settings::Language,
    ) -> root::nn::settings::LanguageCode {
        LanguageCode_Make(arg1)
    }
}