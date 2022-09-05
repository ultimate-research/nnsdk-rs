#[allow(unused_imports)]
use self::super::root;
pub mod detail;

pub type Tick = u64;
pub type LightEventType = u64;
#[repr(C)]
pub struct EventType {
    pub _x0: *mut root::nn::os::EventType,
    pub _x8: *mut root::nn::os::EventType,
    pub isSignaled: bool,
    pub initiallySignaled: bool,
    pub shouldAutoClear: bool,
    pub isInit: bool,
    pub signalCounter: u32,
    pub signalCounter2: u32,
    pub crit: root::nn::os::detail::InternalCriticalSection,
    pub condvar: root::nn::os::detail::InternalConditionVariable,
}

pub type Event = root::nn::os::EventType;
pub const EventClearMode_EventClearMode_ManualClear: root::nn::os::EventClearMode = 0;
pub const EventClearMode_EventClearMode_AutoClear: root::nn::os::EventClearMode = 1;
pub type EventClearMode = u32;
#[repr(C)]
pub struct ThreadType {
    pub _0: [u8; 64usize],
    pub State: u32,
    pub _44: bool,
    pub _45: bool,
    pub _46: u8,
    pub PriorityBase: u32,
    pub StackBase: *mut libc::c_void,
    pub Stack: *mut libc::c_void,
    pub StackSize: root::size_t,
    pub Arg: *mut libc::c_void,
    pub ThreadFunc: u64,
    pub _88: [u8; 256usize],
    pub Name: [u8; 32usize],
    pub Crit: root::nn::os::detail::InternalCriticalSection,
    pub Condvar: root::nn::os::detail::InternalConditionVariable,
    pub Handle: u32,
    pub padding: [u8; 24usize],
}

pub struct MessageQueueType {
    pub _x0: u64,
    pub _x8: u64,
    pub _x10: u64,
    pub _x18: u64,
    pub Buffer: *mut libc::c_void,
    pub MaxCount: u32,
    pub Count: u32,
    pub Offset: u32,
    pub Initialized: bool,
    pub _x38: root::nn::os::detail::InternalCriticalSection,
    pub _x3C: root::nn::os::detail::InternalConditionVariable,
    pub _x40: root::nn::os::detail::InternalConditionVariable,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ConditionVariableType {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SystemEvent {
    _unused: [u8; 0x28],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SystemEventType {
    pub _unused: [u8; 0x29],
}
impl SystemEventType {
    pub fn new(clear_mode: SystemEventClearMode) -> Self {
        let x = Self { _unused: [0; 0x29] };
        unsafe { CreateSystemEvent(&x, clear_mode, false) };
        x
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum SystemEventClearMode {
    Manual = 0,
    Auto = 1,
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os11SetHostArgcEi"]
    pub fn SetHostArgc(arg1: root::s32);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os11GetHostArgcEv"]
    pub fn GetHostArgc() -> root::s32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os11SetHostArgvEPPc"]
    pub fn SetHostArgv(arg1: *mut *mut u8);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os11GetHostArgvEv"]
    pub fn GetHostArgv() -> *mut *mut u8;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os30InitializeVirtualAddressMemoryEv"]
    pub fn InitializeVirtualAddressMemory();
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os21AllocateAddressRegionEPmm"]
    pub fn AllocateAddressRegion(arg1: *mut u64, arg2: u64) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os14AllocateMemoryEPmm"]
    pub fn AllocateMemory(arg1: *mut u64, arg2: u64) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os19AllocateMemoryPagesEmm"]
    pub fn AllocateMemoryPages(arg1: u64, arg2: u64) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os19AllocateMemoryBlockEPmm"]
    pub fn AllocateMemoryBlock(arg1: *mut u64, arg2: u64);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os15FreeMemoryBlockEmm"]
    pub fn FreeMemoryBlock(arg1: u64, arg2: u64);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os17SetMemoryHeapSizeEm"]
    pub fn SetMemoryHeapSize(arg1: u64);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os17CreateSystemEventEPNS0_15SystemEventTypeENS0_14EventClearModeEb"]
    pub fn CreateSystemEvent(
        arg1: *const SystemEventType,
        clear_mode: SystemEventClearMode,
        skip_init: bool,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os18TryWaitSystemEventEPNS0_15SystemEventTypeE"]
    pub fn TryWaitSystemEvent(arg1: *const SystemEventType) -> bool;
}
#[repr(C)]
pub struct MutexType {
    pub impl_: root::nnosMutexType,
}


extern "C" {
    #[link_name = "\u{1}_ZN2nn2os15InitializeMutexEPNS0_9MutexTypeEbi"]
    pub fn InitializeMutex(
        arg1: *mut root::nn::os::MutexType,
        arg2: bool,
        arg3: root::s32,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os13FinalizeMutexEPNS0_9MutexTypeE"]
    pub fn FinalizeMutex(arg1: *mut root::nn::os::MutexType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os9LockMutexEPNS0_9MutexTypeE"]
    pub fn LockMutex(arg1: *mut root::nn::os::MutexType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os12TryLockMutexEPNS0_9MutexTypeE"]
    pub fn TryLockMutex(arg1: *mut root::nn::os::MutexType) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os11UnlockMutexEPNS0_9MutexTypeE"]
    pub fn UnlockMutex(arg1: *mut root::nn::os::MutexType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os28IsMutexLockedByCurrentThreadEPKNS0_9MutexTypeE"]
    pub fn IsMutexLockedByCurrentThread(arg1: *const root::nn::os::MutexType) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os22InitializeMessageQueueEPNS0_16MessageQueueTypeEPmm"]
    pub fn InitializeMessageQueue(
        arg1: *mut root::nn::os::MessageQueueType,
        buf: *mut u64,
        queueCount: u64,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os20FinalizeMessageQueueEPNS0_16MessageQueueTypeE"]
    pub fn FinalizeMessageQueue(arg1: *mut root::nn::os::MessageQueueType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os19TrySendMessageQueueEPNS0_16MessageQueueTypeEm"]
    pub fn TrySendMessageQueue(
        arg1: *mut root::nn::os::MessageQueueType,
        arg2: u64,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os16SendMessageQueueEPNS0_16MessageQueueTypeEm"]
    pub fn SendMessageQueue(arg1: *mut root::nn::os::MessageQueueType, arg2: u64);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os21TimedSendMessageQueueEPNS0_16MessageQueueTypeEmNS_8TimeSpanE"]
    pub fn TimedSendMessageQueue(
        arg1: *mut root::nn::os::MessageQueueType,
        arg2: u64,
        arg3: root::nn::TimeSpan,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os22TryReceiveMessageQueueEPmPNS0_16MessageQueueTypeE"]
    pub fn TryReceiveMessageQueue(
        out: *mut u64,
        arg1: *mut root::nn::os::MessageQueueType,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os19ReceiveMessageQueueEPmPNS0_16MessageQueueTypeE"]
    pub fn ReceiveMessageQueue(
        out: *mut u64,
        arg1: *mut root::nn::os::MessageQueueType,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os24TimedReceiveMessageQueueEPmPNS0_16MessageQueueTypeENS_8TimeSpanE"]
    pub fn TimedReceiveMessageQueue(
        out: *mut u64,
        arg1: *mut root::nn::os::MessageQueueType,
        arg2: root::nn::TimeSpan,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os19TryPeekMessageQueueEPmPKNS0_16MessageQueueTypeE"]
    pub fn TryPeekMessageQueue(
        arg1: *mut u64,
        arg2: *const root::nn::os::MessageQueueType,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os16PeekMessageQueueEPmPKNS0_16MessageQueueTypeE"]
    pub fn PeekMessageQueue(
        arg1: *mut u64,
        arg2: *const root::nn::os::MessageQueueType,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os21TimedPeekMessageQueueEPmPKNS0_16MessageQueueTypeE"]
    pub fn TimedPeekMessageQueue(
        arg1: *mut u64,
        arg2: *const root::nn::os::MessageQueueType,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os18TryJamMessageQueueEPNS0_16MessageQueueTypeEm"]
    pub fn TryJamMessageQueue(
        arg1: *mut root::nn::os::MessageQueueType,
        arg2: u64,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os15JamMessageQueueEPNS0_16MessageQueueTypeEm"]
    pub fn JamMessageQueue(arg1: *mut root::nn::os::MessageQueueType, arg2: u64);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os20TimedJamMessageQueueEPNS0_16MessageQueueTypeEmNS_8TimeSpanE"]
    pub fn TimedJamMessageQueue(
        arg1: *mut root::nn::os::MessageQueueType,
        arg2: u64,
        arg3: root::nn::TimeSpan,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os27InitializeConditionVariableEPNS0_21ConditionVariableTypeE"]
    pub fn InitializeConditionVariable(arg1: *mut root::nn::os::ConditionVariableType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os25FinalizeConditionVariableEPNS0_21ConditionVariableTypeE"]
    pub fn FinalizeConditionVariable(arg1: *mut root::nn::os::ConditionVariableType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os23SignalConditionVariableEPNS0_21ConditionVariableTypeE"]
    pub fn SignalConditionVariable(arg1: *mut root::nn::os::ConditionVariableType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os26BroadcastConditionVariableEPNS0_21ConditionVariableTypeE"]
    pub fn BroadcastConditionVariable(arg1: *mut root::nn::os::ConditionVariableType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os21WaitConditionVariableEPNS0_21ConditionVariableTypeE"]
    pub fn WaitConditionVariable(arg1: *mut root::nn::os::ConditionVariableType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os26TimedWaitConditionVariableEPNS0_21ConditionVariableTypeEPNS0_9MutexTypeENS_8TimeSpanE"]
    pub fn TimedWaitConditionVariable(
        arg1: *mut root::nn::os::ConditionVariableType,
        arg2: *mut root::nn::os::MutexType,
        arg3: root::nn::TimeSpan,
    ) -> u8;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os12CreateThreadEPNS0_10ThreadTypeEPFvPvES3_S3_mii"]
    pub fn CreateThread(
        arg1: *mut root::nn::os::ThreadType,
        arg2: ::core::option::Option<unsafe extern "C" fn(arg1: *mut libc::c_void)>,
        arg: *mut libc::c_void,
        srcStack: *mut libc::c_void,
        stackSize: u64,
        priority: root::s32,
        coreNum: root::s32,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os12CreateThreadEPNS0_10ThreadTypeEPFvPvES3_S3_mi"]
    pub fn CreateThread1(
        arg1: *mut root::nn::os::ThreadType,
        arg2: unsafe extern "C" fn(arg1: *mut libc::c_void),
        arg: *mut libc::c_void,
        srcStack: *mut libc::c_void,
        stackSize: u64,
        priority: root::s32,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os13DestroyThreadEPNS0_10ThreadTypeE"]
    pub fn DestroyThread(arg1: *mut root::nn::os::ThreadType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os11StartThreadEPNS0_10ThreadTypeE"]
    pub fn StartThread(arg1: *mut root::nn::os::ThreadType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os13SetThreadNameEPNS0_10ThreadTypeEPKc"]
    pub fn SetThreadName(
        arg1: *mut root::nn::os::ThreadType,
        threadName: *const u8,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os20SetThreadNamePointerEPNS0_10ThreadTypeEPKc"]
    pub fn SetThreadNamePointer(
        arg1: *mut root::nn::os::ThreadType,
        arg2: *const u8,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os20GetThreadNamePointerEPKNS0_10ThreadTypeE"]
    pub fn GetThreadNamePointer(
        arg1: *const root::nn::os::ThreadType,
    ) -> *mut u8;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os16GetCurrentThreadEv"]
    pub fn GetCurrentThread() -> *mut root::nn::os::ThreadType;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os20ChangeThreadPriorityEPNS0_10ThreadTypeEi"]
    pub fn ChangeThreadPriority(
        thread: *mut root::nn::os::ThreadType,
        priority: root::s32,
    ) -> root::s32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os17GetThreadPriorityEPKNS0_10ThreadTypeE"]
    pub fn GetThreadPriority(thread: *const root::nn::os::ThreadType) -> root::s32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os11YieldThreadEv"]
    pub fn YieldThread();
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os13SuspendThreadEPNS0_10ThreadTypeE"]
    pub fn SuspendThread(arg1: *mut root::nn::os::ThreadType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os12ResumeThreadEPNS0_10ThreadTypeE"]
    pub fn ResumeThread(arg1: *mut root::nn::os::ThreadType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os11SleepThreadENS_8TimeSpanE"]
    pub fn SleepThread(arg1: root::nn::TimeSpan);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os10WaitThreadEPNS0_10ThreadTypeE"]
    pub fn WaitThread(arg1: *mut root::nn::os::ThreadType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os15InitializeEventEPNS0_9EventTypeEbNS0_14EventClearModeE"]
    pub fn InitializeEvent(
        arg1: *mut root::nn::os::EventType,
        initiallySignaled: bool,
        clearMode: root::nn::os::EventClearMode,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os13FinalizeEventEPNS0_9EventTypeE"]
    pub fn FinalizeEvent(arg1: *mut root::nn::os::EventType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os11SignalEventEPNS0_9EventTypeE"]
    pub fn SignalEvent(arg1: *mut root::nn::os::EventType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os9WaitEventEPNS0_9EventTypeE"]
    pub fn WaitEvent(arg1: *mut root::nn::os::EventType);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os12TryWaitEventEPNS0_9EventTypeE"]
    pub fn TryWaitEvent(arg1: *mut root::nn::os::EventType) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os14TimedWaitEventEPNS0_9EventTypeENS_8TimeSpanE"]
    pub fn TimedWaitEvent(
        arg1: *mut root::nn::os::EventType,
        arg2: root::nn::TimeSpan,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os10ClearEventEPNS0_9EventTypeE"]
    pub fn ClearEvent(arg1: *mut root::nn::os::EventType);
}
#[repr(C)]
pub struct CpuRegister {
    #[doc = "< 64-bit AArch64 register view."]
    pub x: root::__BindgenUnionField<u64>,
    #[doc = "< 32-bit AArch64 register view."]
    pub w: root::__BindgenUnionField<u32>,
    #[doc = "< AArch32 register view."]
    pub r: root::__BindgenUnionField<u32>,
    pub bindgen_union_field: u64,
}

#[doc = " Armv8 NEON register."]
#[repr(C)]
#[repr(align(16))]
pub struct FpuRegister {
    #[doc = "< 128-bit vector view."]
    pub v: root::__BindgenUnionField<u128>,
    #[doc = "< 64-bit double-precision view."]
    pub d: root::__BindgenUnionField<f64>,
    #[doc = "< 32-bit single-precision view."]
    pub s: root::__BindgenUnionField<f32>,
    pub bindgen_union_field: u128,
}

#[repr(C)]
#[repr(align(16))]
pub struct UserExceptionInfo {
    #[doc = "< See \\ref ThreadExceptionDesc."]
    pub ErrorDescription: u32,
    pub pad: [u32; 3usize],
    #[doc = "< GPRs 0..28. Note: also contains AArch32 registers."]
    pub CpuRegisters: [root::nn::os::CpuRegister; 29usize],
    #[doc = "< Frame pointer."]
    pub FP: root::nn::os::CpuRegister,
    #[doc = "< Link register."]
    pub LR: root::nn::os::CpuRegister,
    #[doc = "< Stack pointer."]
    pub SP: root::nn::os::CpuRegister,
    #[doc = "< Program counter (elr_el1)."]
    pub PC: root::nn::os::CpuRegister,
    pub padding: u64,
    #[doc = "< 32 general-purpose NEON registers."]
    pub FpuRegisters: [root::nn::os::FpuRegister; 32usize],
    #[doc = "< pstate & 0xFF0FFE20"]
    pub PState: u32,
    pub AFSR0: u32,
    pub AFSR1: u32,
    pub ESR: u32,
    #[doc = "< Fault Address Register."]
    pub FAR: root::nn::os::CpuRegister,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn2os23SetUserExceptionHandlerEPFvPNS0_17UserExceptionInfoEEPvmS2_"]
    #[allow(improper_ctypes)]
    pub fn SetUserExceptionHandler(
        arg1: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut root::nn::os::UserExceptionInfo),
        >,
        arg2: *mut libc::c_void,
        arg3: root::ulong,
        arg4: *mut root::nn::os::UserExceptionInfo,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os19GenerateRandomBytesEPvm"]
    pub fn GenerateRandomBytes(arg1: *mut libc::c_void, arg2: u64);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os13GetSystemTickEv"]
    pub fn GetSystemTick() -> root::nn::os::Tick;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2os26GetThreadAvailableCoreMaskEv"]
    pub fn GetThreadAvailableCoreMask() -> u64;
}