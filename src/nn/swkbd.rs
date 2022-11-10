use alloc::vec;
use libc::c_void;

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
    keyboard_config: KeyboardConfig,
    work_buffer: *mut c_void,
    work_buffer_size: usize,
    // TODO: Offer methods and APIs to handle those
    text_buffer: *mut c_void,
    text_buffer_size: usize,
    custom_dictionary_buffer: *mut c_void,
    custom_dictionary_buffer_size: usize,
}

impl ShowKeyboardArg {
    pub fn new(keyboard_config: KeyboardConfig) -> Self {
        let mut arg = ShowKeyboardArg {
            keyboard_config,
            work_buffer: 0 as _,
            work_buffer_size: 0,
            text_buffer: 0 as _,
            text_buffer_size: 0,
            custom_dictionary_buffer: 0 as _,
            custom_dictionary_buffer_size: 0,
        };

        arg.work_buffer_size = unsafe { get_required_work_buffer_size(false) };
        arg.work_buffer = vec![0u8; arg.work_buffer_size].leak().as_ptr() as _;
        
        arg
    }
}

impl Drop for ShowKeyboardArg {
    fn drop(&mut self) {
        if !self.work_buffer.is_null() {
            unsafe {
                libc::free(self.work_buffer);
            }
        }
        if !self.text_buffer.is_null() {
            unsafe {
                libc::free(self.text_buffer);
            }
        }
        if !self.custom_dictionary_buffer.is_null() {
            unsafe {
                libc::free(self.custom_dictionary_buffer);
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

    #[link_name = "_ZN2nn5swkbd17MakePresetDefaultEPNS0_14KeyboardConfigE"]
    pub fn make_preset_default(keyboard_config: *mut KeyboardConfig);
    
    #[link_name = "_ZN2nn5swkbd18MakePresetPasswordEPNS0_14KeyboardConfigE"]
    pub fn make_preset_password(keyboard_config: *mut KeyboardConfig);

    #[link_name = "_ZN2nn5swkbd18MakePresetUserNameEPNS0_14KeyboardConfigE"]
    pub fn make_preset_user_name(keyboard_config: *mut KeyboardConfig);
    
    #[link_name = "_ZN2nn5swkbd12ShowKeyboardEPNS0_6StringERKNS0_15ShowKeyboardArgE"]
    pub fn show_keyboard(out_string: *mut ShowKeyboardString, arg: *const ShowKeyboardArg) -> u32;

    //#[link_name = "_ZN2nn5swkbd17SetHeaderTextUtf8EPNS0_14KeyboardConfigEPKc"]
    #[link_name = "_ZN2nn5swkbd13SetHeaderTextEPNS0_14KeyboardConfigEPKDs"]
    pub fn set_header_text(keyboard_config: *mut KeyboardConfig, text: *const u16);

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