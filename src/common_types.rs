
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::core::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::core::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::core::mem::transmute(self)
    }
}
impl<T> ::core::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::core::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::core::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::core::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::core::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::core::cmp::Eq for __BindgenUnionField<T> {}
#[allow(unused_imports)]
use self::super::root;
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __u_long = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __int_least8_t = root::__int8_t;
pub type __uint_least8_t = root::__uint8_t;
pub type __int_least16_t = root::__int16_t;
pub type __uint_least16_t = root::__uint16_t;
pub type __int_least32_t = root::__int32_t;
pub type __uint_least32_t = root::__uint32_t;
pub type __int_least64_t = root::__int64_t;
pub type __uint_least64_t = root::__uint64_t;
pub type __quad_t = libc::c_long;
pub type __u_quad_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [libc::c_int; 2usize],
}

pub type __clock_t = libc::c_long;
pub type __rlim_t = libc::c_ulong;
pub type __rlim64_t = libc::c_ulong;
pub type __id_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __daddr_t = libc::c_int;
pub type __key_t = libc::c_int;
pub type __clockid_t = libc::c_int;
pub type __timer_t = *mut libc::c_void;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blkcnt64_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsblkcnt64_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsfilcnt64_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __syscall_ulong_t = libc::c_ulong;
pub type __loff_t = root::__off64_t;
pub type __caddr_t = *mut libc::c_char;
pub type __intptr_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type __sig_atomic_t = libc::c_int;
pub type int_least8_t = root::__int_least8_t;
pub type int_least16_t = root::__int_least16_t;
pub type int_least32_t = root::__int_least32_t;
pub type int_least64_t = root::__int_least64_t;
pub type uint_least8_t = root::__uint_least8_t;
pub type uint_least16_t = root::__uint_least16_t;
pub type uint_least32_t = root::__uint_least32_t;
pub type uint_least64_t = root::__uint_least64_t;
pub type int_fast8_t = libc::c_schar;
pub type int_fast16_t = libc::c_long;
pub type int_fast32_t = libc::c_long;
pub type int_fast64_t = libc::c_long;
pub type uint_fast8_t = libc::c_uchar;
pub type uint_fast16_t = libc::c_ulong;
pub type uint_fast32_t = libc::c_ulong;
pub type uint_fast64_t = libc::c_ulong;
pub type intmax_t = root::__intmax_t;
pub type uintmax_t = root::__uintmax_t;
pub type size_t = libc::c_ulong;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct imaxdiv_t {
    pub quot: libc::c_long,
    pub rem: libc::c_long,
}

extern "C" {
    pub fn imaxabs(__n: root::intmax_t) -> root::intmax_t;
}
extern "C" {
    pub fn imaxdiv(__numer: root::intmax_t, __denom: root::intmax_t) -> root::imaxdiv_t;
}
extern "C" {
    pub fn strtoimax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> root::intmax_t;
}
extern "C" {
    pub fn strtoumax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> root::uintmax_t;
}
extern "C" {
    pub fn wcstoimax(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: libc::c_int,
    ) -> root::intmax_t;
}
extern "C" {
    pub fn wcstoumax(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: libc::c_int,
    ) -> root::uintmax_t;
}
extern "C" {
    #[link_name = "\u{1}__cxa_demangle"]
    pub fn cxa_demangle(
        symbol_name: *const u8,
        unk1: *const u8,
        unk2: *const u8,
        result: *mut u32,
    ) -> *mut u8;
}
pub type s8 = i8;
pub type s16 = i16;
pub type s32 = i32;
pub type s64 = i64;
pub type s128 = root::__int128_t;
pub type uchar = libc::c_uchar;
pub type ulong = libc::c_ulong;
pub type uint = u32;
pub type Result = u32;
pub type Handle = u32;
pub type ThreadFunc = ::core::option::Option<unsafe extern "C" fn(arg1: *mut libc::c_void)>;
pub const Module_Kernel: root::_bindgen_ty_1 = 1;
pub const Module_Libnx: root::_bindgen_ty_1 = 345;
pub const Module_HomebrewAbi: root::_bindgen_ty_1 = 346;
pub const Module_HomebrewLoader: root::_bindgen_ty_1 = 347;
pub const Module_LibnxNvidia: root::_bindgen_ty_1 = 348;
pub const Module_LibnxBinder: root::_bindgen_ty_1 = 349;
#[doc = " Module values"]
pub type _bindgen_ty_1 = u32;
pub const KernelError_OutOfSessions: root::_bindgen_ty_2 = 7;
pub const KernelError_InvalidCapabilityDescriptor: root::_bindgen_ty_2 = 14;
pub const KernelError_NotImplemented: root::_bindgen_ty_2 = 33;
pub const KernelError_ThreadTerminating: root::_bindgen_ty_2 = 59;
pub const KernelError_OutOfDebugEvents: root::_bindgen_ty_2 = 70;
pub const KernelError_InvalidSize: root::_bindgen_ty_2 = 101;
pub const KernelError_InvalidAddress: root::_bindgen_ty_2 = 102;
pub const KernelError_ResourceExhausted: root::_bindgen_ty_2 = 103;
pub const KernelError_OutOfMemory: root::_bindgen_ty_2 = 104;
pub const KernelError_OutOfHandles: root::_bindgen_ty_2 = 105;
pub const KernelError_InvalidMemoryState: root::_bindgen_ty_2 = 106;
pub const KernelError_InvalidMemoryPermissions: root::_bindgen_ty_2 = 108;
pub const KernelError_InvalidMemoryRange: root::_bindgen_ty_2 = 110;
pub const KernelError_InvalidPriority: root::_bindgen_ty_2 = 112;
pub const KernelError_InvalidCoreId: root::_bindgen_ty_2 = 113;
pub const KernelError_InvalidHandle: root::_bindgen_ty_2 = 114;
pub const KernelError_InvalidUserBuffer: root::_bindgen_ty_2 = 115;
pub const KernelError_InvalidCombination: root::_bindgen_ty_2 = 116;
pub const KernelError_TimedOut: root::_bindgen_ty_2 = 117;
pub const KernelError_Cancelled: root::_bindgen_ty_2 = 118;
pub const KernelError_OutOfRange: root::_bindgen_ty_2 = 119;
pub const KernelError_InvalidEnumValue: root::_bindgen_ty_2 = 120;
pub const KernelError_NotFound: root::_bindgen_ty_2 = 121;
pub const KernelError_AlreadyExists: root::_bindgen_ty_2 = 122;
pub const KernelError_ConnectionClosed: root::_bindgen_ty_2 = 123;
pub const KernelError_UnhandledUserInterrupt: root::_bindgen_ty_2 = 124;
pub const KernelError_InvalidState: root::_bindgen_ty_2 = 125;
pub const KernelError_ReservedValue: root::_bindgen_ty_2 = 126;
pub const KernelError_InvalidHwBreakpoint: root::_bindgen_ty_2 = 127;
pub const KernelError_FatalUserException: root::_bindgen_ty_2 = 128;
pub const KernelError_OwnedByAnotherProcess: root::_bindgen_ty_2 = 129;
pub const KernelError_ConnectionRefused: root::_bindgen_ty_2 = 131;
pub const KernelError_OutOfResource: root::_bindgen_ty_2 = 132;
pub const KernelError_IpcMapFailed: root::_bindgen_ty_2 = 259;
pub const KernelError_IpcCmdbufTooSmall: root::_bindgen_ty_2 = 260;
pub const KernelError_NotDebugged: root::_bindgen_ty_2 = 520;
#[doc = " Kernel error codes"]
pub type _bindgen_ty_2 = u32;
pub const LibnxError_BadReloc: root::_bindgen_ty_3 = 1;
pub const LibnxError_OutOfMemory: root::_bindgen_ty_3 = 2;
pub const LibnxError_AlreadyMapped: root::_bindgen_ty_3 = 3;
pub const LibnxError_BadGetInfo_Stack: root::_bindgen_ty_3 = 4;
pub const LibnxError_BadGetInfo_Heap: root::_bindgen_ty_3 = 5;
pub const LibnxError_BadQueryMemory: root::_bindgen_ty_3 = 6;
pub const LibnxError_AlreadyInitialized: root::_bindgen_ty_3 = 7;
pub const LibnxError_NotInitialized: root::_bindgen_ty_3 = 8;
pub const LibnxError_NotFound: root::_bindgen_ty_3 = 9;
pub const LibnxError_IoError: root::_bindgen_ty_3 = 10;
pub const LibnxError_BadInput: root::_bindgen_ty_3 = 11;
pub const LibnxError_BadReent: root::_bindgen_ty_3 = 12;
pub const LibnxError_BufferProducerError: root::_bindgen_ty_3 = 13;
pub const LibnxError_HandleTooEarly: root::_bindgen_ty_3 = 14;
pub const LibnxError_HeapAllocFailed: root::_bindgen_ty_3 = 15;
pub const LibnxError_TooManyOverrides: root::_bindgen_ty_3 = 16;
pub const LibnxError_ParcelError: root::_bindgen_ty_3 = 17;
pub const LibnxError_BadGfxInit: root::_bindgen_ty_3 = 18;
pub const LibnxError_BadGfxEventWait: root::_bindgen_ty_3 = 19;
pub const LibnxError_BadGfxQueueBuffer: root::_bindgen_ty_3 = 20;
pub const LibnxError_BadGfxDequeueBuffer: root::_bindgen_ty_3 = 21;
pub const LibnxError_AppletCmdidNotFound: root::_bindgen_ty_3 = 22;
pub const LibnxError_BadAppletReceiveMessage: root::_bindgen_ty_3 = 23;
pub const LibnxError_BadAppletNotifyRunning: root::_bindgen_ty_3 = 24;
pub const LibnxError_BadAppletGetCurrentFocusState: root::_bindgen_ty_3 = 25;
pub const LibnxError_BadAppletGetOperationMode: root::_bindgen_ty_3 = 26;
pub const LibnxError_BadAppletGetPerformanceMode: root::_bindgen_ty_3 = 27;
pub const LibnxError_BadUsbCommsRead: root::_bindgen_ty_3 = 28;
pub const LibnxError_BadUsbCommsWrite: root::_bindgen_ty_3 = 29;
pub const LibnxError_InitFail_SM: root::_bindgen_ty_3 = 30;
pub const LibnxError_InitFail_AM: root::_bindgen_ty_3 = 31;
pub const LibnxError_InitFail_HID: root::_bindgen_ty_3 = 32;
pub const LibnxError_InitFail_FS: root::_bindgen_ty_3 = 33;
pub const LibnxError_BadGetInfo_Rng: root::_bindgen_ty_3 = 34;
pub const LibnxError_JitUnavailable: root::_bindgen_ty_3 = 35;
pub const LibnxError_WeirdKernel: root::_bindgen_ty_3 = 36;
pub const LibnxError_IncompatSysVer: root::_bindgen_ty_3 = 37;
pub const LibnxError_InitFail_Time: root::_bindgen_ty_3 = 38;
pub const LibnxError_TooManyDevOpTabs: root::_bindgen_ty_3 = 39;
pub const LibnxError_DomainMessageUnknownType: root::_bindgen_ty_3 = 40;
pub const LibnxError_DomainMessageTooManyObjectIds: root::_bindgen_ty_3 = 41;
pub const LibnxError_AppletFailedToInitialize: root::_bindgen_ty_3 = 42;
pub const LibnxError_ApmFailedToInitialize: root::_bindgen_ty_3 = 43;
pub const LibnxError_NvinfoFailedToInitialize: root::_bindgen_ty_3 = 44;
pub const LibnxError_NvbufFailedToInitialize: root::_bindgen_ty_3 = 45;
pub const LibnxError_LibAppletBadExit: root::_bindgen_ty_3 = 46;
pub const LibnxError_InvalidCmifOutHeader: root::_bindgen_ty_3 = 47;
pub const LibnxError_ShouldNotHappen: root::_bindgen_ty_3 = 48;
#[doc = " libnx error codes"]
pub type _bindgen_ty_3 = u32;
pub const LibnxBinderError_Unknown: root::_bindgen_ty_4 = 1;
pub const LibnxBinderError_NoMemory: root::_bindgen_ty_4 = 2;
pub const LibnxBinderError_InvalidOperation: root::_bindgen_ty_4 = 3;
pub const LibnxBinderError_BadValue: root::_bindgen_ty_4 = 4;
pub const LibnxBinderError_BadType: root::_bindgen_ty_4 = 5;
pub const LibnxBinderError_NameNotFound: root::_bindgen_ty_4 = 6;
pub const LibnxBinderError_PermissionDenied: root::_bindgen_ty_4 = 7;
pub const LibnxBinderError_NoInit: root::_bindgen_ty_4 = 8;
pub const LibnxBinderError_AlreadyExists: root::_bindgen_ty_4 = 9;
pub const LibnxBinderError_DeadObject: root::_bindgen_ty_4 = 10;
pub const LibnxBinderError_FailedTransaction: root::_bindgen_ty_4 = 11;
pub const LibnxBinderError_BadIndex: root::_bindgen_ty_4 = 12;
pub const LibnxBinderError_NotEnoughData: root::_bindgen_ty_4 = 13;
pub const LibnxBinderError_WouldBlock: root::_bindgen_ty_4 = 14;
pub const LibnxBinderError_TimedOut: root::_bindgen_ty_4 = 15;
pub const LibnxBinderError_UnknownTransaction: root::_bindgen_ty_4 = 16;
pub const LibnxBinderError_FdsNotAllowed: root::_bindgen_ty_4 = 17;
#[doc = " libnx binder error codes"]
pub type _bindgen_ty_4 = u32;
pub const LibnxNvidiaError_Unknown: root::_bindgen_ty_5 = 1;
#[doc = "< Maps to Nvidia: 1"]
pub const LibnxNvidiaError_NotImplemented: root::_bindgen_ty_5 = 2;
#[doc = "< Maps to Nvidia: 2"]
pub const LibnxNvidiaError_NotSupported: root::_bindgen_ty_5 = 3;
#[doc = "< Maps to Nvidia: 3"]
pub const LibnxNvidiaError_NotInitialized: root::_bindgen_ty_5 = 4;
#[doc = "< Maps to Nvidia: 4"]
pub const LibnxNvidiaError_BadParameter: root::_bindgen_ty_5 = 5;
#[doc = "< Maps to Nvidia: 5"]
pub const LibnxNvidiaError_Timeout: root::_bindgen_ty_5 = 6;
#[doc = "< Maps to Nvidia: 6"]
pub const LibnxNvidiaError_InsufficientMemory: root::_bindgen_ty_5 = 7;
#[doc = "< Maps to Nvidia: 7"]
pub const LibnxNvidiaError_ReadOnlyAttribute: root::_bindgen_ty_5 = 8;
#[doc = "< Maps to Nvidia: 8"]
pub const LibnxNvidiaError_InvalidState: root::_bindgen_ty_5 = 9;
#[doc = "< Maps to Nvidia: 9"]
pub const LibnxNvidiaError_InvalidAddress: root::_bindgen_ty_5 = 10;
#[doc = "< Maps to Nvidia: 10"]
pub const LibnxNvidiaError_InvalidSize: root::_bindgen_ty_5 = 11;
#[doc = "< Maps to Nvidia: 11"]
pub const LibnxNvidiaError_BadValue: root::_bindgen_ty_5 = 12;
#[doc = "< Maps to Nvidia: 13"]
pub const LibnxNvidiaError_AlreadyAllocated: root::_bindgen_ty_5 = 13;
#[doc = "< Maps to Nvidia: 14"]
pub const LibnxNvidiaError_Busy: root::_bindgen_ty_5 = 14;
#[doc = "< Maps to Nvidia: 15"]
pub const LibnxNvidiaError_ResourceError: root::_bindgen_ty_5 = 15;
#[doc = "< Maps to Nvidia: 16"]
pub const LibnxNvidiaError_CountMismatch: root::_bindgen_ty_5 = 16;
#[doc = "< Maps to Nvidia: 0x1000"]
pub const LibnxNvidiaError_SharedMemoryTooSmall: root::_bindgen_ty_5 = 17;
#[doc = "< Maps to Nvidia: 0x30003"]
pub const LibnxNvidiaError_FileOperationFailed: root::_bindgen_ty_5 = 18;
#[doc = "< Maps to Nvidia: 0x3000F"]
pub const LibnxNvidiaError_IoctlFailed: root::_bindgen_ty_5 = 19;
#[doc = " libnx nvidia error codes"]
pub type _bindgen_ty_5 = u32;

#[repr(C)]
pub struct nnosMutexType {
    pub curState: u8,
    pub isRecursiveMutex: bool,
    pub lockLevel: root::s32,
    pub _6: [u8; 18usize],
}

pub type Elf32_Half = u16;
pub type Elf64_Half = u16;
pub type Elf32_Word = u32;
pub type Elf32_Sword = i32;
pub type Elf64_Word = u32;
pub type Elf64_Sword = i32;
pub type Elf32_Xword = u64;
pub type Elf32_Sxword = i64;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;
pub type Elf32_Addr = u32;
pub type Elf64_Addr = u64;
pub type Elf32_Off = u32;
pub type Elf64_Off = u64;
pub type Elf32_Section = u16;
pub type Elf64_Section = u16;
pub type Elf32_Versym = root::Elf32_Half;
pub type Elf64_Versym = root::Elf64_Half;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Ehdr {
    pub e_ident: [libc::c_uchar; 16usize],
    pub e_type: root::Elf32_Half,
    pub e_machine: root::Elf32_Half,
    pub e_version: root::Elf32_Word,
    pub e_entry: root::Elf32_Addr,
    pub e_phoff: root::Elf32_Off,
    pub e_shoff: root::Elf32_Off,
    pub e_flags: root::Elf32_Word,
    pub e_ehsize: root::Elf32_Half,
    pub e_phentsize: root::Elf32_Half,
    pub e_phnum: root::Elf32_Half,
    pub e_shentsize: root::Elf32_Half,
    pub e_shnum: root::Elf32_Half,
    pub e_shstrndx: root::Elf32_Half,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Ehdr {
    pub e_ident: [libc::c_uchar; 16usize],
    pub e_type: root::Elf64_Half,
    pub e_machine: root::Elf64_Half,
    pub e_version: root::Elf64_Word,
    pub e_entry: root::Elf64_Addr,
    pub e_phoff: root::Elf64_Off,
    pub e_shoff: root::Elf64_Off,
    pub e_flags: root::Elf64_Word,
    pub e_ehsize: root::Elf64_Half,
    pub e_phentsize: root::Elf64_Half,
    pub e_phnum: root::Elf64_Half,
    pub e_shentsize: root::Elf64_Half,
    pub e_shnum: root::Elf64_Half,
    pub e_shstrndx: root::Elf64_Half,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Shdr {
    pub sh_name: root::Elf32_Word,
    pub sh_type: root::Elf32_Word,
    pub sh_flags: root::Elf32_Word,
    pub sh_addr: root::Elf32_Addr,
    pub sh_offset: root::Elf32_Off,
    pub sh_size: root::Elf32_Word,
    pub sh_link: root::Elf32_Word,
    pub sh_info: root::Elf32_Word,
    pub sh_addralign: root::Elf32_Word,
    pub sh_entsize: root::Elf32_Word,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Shdr {
    pub sh_name: root::Elf64_Word,
    pub sh_type: root::Elf64_Word,
    pub sh_flags: root::Elf64_Xword,
    pub sh_addr: root::Elf64_Addr,
    pub sh_offset: root::Elf64_Off,
    pub sh_size: root::Elf64_Xword,
    pub sh_link: root::Elf64_Word,
    pub sh_info: root::Elf64_Word,
    pub sh_addralign: root::Elf64_Xword,
    pub sh_entsize: root::Elf64_Xword,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Chdr {
    pub ch_type: root::Elf32_Word,
    pub ch_size: root::Elf32_Word,
    pub ch_addralign: root::Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Chdr {
    pub ch_type: root::Elf64_Word,
    pub ch_reserved: root::Elf64_Word,
    pub ch_size: root::Elf64_Xword,
    pub ch_addralign: root::Elf64_Xword,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Sym {
    pub st_name: root::Elf32_Word,
    pub st_value: root::Elf32_Addr,
    pub st_size: root::Elf32_Word,
    pub st_info: libc::c_uchar,
    pub st_other: libc::c_uchar,
    pub st_shndx: root::Elf32_Section,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Sym {
    pub st_name: root::Elf64_Word,
    pub st_info: libc::c_uchar,
    pub st_other: libc::c_uchar,
    pub st_shndx: root::Elf64_Section,
    pub st_value: root::Elf64_Addr,
    pub st_size: root::Elf64_Xword,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Syminfo {
    pub si_boundto: root::Elf32_Half,
    pub si_flags: root::Elf32_Half,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Syminfo {
    pub si_boundto: root::Elf64_Half,
    pub si_flags: root::Elf64_Half,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Rel {
    pub r_offset: root::Elf32_Addr,
    pub r_info: root::Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Rel {
    pub r_offset: root::Elf64_Addr,
    pub r_info: root::Elf64_Xword,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Rela {
    pub r_offset: root::Elf32_Addr,
    pub r_info: root::Elf32_Word,
    pub r_addend: root::Elf32_Sword,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Rela {
    pub r_offset: root::Elf64_Addr,
    pub r_info: root::Elf64_Xword,
    pub r_addend: root::Elf64_Sxword,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Phdr {
    pub p_type: root::Elf32_Word,
    pub p_offset: root::Elf32_Off,
    pub p_vaddr: root::Elf32_Addr,
    pub p_paddr: root::Elf32_Addr,
    pub p_filesz: root::Elf32_Word,
    pub p_memsz: root::Elf32_Word,
    pub p_flags: root::Elf32_Word,
    pub p_align: root::Elf32_Word,
}

pub struct Elf64_Phdr {
    pub p_type: root::Elf64_Word,
    pub p_flags: root::Elf64_Word,
    pub p_offset: root::Elf64_Off,
    pub p_vaddr: root::Elf64_Addr,
    pub p_paddr: root::Elf64_Addr,
    pub p_filesz: root::Elf64_Xword,
    pub p_memsz: root::Elf64_Xword,
    pub p_align: root::Elf64_Xword,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf32_Dyn {
    pub d_tag: root::Elf32_Sword,
    pub d_un: root::Elf32_Dyn__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Elf32_Dyn__bindgen_ty_1 {
    pub d_val: root::Elf32_Word,
    pub d_ptr: root::Elf32_Addr,
    _bindgen_union_align: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_Dyn {
    pub d_tag: root::Elf64_Sxword,
    pub d_un: root::Elf64_Dyn__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Elf64_Dyn__bindgen_ty_1 {
    pub d_val: root::Elf64_Xword,
    pub d_ptr: root::Elf64_Addr,
    _bindgen_union_align: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Verdef {
    pub vd_version: root::Elf32_Half,
    pub vd_flags: root::Elf32_Half,
    pub vd_ndx: root::Elf32_Half,
    pub vd_cnt: root::Elf32_Half,
    pub vd_hash: root::Elf32_Word,
    pub vd_aux: root::Elf32_Word,
    pub vd_next: root::Elf32_Word,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Verdef {
    pub vd_version: root::Elf64_Half,
    pub vd_flags: root::Elf64_Half,
    pub vd_ndx: root::Elf64_Half,
    pub vd_cnt: root::Elf64_Half,
    pub vd_hash: root::Elf64_Word,
    pub vd_aux: root::Elf64_Word,
    pub vd_next: root::Elf64_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Verdaux {
    pub vda_name: root::Elf32_Word,
    pub vda_next: root::Elf32_Word,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Verdaux {
    pub vda_name: root::Elf64_Word,
    pub vda_next: root::Elf64_Word,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Verneed {
    pub vn_version: root::Elf32_Half,
    pub vn_cnt: root::Elf32_Half,
    pub vn_file: root::Elf32_Word,
    pub vn_aux: root::Elf32_Word,
    pub vn_next: root::Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Verneed {
    pub vn_version: root::Elf64_Half,
    pub vn_cnt: root::Elf64_Half,
    pub vn_file: root::Elf64_Word,
    pub vn_aux: root::Elf64_Word,
    pub vn_next: root::Elf64_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Vernaux {
    pub vna_hash: root::Elf32_Word,
    pub vna_flags: root::Elf32_Half,
    pub vna_other: root::Elf32_Half,
    pub vna_name: root::Elf32_Word,
    pub vna_next: root::Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Vernaux {
    pub vna_hash: root::Elf64_Word,
    pub vna_flags: root::Elf64_Half,
    pub vna_other: root::Elf64_Half,
    pub vna_name: root::Elf64_Word,
    pub vna_next: root::Elf64_Word,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf32_auxv_t {
    pub a_type: u32,
    pub a_un: root::Elf32_auxv_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Elf32_auxv_t__bindgen_ty_1 {
    pub a_val: u32,
    _bindgen_union_align: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_auxv_t {
    pub a_type: u64,
    pub a_un: root::Elf64_auxv_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Elf64_auxv_t__bindgen_ty_1 {
    pub a_val: u64,
    _bindgen_union_align: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Nhdr {
    pub n_namesz: root::Elf32_Word,
    pub n_descsz: root::Elf32_Word,
    pub n_type: root::Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Nhdr {
    pub n_namesz: root::Elf64_Word,
    pub n_descsz: root::Elf64_Word,
    pub n_type: root::Elf64_Word,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Move {
    pub m_value: root::Elf32_Xword,
    pub m_info: root::Elf32_Word,
    pub m_poffset: root::Elf32_Word,
    pub m_repeat: root::Elf32_Half,
    pub m_stride: root::Elf32_Half,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Move {
    pub m_value: root::Elf64_Xword,
    pub m_info: root::Elf64_Xword,
    pub m_poffset: root::Elf64_Xword,
    pub m_repeat: root::Elf64_Half,
    pub m_stride: root::Elf64_Half,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Elf32_gptab {
    pub gt_header: root::Elf32_gptab__bindgen_ty_1,
    pub gt_entry: root::Elf32_gptab__bindgen_ty_2,
    _bindgen_union_align: [u32; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_gptab__bindgen_ty_1 {
    pub gt_current_g_value: root::Elf32_Word,
    pub gt_unused: root::Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_gptab__bindgen_ty_2 {
    pub gt_g_value: root::Elf32_Word,
    pub gt_bytes: root::Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_RegInfo {
    pub ri_gprmask: root::Elf32_Word,
    pub ri_cprmask: [root::Elf32_Word; 4usize],
    pub ri_gp_value: root::Elf32_Sword,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf_Options {
    pub kind: libc::c_uchar,
    pub size: libc::c_uchar,
    pub section: root::Elf32_Section,
    pub info: root::Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf_Options_Hw {
    pub hwp_flags1: root::Elf32_Word,
    pub hwp_flags2: root::Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Lib {
    pub l_name: root::Elf32_Word,
    pub l_time_stamp: root::Elf32_Word,
    pub l_checksum: root::Elf32_Word,
    pub l_version: root::Elf32_Word,
    pub l_flags: root::Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Lib {
    pub l_name: root::Elf64_Word,
    pub l_time_stamp: root::Elf64_Word,
    pub l_checksum: root::Elf64_Word,
    pub l_version: root::Elf64_Word,
    pub l_flags: root::Elf64_Word,
}
pub type Elf32_Conflict = root::Elf32_Addr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf_MIPS_ABIFlags_v0 {
    pub version: root::Elf32_Half,
    pub isa_level: libc::c_uchar,
    pub isa_rev: libc::c_uchar,
    pub gpr_size: libc::c_uchar,
    pub cpr1_size: libc::c_uchar,
    pub cpr2_size: libc::c_uchar,
    pub fp_abi: libc::c_uchar,
    pub isa_ext: root::Elf32_Word,
    pub ases: root::Elf32_Word,
    pub flags1: root::Elf32_Word,
    pub flags2: root::Elf32_Word,
}
pub const Val_GNU_MIPS_ABI_FP_ANY: root::_bindgen_ty_6 = 0;
pub const Val_GNU_MIPS_ABI_FP_DOUBLE: root::_bindgen_ty_6 = 1;
pub const Val_GNU_MIPS_ABI_FP_SINGLE: root::_bindgen_ty_6 = 2;
pub const Val_GNU_MIPS_ABI_FP_SOFT: root::_bindgen_ty_6 = 3;
pub const Val_GNU_MIPS_ABI_FP_OLD_64: root::_bindgen_ty_6 = 4;
pub const Val_GNU_MIPS_ABI_FP_XX: root::_bindgen_ty_6 = 5;
pub const Val_GNU_MIPS_ABI_FP_64: root::_bindgen_ty_6 = 6;
pub const Val_GNU_MIPS_ABI_FP_64A: root::_bindgen_ty_6 = 7;
pub const Val_GNU_MIPS_ABI_FP_MAX: root::_bindgen_ty_6 = 7;
pub type _bindgen_ty_6 = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: root::size_t,
}
pub type u_char = root::__u_char;
pub type u_short = root::__u_short;
pub type u_int = root::__u_int;
pub type u_long = root::__u_long;
pub type quad_t = root::__quad_t;
pub type u_quad_t = root::__u_quad_t;
pub type fsid_t = root::__fsid_t;
pub type loff_t = root::__loff_t;
pub type ino_t = root::__ino_t;
pub type ino64_t = root::__ino64_t;
pub type dev_t = root::__dev_t;
pub type gid_t = root::__gid_t;
pub type mode_t = root::__mode_t;
pub type nlink_t = root::__nlink_t;
pub type uid_t = root::__uid_t;
pub type off_t = root::__off_t;
pub type off64_t = root::__off64_t;
pub type pid_t = root::__pid_t;
pub type id_t = root::__id_t;
pub type ssize_t = root::__ssize_t;
pub type daddr_t = root::__daddr_t;
pub type caddr_t = root::__caddr_t;
pub type key_t = root::__key_t;
pub type clock_t = root::__clock_t;
pub type clockid_t = root::__clockid_t;
pub type time_t = root::__time_t;
pub type timer_t = root::__timer_t;
pub type useconds_t = root::__useconds_t;
pub type suseconds_t = root::__suseconds_t;
pub type ushort = libc::c_ushort;
pub type u_int8_t = root::__uint8_t;
pub type u_int16_t = root::__uint16_t;
pub type u_int32_t = root::__uint32_t;
pub type u_int64_t = root::__uint64_t;
pub type register_t = libc::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16usize],
}
pub type sigset_t = root::__sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: root::__time_t,
    pub tv_usec: root::__suseconds_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: root::__time_t,
    pub tv_nsec: root::__syscall_slong_t,
}
pub type __fd_mask = libc::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub fds_bits: [root::__fd_mask; 16usize],
}
pub type fd_mask = root::__fd_mask;
pub type blksize_t = root::__blksize_t;
pub type blkcnt_t = root::__blkcnt_t;
pub type fsblkcnt_t = root::__fsblkcnt_t;
pub type fsfilcnt_t = root::__fsfilcnt_t;
pub type blkcnt64_t = root::__blkcnt64_t;
pub type fsblkcnt64_t = root::__fsblkcnt64_t;
pub type fsfilcnt64_t = root::__fsfilcnt64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: libc::c_uint,
    pub __writers: libc::c_uint,
    pub __wrphase_futex: libc::c_uint,
    pub __writers_futex: libc::c_uint,
    pub __pad3: libc::c_uint,
    pub __pad4: libc::c_uint,
    pub __cur_writer: libc::c_int,
    pub __shared: libc::c_int,
    pub __rwelision: libc::c_schar,
    pub __pad1: [libc::c_uchar; 7usize],
    pub __pad2: libc::c_ulong,
    pub __flags: libc::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut root::__pthread_internal_list,
    pub __next: *mut root::__pthread_internal_list,
}
pub type __pthread_list_t = root::__pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: root::__pthread_list_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: root::__pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: root::__pthread_cond_s__bindgen_ty_2,
    pub __g_refs: [libc::c_uint; 2usize],
    pub __g_size: [libc::c_uint; 2usize],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: root::__pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: root::__pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_t = libc::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4usize],
    pub __align: libc::c_int,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4usize],
    pub __align: libc::c_int,
    _bindgen_union_align: u32,
}
pub type pthread_key_t = libc::c_uint;
pub type pthread_once_t = libc::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56usize],
    pub __align: libc::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: root::__pthread_mutex_s,
    pub __size: [libc::c_char; 40usize],
    pub __align: libc::c_long,
    _bindgen_union_align: [u64; 5usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: root::__pthread_cond_s,
    pub __size: [libc::c_char; 48usize],
    pub __align: libc::c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: root::__pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56usize],
    pub __align: libc::c_long,
    _bindgen_union_align: [u64; 7usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8usize],
    pub __align: libc::c_long,
    _bindgen_union_align: u64,
}
pub type pthread_spinlock_t = libc::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [libc::c_char; 32usize],
    pub __align: libc::c_long,
    _bindgen_union_align: [u64; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [libc::c_char; 4usize],
    pub __align: libc::c_int,
    _bindgen_union_align: u32,
}
pub type socklen_t = root::__socklen_t;
pub const __socket_type_SOCK_STREAM: root::__socket_type = 1;
pub const __socket_type_SOCK_DGRAM: root::__socket_type = 2;
pub const __socket_type_SOCK_RAW: root::__socket_type = 3;
pub const __socket_type_SOCK_RDM: root::__socket_type = 4;
pub const __socket_type_SOCK_SEQPACKET: root::__socket_type = 5;
pub const __socket_type_SOCK_DCCP: root::__socket_type = 6;
pub const __socket_type_SOCK_PACKET: root::__socket_type = 10;
pub const __socket_type_SOCK_CLOEXEC: root::__socket_type = 524288;
pub const __socket_type_SOCK_NONBLOCK: root::__socket_type = 2048;
pub type __socket_type = u32;
pub type sa_family_t = libc::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
    pub sa_family: root::sa_family_t,
    pub sa_data: [libc::c_char; 14usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_storage {
    pub ss_family: root::sa_family_t,
    pub __ss_padding: [libc::c_char; 118usize],
    pub __ss_align: libc::c_ulong,
}
pub const MSG_OOB: root::_bindgen_ty_7 = 1;
pub const MSG_PEEK: root::_bindgen_ty_7 = 2;
pub const MSG_DONTROUTE: root::_bindgen_ty_7 = 4;
pub const MSG_TRYHARD: root::_bindgen_ty_7 = 4;
pub const MSG_CTRUNC: root::_bindgen_ty_7 = 8;
pub const MSG_PROXY: root::_bindgen_ty_7 = 16;
pub const MSG_TRUNC: root::_bindgen_ty_7 = 32;
pub const MSG_DONTWAIT: root::_bindgen_ty_7 = 64;
pub const MSG_EOR: root::_bindgen_ty_7 = 128;
pub const MSG_WAITALL: root::_bindgen_ty_7 = 256;
pub const MSG_FIN: root::_bindgen_ty_7 = 512;
pub const MSG_SYN: root::_bindgen_ty_7 = 1024;
pub const MSG_CONFIRM: root::_bindgen_ty_7 = 2048;
pub const MSG_RST: root::_bindgen_ty_7 = 4096;
pub const MSG_ERRQUEUE: root::_bindgen_ty_7 = 8192;
pub const MSG_NOSIGNAL: root::_bindgen_ty_7 = 16384;
pub const MSG_MORE: root::_bindgen_ty_7 = 32768;
pub const MSG_WAITFORONE: root::_bindgen_ty_7 = 65536;
pub const MSG_BATCH: root::_bindgen_ty_7 = 262144;
pub const MSG_ZEROCOPY: root::_bindgen_ty_7 = 67108864;
pub const MSG_FASTOPEN: root::_bindgen_ty_7 = 536870912;
pub const MSG_CMSG_CLOEXEC: root::_bindgen_ty_7 = 1073741824;
pub type _bindgen_ty_7 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: root::socklen_t,
    pub msg_iov: *mut root::iovec,
    pub msg_iovlen: root::size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: root::size_t,
    pub msg_flags: libc::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct cmsghdr {
    pub cmsg_len: root::size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: root::__IncompleteArrayField<libc::c_uchar>,
}
pub const SCM_RIGHTS: root::_bindgen_ty_8 = 1;
pub const SCM_CREDENTIALS: root::_bindgen_ty_8 = 2;
pub type _bindgen_ty_8 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ucred {
    pub pid: root::pid_t,
    pub uid: root::uid_t,
    pub gid: root::gid_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fd_set {
    pub fds_bits: [libc::c_ulong; 16usize],
}
pub type __kernel_sighandler_t =
    ::core::option::Option<unsafe extern "C" fn(arg1: libc::c_int)>;
pub type __kernel_key_t = libc::c_int;
pub type __kernel_mqd_t = libc::c_int;
pub type __kernel_old_uid_t = libc::c_ushort;
pub type __kernel_old_gid_t = libc::c_ushort;
pub type __kernel_old_dev_t = libc::c_ulong;
pub type __kernel_long_t = libc::c_long;
pub type __kernel_ulong_t = libc::c_ulong;
pub type __kernel_ino_t = root::__kernel_ulong_t;
pub type __kernel_mode_t = libc::c_uint;
pub type __kernel_pid_t = libc::c_int;
pub type __kernel_ipc_pid_t = libc::c_int;
pub type __kernel_uid_t = libc::c_uint;
pub type __kernel_gid_t = libc::c_uint;
pub type __kernel_suseconds_t = root::__kernel_long_t;
pub type __kernel_daddr_t = libc::c_int;
pub type __kernel_uid32_t = libc::c_uint;
pub type __kernel_gid32_t = libc::c_uint;
pub type __kernel_size_t = root::__kernel_ulong_t;
pub type __kernel_ssize_t = root::__kernel_long_t;
pub type __kernel_ptrdiff_t = root::__kernel_long_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fsid_t {
    pub val: [libc::c_int; 2usize],
}
pub type __kernel_off_t = root::__kernel_long_t;
pub type __kernel_loff_t = libc::c_longlong;
pub type __kernel_time_t = root::__kernel_long_t;
pub type __kernel_time64_t = libc::c_longlong;
pub type __kernel_clock_t = root::__kernel_long_t;
pub type __kernel_timer_t = libc::c_int;
pub type __kernel_clockid_t = libc::c_int;
pub type __kernel_caddr_t = *mut libc::c_char;
pub type __kernel_uid16_t = libc::c_ushort;
pub type __kernel_gid16_t = libc::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct linger {
    pub l_onoff: libc::c_int,
    pub l_linger: libc::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osockaddr {
    pub sa_family: libc::c_ushort,
    pub sa_data: [libc::c_uchar; 14usize],
}
pub const SHUT_RD: root::_bindgen_ty_9 = 0;
pub const SHUT_WR: root::_bindgen_ty_9 = 1;
pub const SHUT_RDWR: root::_bindgen_ty_9 = 2;
pub type _bindgen_ty_9 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mmsghdr {
    pub msg_hdr: root::msghdr,
    pub msg_len: libc::c_uint,
}
pub type __int128_t = i128;
pub type __uint128_t = u128;