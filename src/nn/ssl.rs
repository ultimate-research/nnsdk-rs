#[allow(unused_imports)]
use self::super::root;

extern "C" {
    #[link_name = "\u{1}_ZN2nn3ssl10InitializeEv"]
    pub fn Initialize() -> root::Result;

    #[link_name = "\u{1}_ZN2nn3ssl8FinalizeEv"]
    pub fn Finalize() -> root::Result;
}
pub mod Context {

    pub enum CertificateFormat {
        PEM = 0x1,
        DER = 0x2,
    }

    #[repr(C)]
    pub enum SslVersion {
        Auto = 0x1,
        Tls10 = 0x8,
        Tls11 = 0x10,
        Tls12 = 0x20,
    }

    // Estimate through SDK binary usage of the fields
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Context {
        _x: u64,
    }

    impl Context {
        pub fn new() -> Self {
            Self {
                _x: 0,
            }
        }
    }

    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl7ContextC1Ev"]
        pub fn Context(this: *mut Context);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl7Context6CreateENS1_10SslVersionE"]
        pub fn Create(this: *mut Context, version: SslVersion) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl7Context15ImportClientPkiEPmPKcS4_jj"]
        pub fn ImportClientPki(this: *mut Context, out_store_id: &mut u64, p12_buf: *const u8, password_buf: *const u8, p12_buf_len: u32, password_buf_len: u32) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl7Context15ImportServerPkiEPmPKcjNS0_17CertificateFormatE"]
        pub fn ImportServerPki(
            this: *mut Context,
            arg1: *mut u64,
            certData: *const u8,
            certSize: u32,
            certFormat: CertificateFormat,
        ) -> super::root::Result;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl7Context9ImportCrlEPmPKcj"]
        pub fn ImportCrl(this: *mut Context, out_store_id: &mut u64, crl_der_buf: *const u8, crl_der_buf_len: u32) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl7Context7DestroyEv"]
        pub fn Destroy(this: *const Context) -> u32;
    }
}
mod Connection {
    // Estimate through SDK binary usage of the fields. Might be 0x28?
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Connection {
        _x: [u8;0x24],
    }

    impl Connection {
        pub fn new() -> Self {
            Self {
                _x: [0;0x24],
            }
        }
    }

    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10ConnectionC1Ev"]
        pub fn Connection(this: *mut Connection);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection6CreateEPNS0_7ContextE"]
        pub fn Create(this: *mut Connection, context: *const super::Context::Context) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection19SetSocketDescriptorEi"]
        pub fn SetSocketDescriptor(this: *mut Connection, socket_desc: u32) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection11SetHostNameEPKcj"]
        pub fn SetHostName(this: *mut Connection, host_name: *const u8, name_len: u32) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection11DoHandshakeEv"]
        pub fn DoHandshake(this: *mut Connection) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection4ReadEPcj"]
        pub fn Read(this: *const Connection, out_buf: *mut u8, buf_len: usize) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection4ReadEPcPij"]
        pub fn Read1(this: *const Connection, out_buf: *mut u8, out_size_read: *mut i32, buf_len: usize) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection5WriteEPKcj"]
        pub fn Write(this: *const Connection, buf: *const u8, buf_len: usize) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection5WriteEPKcPij"]
        pub fn Write1(this: *const Connection, buf: *const u8, out_size_write: *mut i32, buf_len: usize) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection7PendingEv"]
        pub fn Pending(this: *const Connection) -> usize;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection17FlushSessionCacheEv"]
        pub fn FlushSessionCache(this: *mut Connection) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection9SetOptionENS1_10OptionTypeEb"]
        pub fn SetOption(this: *mut Connection, option: u32, enable: bool) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection15SetVerifyOptionENS1_12VerifyOptionE"]
        pub fn SetVerifyOption(this: *mut Connection, options: u32) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection12GetLastErrorEPNS_6ResultE"]
        pub fn GetLastError(this: *const Connection, out_result: *mut u32) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection7DestroyEv"]
        pub fn Destroy(this: *const Connection) -> u32;
    }
}
