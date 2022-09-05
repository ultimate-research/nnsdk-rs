#[allow(unused_imports)]
use self::super::root;
#[repr(C)]
pub struct Module {
    pub ModuleObject: *mut root::rtld::ModuleObject,
    pub State: u32,
    pub NroPtr: *mut libc::c_void,
    pub BssPtr: *mut libc::c_void,
    pub _x20: *mut libc::c_void,
    pub SourceBuffer: *mut libc::c_void,
    pub Name: [libc::c_char; 256usize],
    pub _x130: u8,
    pub _x131: u8,
    pub isLoaded: bool,
}

#[repr(C)]
pub struct ModuleId {
    pub build_id: [u8; 32usize],
}

#[repr(C)]
pub struct NroHeader {
    pub entrypoint_insn: u32,
    pub mod_offset: u32,
    pub _x8: [u8; 8usize],
    pub magic: u32,
    pub _x14: [u8; 4usize],
    pub size: u32,
    pub reserved_1C: [u8; 4usize],
    pub text_offset: u32,
    pub text_size: u32,
    pub ro_offset: u32,
    pub ro_size: u32,
    pub rw_offset: u32,
    pub rw_size: u32,
    pub bss_size: u32,
    pub _x3C: [u8; 4usize],
    pub module_id: root::nn::ro::ModuleId,
    pub _x60: [u8; 32usize],
}

#[repr(C)]
pub struct ProgramId {
    pub value: u64,
}

#[repr(C)]
pub struct NrrHeader {
    pub magic: u32,
    pub _x4: [u8; 12usize],
    pub program_id_mask: u64,
    pub program_id_pattern: u64,
    pub _x20: [u8; 16usize],
    pub modulus: [u8; 256usize],
    pub fixed_key_signature: [u8; 256usize],
    pub nrr_signature: [u8; 256usize],
    pub program_id: root::nn::ro::ProgramId,
    pub size: u32,
    pub type_: u8,
    pub _x33D: [u8; 3usize],
    pub hashes_offset: u32,
    pub num_hashes: u32,
    pub _x348: [u8; 8usize],
}

#[repr(C)]
pub struct RegistrationInfo {
    pub state: root::nn::ro::RegistrationInfo_State,
    pub nrrPtr: *mut root::nn::ro::NrrHeader,
    pub _x10: u64,
    pub _x18: u64,
}
pub const RegistrationInfo_State_State_Unregistered:
    root::nn::ro::RegistrationInfo_State = 0;
pub const RegistrationInfo_State_State_Registered:
    root::nn::ro::RegistrationInfo_State = 1;
pub type RegistrationInfo_State = u32;

pub const BindFlag_BindFlag_Now: root::nn::ro::BindFlag = 1;
pub const BindFlag_BindFlag_Lazy: root::nn::ro::BindFlag = 2;
pub type BindFlag = u32;
extern "C" {
    #[link_name = "\u{1}_ZN2nn2ro10InitializeEv"]
    pub fn Initialize() -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2ro12LookupSymbolEPmPKc"]
    pub fn LookupSymbol(
        pOutAddress: *mut usize,
        name: *const libc::c_char,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2ro18LookupModuleSymbolEPmPKNS0_6ModuleEPKc"]
    pub fn LookupModuleSymbol(
        pOutAddress: *mut usize,
        pModule: *const root::nn::ro::Module,
        name: *const libc::c_char,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2ro10LoadModuleEPNS0_6ModuleEPKvPvmi"]
    pub fn LoadModule(
        pOutModule: *mut root::nn::ro::Module,
        pImage: *const libc::c_void,
        buffer: *mut libc::c_void,
        bufferSize: root::size_t,
        flag: libc::c_int,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2ro12UnloadModuleEPNS0_6ModuleE"]
    pub fn UnloadModule(arg1: *mut root::nn::ro::Module) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2ro13GetBufferSizeEPmPKv"]
    pub fn GetBufferSize(
        arg1: *mut root::size_t,
        arg2: *const libc::c_void,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2ro18RegisterModuleInfoEPNS0_16RegistrationInfoEPKv"]
    pub fn RegisterModuleInfo(
        arg1: *mut root::nn::ro::RegistrationInfo,
        arg2: *const libc::c_void,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2ro18RegisterModuleInfoEPNS0_16RegistrationInfoEPKvj"]
    pub fn RegisterModuleInfo1(
        arg1: *mut root::nn::ro::RegistrationInfo,
        arg2: *const libc::c_void,
        arg3: root::uint,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2ro20UnregisterModuleInfoEPNS0_16RegistrationInfoE"]
    pub fn UnregisterModuleInfo(
        arg1: *mut root::nn::ro::RegistrationInfo,
    ) -> root::Result;
}