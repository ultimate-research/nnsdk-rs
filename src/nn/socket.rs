#[allow(unused_imports)]
use self::super::root;

#[repr(C)]
pub struct InAddr {
    pub addr: u32,
}

#[repr(C)]
pub struct SockAddrIn {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: [u8;4],
    pub padding: u64,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket10InitializeEPvmmi"]
    pub fn Initialize(
        pool: *mut u8,
        poolSize: root::ulong,
        allocPoolSize: root::ulong,
        concurLimit: i32,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket10InitializeERKNS0_6ConfigE"]
    pub fn Initialize_Config(config: *mut u8) -> root::Result;
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
        arg1: *const u8,
        len: u32,
    ) -> root::s32;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket4SendEiPKvmi"]
    pub fn Send(
        socket: root::s32,
        buffer: *const u8,
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
    #[link_name = "\u{1}_ZN2nn6socket7ConnectEiPKNS0_8SockAddrEj"]
    pub fn Connect(socket: i32, sockaddrin: *const SockAddrIn, sockaddr_len: u32) -> i32;
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn6socket12GetLastErrorEv"]
    pub fn GetLastError() -> u32;
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

