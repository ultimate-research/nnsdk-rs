#[allow(unused_imports)]
use self::super::root;
#[repr(C)]
pub struct PlayReport {
    pub m_EventName: [u8; 32usize],
    pub m_Buff: *const u8,
    pub m_BuffLength: usize,
    pub m_End: usize,
}

#[repr(C)]
pub struct Any64BitId {
    pub id: i64
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport10SetEventIdEPKc"]
    pub fn PlayReport_SetEventId(
        this: *mut PlayReport,
        arg1: *const u8,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport9SetBufferEv"]
    pub fn PlayReport_SetBuffer(this: *mut PlayReport)
        -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport3AddEPKcl"]
    pub fn PlayReport_Add(
        this: *mut PlayReport,
        key: *const u8,
        value: i64,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport3AddEPKcd"]
    pub fn PlayReport_Add1(
        this: *mut PlayReport,
        key: *const u8,
        value: f64,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport3AddEPKcS3_"]
    pub fn PlayReport_Add2(
        this: *mut PlayReport,
        key: *const u8,
        value: *const u8,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport3AddEPKcRKNS0_10Any64BitIdE"]
    pub fn PlayReport_Add3(
            this: *mut PlayReport,
            key: *const u8,
            value: Any64BitId,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport4SaveEv"]
    pub fn PlayReport_Save(this: *mut PlayReport) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport4SaveERKNS_7account3UidE"]
    pub fn PlayReport_Save1(
        this: *mut PlayReport,
        uid: *const root::nn::account::Uid,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReportC1Ev"]
    pub fn PlayReport_PlayReport(this: *mut PlayReport);
}
impl PlayReport {
    #[inline]
    pub fn SetEventId(&mut self, arg1: *const u8) -> root::Result {
        unsafe { PlayReport_SetEventId(self, arg1) }
    }
    #[inline]
    pub fn SetBuffer(&mut self) -> root::Result {
        unsafe { PlayReport_SetBuffer(self) }
    }
    #[inline]
    pub fn AddLong(
        &mut self,
        key: *const u8,
        value: i64,
    ) -> root::Result {
        unsafe { PlayReport_Add(self, key, value) }
    }
    #[inline]
    pub fn AddDouble(
        &mut self,
        key: *const u8,
        value: f64,
    ) -> root::Result {
        unsafe { PlayReport_Add1(self, key, value) }
    }
    #[inline]
    pub fn AddString(
        &mut self,
        key: *const u8,
        value: *const u8,
    ) -> root::Result {
        unsafe { PlayReport_Add2(self, key, value) }
    }
    #[inline]
    pub fn AddAny64BitID(
        &mut self,
        key: *const u8,
        value: Any64BitId,
    ) -> root::Result {
        unsafe { PlayReport_Add3(self, key, value) }
    }
    #[inline]
    pub fn Save(&mut self) -> root::Result {
        unsafe { PlayReport_Save(self) }
    }
    #[inline]
    pub fn SaveWithAccount(
        &mut self,
        uid: *const root::nn::account::Uid,
    ) -> root::Result {
        unsafe { PlayReport_Save1(self, uid) }
    }
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut __bindgen_tmp = ::core::mem::MaybeUninit::uninit();
            PlayReport_PlayReport(__bindgen_tmp.as_mut_ptr());
            __bindgen_tmp.assume_init()
        }
    }
}