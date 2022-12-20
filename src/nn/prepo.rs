use alloc::boxed::Box;
use alloc::string::ToString;

#[allow(unused_imports)]
use self::super::root;
#[repr(C)]
pub struct PlayReport {
    pub event_id: [u8;32],
    pub buffer: *const u8,
    pub size: usize,
    pub position: usize
}

#[repr(C)]
pub struct Any64BitId {
    pub id: i64
}

const EventIdLengthMax: usize = 31;
const KeyLengthMax: usize = 63;

extern "C" {
    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport10SetEventIdEPKc"]
    pub fn PlayReport_SetEventId(
        this: *mut PlayReport,
        event_id: *const u8,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport9SetBufferEv"]
    pub fn PlayReport_SetBuffer(this: *mut PlayReport)
        -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport3AddEPKcl"]
    pub fn PlayReport_AddLong(
        this: *mut PlayReport,
        key: *const u8,
        value: i64,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport3AddEPKcd"]
    pub fn PlayReport_AddDouble(
        this: *mut PlayReport,
        key: *const u8,
        value: f64,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport3AddEPKcS3_"]
    pub fn PlayReport_AddString(
        this: *mut PlayReport,
        key: *const u8,
        value: *const u8,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport3AddEPKcRKNS0_10Any64BitIdE"]
    pub fn PlayReport_AddAny64BitID(
            this: *mut PlayReport,
            key: *const u8,
            value: Any64BitId,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport4SaveEv"]
    pub fn PlayReport_Save(this: PlayReport) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport4SaveERKNS_7account3UidE"]
    pub fn PlayReport_SaveWithUserId(
        this: PlayReport,
        uid: *const root::nn::account::Uid,
    ) -> root::Result;

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReportC1Ev"]
    pub fn PlayReport_PlayReport(this: *mut PlayReport);
}
impl PlayReport {
    #[inline]
    pub fn SetEventId(&mut self, event_id: &str) -> root::Result {
        event_id = (event_id.to_string() + "/0").as_str();
        if event_id.len() > EventIdLengthMax {
            panic!("Event ID is too long!");
        }
        let event_id = event_id.as_bytes().as_ptr();
        unsafe { PlayReport_SetEventId(self, event_id) }
    }
    #[inline]
    pub fn SetBuffer(&mut self) -> root::Result {
        unsafe { PlayReport_SetBuffer(self) }
    }
    #[inline]
    pub fn AddLong(
        &mut self,
        key: &str,
        value: i64,
    ) -> root::Result {
        key = (key.to_string() + "/0").as_str();
        if key.len() > KeyLengthMax {
            panic!("Key is too long!");
        }
        let key = key.as_bytes().as_ptr();
        unsafe { PlayReport_AddLong(self, key, value) }
    }
    #[inline]
    pub fn AddDouble(
        &mut self,
        key: &str,
        value: f64,
    ) -> root::Result {
        key = (key.to_string() + "/0").as_str();
        if key.len() > KeyLengthMax {
            panic!("Key is too long!");
        }
        let key = key.as_bytes().as_ptr();
        unsafe { PlayReport_AddDouble(self, key, value) }
    }
    #[inline]
    pub fn AddString(
        &mut self,
        key: &str,
        value: &str,
    ) -> root::Result {
        key = (key.to_string() + "/0").as_str();
        if key.len() > KeyLengthMax {
            panic!("Key is too long!");
        }
        let key = key.as_bytes().as_ptr();

        value = (value.to_string() + "/0").as_str();
        let value = value.as_bytes().as_ptr();
        unsafe { PlayReport_AddString(self, key, value) }
    }
    #[inline]
    pub fn AddAny64BitID(
        &mut self,
        key: &str,
        value: Any64BitId,
    ) -> root::Result {
        key = (key.to_string() + "/0").as_str();
        if key.len() > KeyLengthMax {
            panic!("Key is too long!");
        }
        let key = key.as_bytes().as_ptr();
        unsafe { PlayReport_AddAny64BitID(self, key, value) }
    }
    #[inline]
    pub fn Save(self) -> root::Result {
        unsafe { PlayReport_Save(self) }
    }
    #[inline]
    pub fn SaveWithUserId(
        self,
        uid: *const root::nn::account::Uid,
    ) -> root::Result {
        unsafe { PlayReport_SaveWithUserId(self, uid) }
    }
    #[inline]
    pub fn new() -> Self {
        let buf = Box::new([0u8; 0x4000]);
        Box::leak(buf);
        let prepo = PlayReport { event_id: [0;32], buffer: buf.as_ptr(), size: 0x4000, position: 0 };

        prepo
    }
}

impl Drop for PlayReport {
    fn drop(&mut self) {
        unsafe { libc::free(self.buffer as *mut libc::c_void); }
    }
}