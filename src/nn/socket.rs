#[allow(unused_imports)]
use self::super::root;

#[repr(C)]
pub struct InAddr {
    pub addr: u32,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket10InitializeEPvmmi"]
    pub fn Initialize(
        pool: *mut libc::c_void,
        poolSize: root::ulong,
        allocPoolSize: root::ulong,
        concurLimit: i32,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket10InitializeERKNS0_6ConfigE"]
    pub fn Initialize_Config(config: *mut libc::c_void) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket8FinalizeEv"]
    pub fn Finalize() -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket10SetSockOptEiiiPKvj"]
    pub fn SetSockOpt(
        socket: root::s32,
        socketLevel: root::s32,
        option: root::s32,
        arg1: *const libc::c_void,
        len: u32,
    ) -> root::s32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket4SendEiPKvmi"]
    pub fn Send(
        socket: root::s32,
        buffer: *const libc::c_void,
        bufferLength: u64,
        flags: root::s32,
    ) -> u64;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket6SocketEiii"]
    pub fn Socket(domain: root::s32, type_: root::s32, proto: root::s32) -> root::s32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket9InetHtonsEt"]
    pub fn InetHtons(arg1: u16) -> u16;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket8InetAtonEPKcPNS0_6InAddrE"]
    pub fn InetAton(
        str: *const u8,
        arg1: *mut root::nn::socket::InAddr,
    ) -> u32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket7ConnectEiPK8sockaddrj"]
    pub fn Connect(socket: root::s32, addr: *const root::sockaddr, addrLen: u32)
        -> u32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket4BindEiPK8sockaddrj"]
    pub fn Bind(socket: root::s32, addr: *const root::sockaddr, addrLen: u32) -> u32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket6ListenEii"]
    pub fn Listen(socket: root::s32, backlog: root::s32) -> u32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket6AcceptEiP8sockaddrPj"]
    pub fn Accept(
        socket: root::s32,
        addrOut: *mut root::sockaddr,
        addrLenOut: *mut u32,
    ) -> u32;
}