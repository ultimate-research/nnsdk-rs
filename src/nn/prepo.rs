#[allow(unused_imports)]
use self::super::root;
#[repr(C)]
pub struct PlayReport {
    pub m_EventName: [u8; 32usize],
    pub m_Buff: *mut libc::c_void,
    pub m_BuffLength: root::size_t,
    pub m_End: u64,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport10SetEventIdEPKc"]
    pub fn PlayReport_SetEventId(
        this: *mut PlayReport,
        arg1: *const u8,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport9SetBufferEv"]
    pub fn PlayReport_SetBuffer(this: *mut PlayReport)
        -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport3AddEPKcl"]
    pub fn PlayReport_Add(
        this: *mut PlayReport,
        arg1: *const u8,
        arg2: i64,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport3AddEPKcd"]
    pub fn PlayReport_Add1(
        this: *mut PlayReport,
        arg1: *const u8,
        arg2: f64,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport3AddEPKcS3_"]
    pub fn PlayReport_Add2(
        this: *mut PlayReport,
        arg1: *const u8,
        arg2: *const u8,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport4SaveEv"]
    pub fn PlayReport_Save(this: *mut PlayReport) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport4SaveERKNS_7account3UidE"]
    pub fn PlayReport_Save1(
        this: *mut PlayReport,
        arg1: *const root::nn::account::Uid,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn5prepo10PlayReportC1Ev"]
    pub fn PlayReport_PlayReport(this: *mut PlayReport);
}
impl PlayReport {
    #[inline]
    pub unsafe fn SetEventId(&mut self, arg1: *const u8) -> root::Result {
        PlayReport_SetEventId(self, arg1)
    }
    #[inline]
    pub unsafe fn SetBuffer(&mut self) -> root::Result {
        PlayReport_SetBuffer(self)
    }
    #[inline]
    pub unsafe fn Add(
        &mut self,
        arg1: *const u8,
        arg2: i64,
    ) -> root::Result {
        PlayReport_Add(self, arg1, arg2)
    }
    #[inline]
    pub unsafe fn Add1(
        &mut self,
        arg1: *const u8,
        arg2: f64,
    ) -> root::Result {
        PlayReport_Add1(self, arg1, arg2)
    }
    #[inline]
    pub unsafe fn Add2(
        &mut self,
        arg1: *const u8,
        arg2: *const u8,
    ) -> root::Result {
        PlayReport_Add2(self, arg1, arg2)
    }
    #[inline]
    pub unsafe fn Save(&mut self) -> root::Result {
        PlayReport_Save(self)
    }
    #[inline]
    pub unsafe fn Save1(
        &mut self,
        arg1: *const root::nn::account::Uid,
    ) -> root::Result {
        PlayReport_Save1(self, arg1)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::core::mem::MaybeUninit::uninit();
        PlayReport_PlayReport(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}