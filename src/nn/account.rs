use alloc::{vec::Vec,vec, string::String};

#[allow(unused_imports)]
use self::super::root;

pub mod detail;
//pub type Nickname = [u8; 33usize];
#[repr(transparent)]
pub struct Nickname(pub [u8; 33usize]);

impl Nickname {
    pub fn new() -> Self {
        Self([0; 33])
    }
}

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
    pub id: [u64; 2usize],
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

    #[link_name = "_ZN2nn7account8FinalizeEv"]
    pub fn Finalize();

    #[link_name = "\u{1}_ZN2nn7account15ShowUserCreatorEv"]
    pub fn ShowUserCreator();

    #[link_name = "\u{1}_ZN2nn7account12ListAllUsersEPiPNS0_3UidEi"]
    pub fn ListAllUsers(
        out_len: *mut root::s32,
        out_uids: *mut Uid,
        num_users: root::s32,
    ) -> root::Result;

    #[link_name = "_ZN2nn7account12GetUserCountEPi"]
    pub fn GetUserCount(count: *mut i32);

    #[link_name = "\u{1}_ZN2nn7account8OpenUserEPNS0_10UserHandleERKNS0_3UidE"]
    pub fn OpenUser(
        handle: *mut UserHandle,
        uid: *const Uid,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn7account19OpenPreselectedUserEPNS0_10UserHandleE"]
    pub fn OpenPreselectedUser(
        handle: *mut UserHandle
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn7account22TryOpenPreselectedUserEPNS0_10UserHandleE"]
    pub fn TryOpenPreselectedUser(
        handle: *mut UserHandle
    ) -> bool;

    #[link_name = "\u{1}_ZN2nn7account9GetUserIdEPNS0_3UidERKNS0_10UserHandleE"]
    pub fn GetUserId(
        out_user_id: *mut Uid,
        handle: *const UserHandle
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn7account32IsNetworkServiceAccountAvailableEPbRKNS0_10UserHandleE"]
    pub fn IsNetworkServiceAccountAvailable(
        out: *mut bool,
        arg1: *const UserHandle,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn7account9CloseUserERKNS0_10UserHandleE"]
    pub fn CloseUser(handle: *const UserHandle);

    #[link_name = "\u{1}_ZN2nn7account36EnsureNetworkServiceAccountAvailableERKNS0_10UserHandleE"]
    pub fn EnsureNetworkServiceAccountAvailable(
        handle: *const UserHandle,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn7account44EnsureNetworkServiceAccountIdTokenCacheAsyncEPNS0_12AsyncContextERKNS0_10UserHandleE"]
    pub fn EnsureNetworkServiceAccountIdTokenCacheAsync(
        context: *mut AsyncContext,
        handle: *const UserHandle,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn7account37LoadNetworkServiceAccountIdTokenCacheEPmPcmRKNS0_10UserHandleE"]
    pub fn LoadNetworkServiceAccountIdTokenCache(
        arg1: *mut u64,
        arg2: *mut u8,
        arg3: u64,
        arg4: *const UserHandle,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn7account17GetLastOpenedUserEPNS0_3UidE"]
    pub fn GetLastOpenedUser(arg1: *mut Uid) -> root::Result;

    #[link_name = "\u{1}_ZN2nn7account11GetNicknameEPNS0_8NicknameERKNS0_3UidE"]
    pub fn GetNickname(
        nickname: *mut Nickname,
        userID: *const Uid,
    ) -> root::Result;
}

pub fn initialize() {
    unsafe { Initialize() };
}

pub fn finalize() {
    unsafe { Finalize() };
}

pub fn show_user_creator() {
    unsafe { ShowUserCreator() };
}

pub fn list_all_users() -> Vec<Uid> {
    let num_users = get_user_count();

    let mut out_len = 0;
    let mut out_uids: Vec<Uid> = vec![Uid::new(); num_users as usize];

    unsafe {
        ListAllUsers(&mut out_len, out_uids.as_mut_ptr(), num_users);
    }

    return out_uids;
}

pub fn get_user_count() -> i32 {
    let mut count = 0;

    unsafe {
        GetUserCount(&mut count);
    }

    return count
}

pub fn open_user(uid: &Uid) -> UserHandle {
    let mut handle = UserHandle::new();

    unsafe {
        OpenUser(&mut handle, uid);
    }
    return handle;
}

pub fn open_preselected_user() -> UserHandle {
    let mut handle = UserHandle::new();
    unsafe {
        OpenPreselectedUser(&mut handle);
    }
    return handle;
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

pub fn is_network_service_account_available(handle: &UserHandle) -> bool {
    let mut result = true;

    unsafe {
        IsNetworkServiceAccountAvailable(&mut result, handle);
    }

    return result;
}

pub fn close_user(user_handle: UserHandle) {
    unsafe { CloseUser(&user_handle) }
}

pub fn ensure_network_service_account_available(handle: &UserHandle) {
    unsafe {
        EnsureNetworkServiceAccountAvailable(handle);
    }
}

pub fn ensure_network_service_account_id_token_cache_async(context: &mut AsyncContext, handle: &UserHandle) {
    unsafe {
        EnsureNetworkServiceAccountIdTokenCacheAsync(context, handle);
    }
}

pub fn get_last_opened_user() -> Uid {
    let mut uid = Uid::new();

    unsafe {
        GetLastOpenedUser(&mut uid);
    }

    return uid;
}

pub fn get_nickname(uid: &Uid) -> String {
    let mut nickname = Nickname::new();

    unsafe {
        GetNickname(&mut nickname, uid);
    }

    let result = String::from_utf8(nickname.0.to_vec()).unwrap();

    return result.replace("\0", "");
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
        out_event: *mut root::nn::os::SystemEvent,
    ) -> root::Result;
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn7account12AsyncContextC1Ev"]
    pub fn AsyncContext_AsyncContext(this: *mut AsyncContext);
}

impl AsyncContext {
    #[inline]
    pub fn has_done(&mut self) -> bool {
        let mut result = false;
        unsafe {
            AsyncContext_HasDone(self, &mut result);
        }
        return result
    }

    #[inline]
    pub fn get_result(&mut self) -> root::Result {
        unsafe {
            return AsyncContext_GetResult(self);
        }
    }
    #[inline]
    pub fn cancel(&mut self) {
        unsafe {
            AsyncContext_Cancel(self);
        }
    }

    #[inline]
    pub fn get_system_event(
        &mut self,
    ) -> root::nn::os::SystemEvent {
        let mut event = root::nn::os::SystemEvent { _unused: [0; 0x28] } ;
        unsafe {
            AsyncContext_GetSystemEvent(self, &mut event);
        }
        return event;
    }

    #[inline]
    pub fn new() -> Self {
        let mut async_context = AsyncContext { _address: 0 };

        unsafe {
            AsyncContext_AsyncContext( &mut async_context );
        }
        return async_context;
    }
}
