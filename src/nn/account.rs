#[allow(unused_imports)]
use self::super::root;

//pub type Nickname = [u8; 33usize];
#[repr(transparent)]
pub struct Nickname(pub [u8; 33usize]);

impl Nickname {
    pub fn new() -> Self {
        Self([0; 33])
    }
}

#[cfg(not(feature = "rustc-dep-of-std"))]
impl core::fmt::Display for Nickname {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut i = 0;
        while self.0[i] != 0 {
            i += 1;
        }

        write!(f, "{}", unsafe {
            alloc::str::from_utf8_unchecked(&self.0[..i])
        })
    }
}

pub type NetworkServiceAccountId = u64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uid {
    id: [u64; 2usize],
}

impl Uid {
    pub fn new() -> Self {
        Self { id: [0, 0] }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct UserHandle {
    pub id: [u64; 3usize],
}

impl UserHandle {
    pub fn new() -> Self {
        Self { id: [0u64; 3] }
    }
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn7account10InitializeEv"]
    pub fn Initialize();
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn7account15ShowUserCreatorEv"]
    pub fn ShowUserCreator();
}

pub mod detail {
    extern "C" {
        #[link_name = "\u{1}_ZN2nn7account6detail13IsInitializedEv"]
        pub fn IsInitialized() -> bool;
    }
}

pub fn try_open_preselected_user() -> Option<UserHandle> {
    let mut handle = UserHandle::new();

    unsafe {
        TryOpenPreselectedUser(&mut handle).then(|| handle)
    }
}

pub fn get_user_id(user_handle: &UserHandle) -> Result<Uid, root::Result> {
    let mut uid = Uid::new();

    unsafe {
        let result = GetUserId(&mut uid, user_handle);
        
        if result != 0 {
            Err(result)
        } else {
            Ok(uid)
        }
    }
}

pub fn close_user(user_handle: UserHandle) {
    unsafe { CloseUser(&user_handle) }
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn7account12ListAllUsersEPiPNS0_3UidEi"]
    pub fn ListAllUsers(
        arg1: *mut root::s32,
        arg2: *mut Uid,
        numUsers: root::s32,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account8OpenUserEPNS0_10UserHandleERKNS0_3UidE"]
    pub fn OpenUser(
        arg1: *mut UserHandle,
        arg2: *const Uid,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account19OpenPreselectedUserEPNS0_10UserHandleE"]
    pub fn OpenPreselectedUser(
        userHandle: *mut UserHandle
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account22TryOpenPreselectedUserEPNS0_10UserHandleE"]
    pub fn TryOpenPreselectedUser(
        userHandle: *mut UserHandle
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account9GetUserIdEPNS0_3UidERKNS0_10UserHandleE"]
    pub fn GetUserId(
        out_user_id: *mut Uid,
        handle: *const UserHandle
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account32IsNetworkServiceAccountAvailableEPbRKNS0_10UserHandleE"]
    pub fn IsNetworkServiceAccountAvailable(
        out: *mut bool,
        arg1: *const UserHandle,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account9CloseUserERKNS0_10UserHandleE"]
    pub fn CloseUser(arg1: *const UserHandle);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account36EnsureNetworkServiceAccountAvailableERKNS0_10UserHandleE"]
    pub fn EnsureNetworkServiceAccountAvailable(
        userHandle: *const UserHandle,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account44EnsureNetworkServiceAccountIdTokenCacheAsyncEPNS0_12AsyncContextERKNS0_10UserHandleE"]
    pub fn EnsureNetworkServiceAccountIdTokenCacheAsync(
        arg1: *mut AsyncContext,
        arg2: *const UserHandle,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account37LoadNetworkServiceAccountIdTokenCacheEPmPcmRKNS0_10UserHandleE"]
    pub fn LoadNetworkServiceAccountIdTokenCache(
        arg1: *mut u64,
        arg2: *mut u8,
        arg3: u64,
        arg4: *const UserHandle,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account17GetLastOpenedUserEPNS0_3UidE"]
    pub fn GetLastOpenedUser(arg1: *mut Uid) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account11GetNicknameEPNS0_8NicknameERKNS0_3UidE"]
    pub fn GetNickname(
        nickname: *mut Nickname,
        userID: *const Uid,
    ) -> root::Result;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsyncContext {
    pub _address: u8,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn7account12AsyncContext7HasDoneEPb"]
    pub fn AsyncContext_HasDone(
        this: *mut AsyncContext,
        arg1: *mut bool,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account12AsyncContext9GetResultEv"]
    pub fn AsyncContext_GetResult(
        this: *mut AsyncContext,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account12AsyncContext6CancelEv"]
    pub fn AsyncContext_Cancel(
        this: *mut AsyncContext,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account12AsyncContext14GetSystemEventEPNS_2os11SystemEventE"]
    pub fn AsyncContext_GetSystemEvent(
        this: *mut AsyncContext,
        arg1: *mut root::nn::os::SystemEvent,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7account12AsyncContextC1Ev"]
    pub fn AsyncContext_AsyncContext(this: *mut AsyncContext);
}
impl AsyncContext {
    #[inline]
    pub unsafe fn HasDone(&mut self, arg1: *mut bool) -> root::Result {
        AsyncContext_HasDone(self, arg1)
    }
    #[inline]
    pub unsafe fn GetResult(&mut self) -> root::Result {
        AsyncContext_GetResult(self)
    }
    #[inline]
    pub unsafe fn Cancel(&mut self) -> root::Result {
        AsyncContext_Cancel(self)
    }
    #[inline]
    pub unsafe fn GetSystemEvent(
        &mut self,
        arg1: *mut root::nn::os::SystemEvent,
    ) -> root::Result {
        AsyncContext_GetSystemEvent(self, arg1)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::core::mem::MaybeUninit::uninit();
        AsyncContext_AsyncContext(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}