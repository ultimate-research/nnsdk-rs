#[allow(unused_imports)]
use self::super::root;
pub mod offlinewebsession;

#[repr(C)]
pub struct ShowOfflineHtmlPageArg {
    data: [u8; 0x2000],
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OfflineBootDisplayKind {
    Default,
    White,
    Black,
    Screenshot,
    BlurredScreenshot,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OfflineBackgroundKind {
    Default,
    Screenshot,
    BlurredScreenshot,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WebSessionBootMode {
    Default,
    InitiallyHidden,
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web19ShowOfflineHtmlPageEPNS0_26OfflineHtmlPageReturnValueERKNS0_22ShowOfflineHtmlPageArgE"]
    pub fn ShowOfflineHtmlPage(
        return_value: *mut OfflineHtmlPageReturnValue,
        arg: *const ShowOfflineHtmlPageArg,
    ) -> u32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArgC2EPKc"]
    pub fn ShowOfflineHtmlPageArg(
        this: *mut ShowOfflineHtmlPageArg,
        page_path: *const u8,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg21SetJsExtensionEnabledEb"]
    pub fn SetOfflineJsExtensionEnabled(
        this: *mut ShowOfflineHtmlPageArg,
        enabled: bool,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg16SetFooterEnabledEb"]
    pub fn SetOfflineFooterEnabled(this: *mut ShowOfflineHtmlPageArg, enabled: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg17SetPointerEnabledEb"]
    pub fn SetOfflinePointerEnabled(this: *mut ShowOfflineHtmlPageArg, enabled: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg25SetBootLoadingIconEnabledEb"]
    pub fn SetOfflineBootLoadingIconEnabled(
        this: *mut ShowOfflineHtmlPageArg,
        enabled: bool,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg18SetWebAudioEnabledEb"]
    pub fn SetOfflineWebAudioEnabled(this: *mut ShowOfflineHtmlPageArg, enabled: bool);
}
extern "C" {
    /// On a scale of 0.0 to 4.0
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg22OverrideWebAudioVolumeEf"]
    pub fn OverrideOfflineWebAudioVolume(
        this: *mut ShowOfflineHtmlPageArg,
        volume: f32,
    );
}
extern "C" {
    /// On a scale of 0.0 to 4.0
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg24OverrideMediaAudioVolumeEf"]
    pub fn OverrideOfflineMediaAudioVolume(
        this: *mut ShowOfflineHtmlPageArg,
        volume: f32,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg20SetBootAsMediaPlayerEb"]
    pub fn SetOfflineBootAsMediaPlayer(
        this: *mut ShowOfflineHtmlPageArg,
        enabled: bool,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg18SetPageFadeEnabledEb"]
    pub fn SetOfflinePageFadeEnabled(this: *mut ShowOfflineHtmlPageArg, enabled: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg19SetPageCacheEnabledEb"]
    pub fn SetOfflinePageCacheEnabled(this: *mut ShowOfflineHtmlPageArg, enabled: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg18SetBootDisplayKindENS0_22OfflineBootDisplayKindE"]
    pub fn SetOfflineBootDisplayKind(
        this: *mut ShowOfflineHtmlPageArg,
        boot_display_kind: OfflineBootDisplayKind,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web22ShowOfflineHtmlPageArg17SetBackgroundKindERKNS0_21OfflineBackgroundKindE"]
    pub fn SetOfflineBackgroundKind(
        this: *mut ShowOfflineHtmlPageArg,
        background_kind: *const OfflineBackgroundKind,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web11SetBootModeEPNS0_22ShowOfflineHtmlPageArgENS0_18WebSessionBootModeE"]
    pub fn SetBootMode(webpage_arg: &ShowOfflineHtmlPageArg, mode: WebSessionBootMode);
}
impl ShowOfflineHtmlPageArg {
    #[inline]
    pub fn new<T: AsRef<[u8]>>(page_path: T) -> Result<Self, core::str::Utf8Error> {
        let mut path_bytes = page_path.as_ref().to_vec();

        if path_bytes.len() > 3072 {
            path_bytes.truncate(3071);
        }

        path_bytes.append(&mut "\0".as_bytes().to_vec());

        unsafe {
            let mut instance = ShowOfflineHtmlPageArg { data: [0; 0x2000] };
            ShowOfflineHtmlPageArg(&mut instance, path_bytes.as_ptr());
            Ok(instance)
        }
    }

    pub fn set_background_kind(&mut self, kind: OfflineBackgroundKind) {
        unsafe { SetOfflineBackgroundKind(self, &kind) }
    }

    pub fn set_boot_display_kind(&mut self, kind: OfflineBootDisplayKind) {
        unsafe { SetOfflineBootDisplayKind(self, kind) }
    }

    pub fn display_footer(&mut self, enabled: bool) {
        unsafe { SetOfflineFooterEnabled(self, enabled) }
    }

    pub fn enable_javascript(&mut self, enabled: bool) {
        unsafe { SetOfflineJsExtensionEnabled(self, enabled) }
    }

    pub fn enable_pointer(&mut self, enabled: bool) {
        unsafe { SetOfflinePointerEnabled(self, enabled) }
    }

    pub fn enable_boot_loading_icon(&mut self, enabled: bool) {
        unsafe { SetOfflineBootLoadingIconEnabled(self, enabled) }
    }

    pub fn enable_web_audio(&mut self, enabled: bool) {
        unsafe { SetOfflineWebAudioEnabled(self, enabled) }
    }

    pub fn set_boot_mode(&mut self, mode: WebSessionBootMode) {
        unsafe { SetBootMode(self, mode) }
    }
}
#[repr(C)]
pub struct OfflineHtmlPageReturnValue {
    exit_reason: u64,
    last_url: [u8; 4096],
    last_url_size: u64,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OfflineExitReason {
    ExitPressed = 0,
    BackPressed = 1,
    Requested = 2,
    LastUrl = 3,
    ErrorDialog = 7,
    Unexpected = 20,
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3web26OfflineHtmlPageReturnValueC1Ev"]
    pub fn OfflineHtmlPageReturnValue(this: *mut OfflineHtmlPageReturnValue);
}
extern "C" {
    #[link_name = "\u{1}_ZNK2nn3web26OfflineHtmlPageReturnValue20GetOfflineExitReasonEv"]
    pub fn GetOfflineExitReason(
        this: *const OfflineHtmlPageReturnValue,
    ) -> OfflineExitReason;
}
extern "C" {
    #[link_name = "\u{1}_ZNK2nn3web26OfflineHtmlPageReturnValue10GetLastUrlEv"]
    pub fn GetLastUrl(this: *const OfflineHtmlPageReturnValue) -> *const u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNK2nn3web26OfflineHtmlPageReturnValue14GetLastUrlSizeEv"]
    pub fn GetLastUrlSize(this: *const OfflineHtmlPageReturnValue) -> usize;
}
impl OfflineHtmlPageReturnValue {
    pub fn new() -> Self {
        let mut instance = OfflineHtmlPageReturnValue {
            exit_reason: 0,
            last_url: [0; 4096],
            last_url_size: 0,
        };

        unsafe {
            OfflineHtmlPageReturnValue(&mut instance);
        }

        instance
    }

    pub fn get_exit_reason(&self) -> OfflineExitReason {
        unsafe { GetOfflineExitReason(self) }
    }
}
