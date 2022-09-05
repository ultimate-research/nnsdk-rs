#[allow(unused_imports)]
use self::super::root;
pub type Url = [libc::c_char; 160usize];
extern "C" {
    #[link_name = "\u{1}_ZN2nn7friends10InitializeEv"]
    pub fn Initialize();
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7friends14GetProfileListEPNS0_12AsyncContextEPNS0_7ProfileERKNS_7account3UidEPKmi"]
    pub fn GetProfileList(
        context: *mut AsyncContext,
        profiles: *mut Profile,
        userID: *const root::nn::account::Uid,
        accountIDs: *const root::nn::account::NetworkServiceAccountId,
        numAccounts: root::s32,
    ) -> root::Result;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Profile {
    pub _address: u8,
}

extern "C" {
    #[link_name = "\u{1}_ZNK2nn7friends7Profile12GetAccountIdEv"]
    pub fn Profile_GetAccountId(
        this: *const Profile,
    ) -> root::nn::account::NetworkServiceAccountId;
}
extern "C" {
    #[link_name = "\u{1}_ZNK2nn7friends7Profile11GetNicknameEv"]
    pub fn Profile_GetNickname(
        this: *const Profile,
    ) -> *mut root::nn::account::Nickname;
}
extern "C" {
    #[link_name = "\u{1}_ZNK2nn7friends7Profile7IsValidEv"]
    pub fn Profile_IsValid(this: *const Profile) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7friends7Profile18GetProfileImageUrlEPA160_ci"]
    pub fn Profile_GetProfileImageUrl(
        this: *mut Profile,
        arg1: *mut Url,
        arg2: root::s32,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7friends7ProfileC1Ev"]
    pub fn Profile_Profile(this: *mut Profile);
}
impl Profile {
    #[inline]
    pub unsafe fn GetAccountId(&self) -> root::nn::account::NetworkServiceAccountId {
        Profile_GetAccountId(self)
    }
    #[inline]
    pub unsafe fn GetNickname(&self) -> *mut root::nn::account::Nickname {
        Profile_GetNickname(self)
    }
    #[inline]
    pub unsafe fn IsValid(&self) -> bool {
        Profile_IsValid(self)
    }
    #[inline]
    pub unsafe fn GetProfileImageUrl(
        &mut self,
        arg1: *mut Url,
        arg2: root::s32,
    ) -> root::Result {
        Profile_GetProfileImageUrl(self, arg1, arg2)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::core::mem::MaybeUninit::uninit();
        Profile_Profile(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct AsyncContext {
    pub _address: u8,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn7friends12AsyncContext14GetSystemEventEPNS_2os11SystemEventE"]
    pub fn AsyncContext_GetSystemEvent(
        this: *mut AsyncContext,
        arg1: *mut root::nn::os::SystemEvent,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZNK2nn7friends12AsyncContext9GetResultEv"]
    pub fn AsyncContext_GetResult(
        this: *const AsyncContext,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7friends12AsyncContextC1Ev"]
    pub fn AsyncContext_AsyncContext(this: *mut AsyncContext);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn7friends12AsyncContextD1Ev"]
    pub fn AsyncContext_AsyncContext_destructor(
        this: *mut AsyncContext,
    );
}
impl AsyncContext {
    #[inline]
    pub unsafe fn GetSystemEvent(
        &mut self,
        arg1: *mut root::nn::os::SystemEvent,
    ) -> root::Result {
        AsyncContext_GetSystemEvent(self, arg1)
    }
    #[inline]
    pub unsafe fn GetResult(&self) -> root::Result {
        AsyncContext_GetResult(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::core::mem::MaybeUninit::uninit();
        AsyncContext_AsyncContext(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        AsyncContext_AsyncContext_destructor(self)
    }
}
