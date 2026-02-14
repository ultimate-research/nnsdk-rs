use alloc::{vec::Vec, boxed::Box, string::String};
use libc::c_void;
use libc::free;

#[repr(C)]
pub struct KeyboardConfig {
    // inner: [u8; 0x4d0],
    unk1: [u8; 0x3ab],
    // 0x3ac
    max_text_length: u8, // Confirmed to be a u32
    // min_text_length follows immediately after, u32
    unk2: [u8; 0xd],
    // 0x3b9
    input_mode: u8, // Not sure, probably set by the presets already
    unk3: u32,
    // 0x3bd
    cancel_disabled: bool,
    unk4: [u8; 0x113],
}

impl KeyboardConfig {
    pub fn new() -> Self {
        Self {
            unk1: [0u8;0x3ab],
            max_text_length: 0,
            unk2: [0u8;0xd],
            input_mode: 0,
            unk3: 0,
            cancel_disabled: false,
            unk4: [0u8;0x113],
        }
    }

    /// Maximum is 500 characters, however we represent it as a u8 for now because more reverse engineering is needed to confirm the type.
    pub fn set_text_max_length(&mut self, max_length: u8) {
        self.max_text_length = if max_length == 0 {
            1
        } else {
            max_length
        }
    }

    pub fn enable_cancel_button(&mut self, enable: bool) {
        self.cancel_disabled = !enable;
    }
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
pub struct ShowKeyboardArg {
    pub keyboard_config: [u8; 0x4D0],
    pub work_buffer: *mut c_void,
    pub work_buffer_size: usize,
    pub text_buffer: *const c_void,
    pub text_buffer_size: usize,
    pub custom_dictionary_buffer: *const c_void,
    pub custom_dictionary_buffer_size: usize,
}

#[repr(transparent)]
pub struct SwkbdString(Box<[u16]>);

impl SwkbdString {
    fn that_big_size() -> Self {
        let x: Box<[u16; 1002]> = unsafe { Box::new_zeroed().assume_init() };
        SwkbdString(x)
    }
}

impl From<SwkbdString> for String {
    fn from(s: SwkbdString) -> String {
        let end = s.0.iter().position(|c| *c == 0).unwrap_or_else(|| s.0.len());
        String::from_utf16_lossy(&s.0[..end])
    }
}

impl ShowKeyboardArg {
    pub fn new() -> Box<Self> {
        let mut arg: Box<Self> = unsafe { Box::new_zeroed().assume_init() };

        unsafe {
            make_preset_default(&mut arg.keyboard_config);
        }
        // max length
        arg.keyboard_config[0x3ac] = 20;
        // mode
        arg.keyboard_config[0x3b8] = 0;
        // cancel
        arg.keyboard_config[0x3bc] = 0;



        let work_buffer_size = 0xd000;
        let work_buffer: Box<[u8; 0xd000]> = unsafe { Box::new_zeroed().assume_init() };
        let work_buffer = Box::leak(work_buffer) as *mut _ as *mut c_void;

        arg.work_buffer = work_buffer;
        arg.work_buffer_size = work_buffer_size;

        arg
    }

    pub fn header_text(&mut self, s: &str) -> &mut Self {
        let x: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
            set_header_text(&self.keyboard_config, x.as_ptr() as _);
        }

        std::mem::drop(x);

        self
    }

    pub fn show(&self) -> Option<String> {
        let string = SwkbdString::that_big_size();
        if unsafe { show_keyboard(&string, self) } == 0x29f {
            None
        } else {
            Some(string.into())
        }
    }
}



impl Drop for ShowKeyboardArg {
    fn drop(&mut self) {
        if !self.work_buffer.is_null() {
            unsafe {
                free(self.work_buffer);
            }
        }
    }
}

/// The buffer can represent both a UTF8 and UTF16 C string depending on a bool set in KeyboardConfig.
/// The is_utf8 bool is not yet defined in the structure as only the low-level API has been exposed so far.
/// For now, assume the buffer always represents a UTF16 and convert accordingly.
#[repr(C)]
pub struct ShowKeyboardString {
    pub string_buffer: *const u8,
    pub buffer_len: usize,
}

impl ShowKeyboardString {
    pub fn new() -> Self {
        Self {
            string_buffer: 0 as _,
            buffer_len: 0,
        }
    }
}

impl Default for ShowKeyboardString {
    fn default() -> Self {
        Self::new()
    }
}

extern "C" {
    
    #[link_name = "_ZN2nn5swkbd25GetRequiredWorkBufferSizeEb"]
    pub fn get_required_work_buffer_size(use_dictionary: bool) -> usize;

    // #[link_name = "_ZN2nn5swkbd17MakePresetDefaultEPNS0_14KeyboardConfigE"]
    // pub fn make_preset_default(keyboard_config: *mut KeyboardConfig);

    #[link_name = "_ZN2nn5swkbd17MakePresetDefaultEPNS0_14KeyboardConfigE"]
    pub fn make_preset_default(x: *mut [u8; 0x4d0]);
    
    #[link_name = "_ZN2nn5swkbd18MakePresetPasswordEPNS0_14KeyboardConfigE"]
    pub fn make_preset_password(keyboard_config: *mut KeyboardConfig);

    #[link_name = "_ZN2nn5swkbd18MakePresetUserNameEPNS0_14KeyboardConfigE"]
    pub fn make_preset_user_name(keyboard_config: *mut KeyboardConfig);

    #[allow(improper_ctypes)]
    #[link_name = "_ZN2nn5swkbd12ShowKeyboardEPNS0_6StringERKNS0_15ShowKeyboardArgE"]
    fn show_keyboard(string: *const SwkbdString, arg: *const ShowKeyboardArg) -> u32;

    //#[link_name = "_ZN2nn5swkbd17SetHeaderTextUtf8EPNS0_14KeyboardConfigEPKc"]
    #[link_name = "_ZN2nn5swkbd13SetHeaderTextEPNS0_14KeyboardConfigEPKDs"]
    pub fn set_header_text(keyboard_config: *const [u8; 0x4d0], text: *const u16);

    #[link_name = "_ZN2nn5swkbd17SetHeaderTextUtf8EPNS0_14KeyboardConfigEPKc"]
    pub fn set_header_text_utf8(keyboard_config: *mut KeyboardConfig, text: *const u8);
    
    #[link_name = "_ZN2nn5swkbd16SetGuideTextUtf8EPNS0_14KeyboardConfigEPKc"]
    pub fn set_guide_text_utf8(keyboard_config: *mut KeyboardConfig, text: *const u8);
    
    #[link_name = "_ZN2nn5swkbd18SetInitialTextUtf8EPNS0_15ShowKeyboardArgEPKc"]
    pub fn set_initial_text_utf8(keyboard_config: *mut ShowKeyboardArg, text: *const u8);
    
    #[link_name = "_ZN2nn5swkbd28SetLeftOptionalSymbolKeyUtf8EPNS0_14KeyboardConfigEPKc"]
    pub fn set_left_optional_symbol_key_utf8(keyboard_config: *mut KeyboardConfig, text: *const u8);
    
    #[link_name = "_ZN2nn5swkbd13SetOkTextUtf8EPNS0_14KeyboardConfigEPKc"]
    pub fn set_ok_text_utf8(keyboard_config: *mut KeyboardConfig, text: *const u8);
    
    #[link_name = "_ZN2nn5swkbd14SetSubTextUtf8EPNS0_14KeyboardConfigEPKc"]
    pub fn set_sub_text_utf8(keyboard_config: *mut KeyboardConfig, text: *const u8);
}
