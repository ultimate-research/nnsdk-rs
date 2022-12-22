use alloc::boxed::Box;
use alloc::string::{ToString, String};

#[allow(unused_imports)]
use self::super::root;
#[repr(C)]
#[derive(Debug)]
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

    #[link_name = "\u{1}_ZN2nn5prepo10PlayReport9SetBufferEPvm"]
    pub fn PlayReport_SetBuffer(this: *mut PlayReport, buf: *const u8, size: usize);

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

    #[link_name = "_ZN2nn5prepo10PlayReportC1EPKc"]
    pub fn PlayReport_PlayReportWithEventID(this: *mut PlayReport, event_id: *const u8);

    #[link_name = "\u{1}_ZNK2nn5prepo10PlayReport8GetCountEv"]
    pub fn PlayReport_GetCount(this: *mut PlayReport) -> u64;

    #[link_name = "_ZN2nn5prepo28RequestImmediateTransmissionEv"]
    pub fn RequestImmediateTransmission() -> root::Result;
}
impl PlayReport {
    #[inline]
    pub fn get_count(&mut self) -> u64 {
        unsafe { PlayReport_GetCount(self) }
    }
    #[inline]
    pub fn set_event_id(&mut self, event_id: &str) -> root::Result {
        let event_id = event_id.to_string() + "\0";
        if event_id.len() > EventIdLengthMax {
            panic!("Event ID is too long!");
        }
        let event_id = event_id.as_bytes().as_ptr();
        unsafe { PlayReport_SetEventId(self, event_id) }
    }
    #[inline]
    fn set_buffer(&mut self, buf: *const u8, size: usize) {
        unsafe { PlayReport_SetBuffer(self, buf, size) }
    }
    #[inline]
    pub fn add_long(
        &mut self,
        key: &str,
        value: i64,
    ) -> root::Result {
        let key = key.to_string() + "\0";
        if key.len() > KeyLengthMax {
            panic!("Key is too long!");
        }
        let key = key.as_bytes().as_ptr();
        unsafe { PlayReport_AddLong(self, key, value) }
    }
    #[inline]
    pub fn add_double(
        &mut self,
        key: &str,
        value: f64,
    ) -> root::Result {
        let key = key.to_string() + "\0";
        if key.len() > KeyLengthMax {
            panic!("Key is too long!");
        }
        let key = key.as_bytes().as_ptr();
        unsafe { PlayReport_AddDouble(self, key, value) }
    }
    #[inline]
    pub fn add_string(
        &mut self,
        key: &str,
        value: &str,
    ) -> root::Result {
        let key = key.to_string() + "\0";
        if key.len() > KeyLengthMax {
            panic!("Key is too long!");
        }
        let key = key.as_bytes().as_ptr();

        let value = value.to_string() + "\0";
        let value = value.as_bytes().as_ptr();
        unsafe { PlayReport_AddString(self, key, value) }
    }
    #[inline]
    pub fn add_any64bitid(
        &mut self,
        key: &str,
        value: Any64BitId,
    ) -> root::Result {
        let key = key.to_string() + "\0";
        if key.len() > KeyLengthMax {
            panic!("Key is too long!");
        }
        let key = key.as_bytes().as_ptr();
        unsafe { PlayReport_AddAny64BitID(self, key, value) }
    }
    #[inline]
    pub fn save(self) -> root::Result {
        unsafe { PlayReport_Save(self) }
    }
    #[inline]
    pub fn save_with_user_id(
        self,
        uid: *const root::nn::account::Uid,
    ) -> root::Result {
        unsafe { PlayReport_SaveWithUserId(self, uid) }
    }
    #[inline]
    pub fn save_for_current_user(self) {
        // This provides a UserHandle and sets the User in a Open state to be used.
        let handle = root::nn::account::try_open_preselected_user().expect("OpenPreselectedUser should not return false");
        // Obtain the UID for this user
        let uid = root::nn::account::get_user_id(&handle).expect("GetUserId should return a valid Uid");
        self.save_with_user_id(&uid);
        root::nn::account::close_user(handle);
    }
    #[inline]
    pub fn new() -> Self {
        let buf = Box::new([0u8; 0x4000]);
        let buf = Box::leak(buf);
        let buf = Box::new([0u8; 0x4000]);
        let buf = Box::leak(buf);
        let mut prepo: PlayReport = PlayReport { event_id: [0;32], buffer: core::ptr::null(), size: 0, position: 0 };
        unsafe { PlayReport_PlayReport(&mut prepo) };

        prepo.set_buffer(buf.as_ptr(), 0x4000);
        prepo
    }
    #[inline]
    pub fn new_with_event_id(event_id: &str) -> Self {
        let event_id = event_id.to_string() + "\0";
        if event_id.len() > EventIdLengthMax {
            panic!("Event ID is too long!");
        }
        let event_id = event_id.as_bytes().as_ptr();

        let buf = Box::new([0u8; 0x4000]);
        let buf = Box::leak(buf);
        let mut prepo: PlayReport = PlayReport { event_id: [0;32], buffer: core::ptr::null(), size: 0, position: 0 };
        unsafe { PlayReport_PlayReportWithEventID(&mut prepo, event_id) };

        prepo.set_buffer(buf.as_ptr(), 0x4000);
        prepo
    }
}

impl Drop for PlayReport {
    fn drop(&mut self) {
        unsafe { libc::free(self.buffer as *mut libc::c_void); }
    }
}