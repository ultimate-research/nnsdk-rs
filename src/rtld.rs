#[allow(unused_imports)]
use self::super::root;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModuleObject {
    pub next: *mut ModuleObject,
    pub prev: *mut ModuleObject,
    pub rela_or_rel_plt: ModuleObject__bindgen_ty_1,
    pub rela_or_rel: ModuleObject__bindgen_ty_2,
    pub module_base: u64,
    pub dynamic: *mut root::Elf64_Dyn,
    pub is_rela: bool,
    pub rela_or_rel_plt_size: u64,
    pub dt_init: ::std::option::Option<unsafe extern "C" fn()>,
    pub dt_fini: ::std::option::Option<unsafe extern "C" fn()>,
    pub hash_bucket: *mut u32,
    pub hash_chain: *mut u32,
    pub dynstr: *mut u8,
    pub dynsym: *mut root::Elf64_Sym,
    pub dynstr_size: u64,
    pub got: *mut *mut u8,
    pub rela_dyn_size: u64,
    pub rel_dyn_size: u64,
    pub rel_count: u64,
    pub rela_count: u64,
    pub hash_nchain_value: u64,
    pub hash_nbucket_value: u64,
    pub got_stub_ptr: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ModuleObject__bindgen_ty_1 {
    pub rel: *mut root::Elf64_Rel,
    pub rela: *mut root::Elf64_Rela,
    pub raw: *mut u8,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ModuleObject__bindgen_ty_2 {
    pub rel: *mut root::Elf64_Rel,
    pub rela: *mut root::Elf64_Rela,
    _bindgen_union_align: u64,
}
extern "C" {
    #[link_name = "\u{1}_ZN4rtld12ModuleObject10InitializeEmP9Elf64_Dyn"]
    pub fn ModuleObject_Initialize(
        this: *mut ModuleObject,
        aslr_base: u64,
        dynamic: *mut root::Elf64_Dyn,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN4rtld12ModuleObject8RelocateEv"]
    pub fn ModuleObject_Relocate(this: *mut ModuleObject);
}
extern "C" {
    #[link_name = "\u{1}_ZN4rtld12ModuleObject15GetSymbolByNameEPKc"]
    pub fn ModuleObject_GetSymbolByName(
        this: *mut ModuleObject,
        name: *const u8,
    ) -> *mut root::Elf64_Sym;
}
extern "C" {
    #[link_name = "\u{1}_ZN4rtld12ModuleObject14ResolveSymbolsEb"]
    pub fn ModuleObject_ResolveSymbols(
        this: *mut ModuleObject,
        do_lazy_got_init: bool,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN4rtld12ModuleObject16TryResolveSymbolEPmP9Elf64_Sym"]
    pub fn ModuleObject_TryResolveSymbol(
        this: *mut ModuleObject,
        target_symbol_address: *mut root::Elf64_Addr,
        symbol: *mut root::Elf64_Sym,
    ) -> bool;
}
impl ModuleObject {
    #[inline]
    pub unsafe fn Initialize(&mut self, aslr_base: u64, dynamic: *mut root::Elf64_Dyn) {
        ModuleObject_Initialize(self, aslr_base, dynamic)
    }
    #[inline]
    pub unsafe fn Relocate(&mut self) {
        ModuleObject_Relocate(self)
    }
    #[inline]
    pub unsafe fn GetSymbolByName(
        &mut self,
        name: *const u8,
    ) -> *mut root::Elf64_Sym {
        ModuleObject_GetSymbolByName(self, name)
    }
    #[inline]
    pub unsafe fn ResolveSymbols(&mut self, do_lazy_got_init: bool) {
        ModuleObject_ResolveSymbols(self, do_lazy_got_init)
    }
    #[inline]
    pub unsafe fn TryResolveSymbol(
        &mut self,
        target_symbol_address: *mut root::Elf64_Addr,
        symbol: *mut root::Elf64_Sym,
    ) -> bool {
        ModuleObject_TryResolveSymbol(self, target_symbol_address, symbol)
    }
}
