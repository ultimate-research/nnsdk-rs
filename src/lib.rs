#![no_std]
#![feature(repr_simd)]
#![feature(new_uninit)]
#![feature(new_zeroed_alloc)]

#[cfg(not(feature = "rustc-dep-of-std"))]
extern crate alloc;

pub mod extensions;
pub mod common_types;
pub mod nn;
pub mod rtld;

#[doc(inline)]
pub use root::nn::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[doc(hidden)]
pub mod root {
    pub use crate::common_types::*;
    #[allow(unused_imports)]
    pub use super::nn;
    pub use super::rtld;
    use self::super::root;

    extern "C" {
        pub fn main(argc: i32, argv: *mut *mut u8) -> i32;
    }
    extern "C" {
        pub fn nninitStartup();
    }
    extern "C" {
        pub fn _init();
    }
    extern "C" {
        pub fn _fini();
    }
    extern "C" {
        pub fn __nnDetailNintendoSdkRuntimeObjectFileRefer();
    }
    extern "C" {
        pub fn __nnDetailNintendoSdkRuntimeObjectFile();
    }
    extern "C" {
        pub fn __nnDetailNintendoSdkNsoFileRefer();
    }
    extern "C" {
        pub fn __nnmusl_init_dso_0();
    }
    extern "C" {
        pub fn __nnmusl_fini_dso_0();
    }
    extern "C" {
        pub fn __nnDetailNintendoSdkNsoFile_0();
    }
    extern "C" {
        pub fn nnosInitializeMutex(arg1: *mut root::nnosMutexType, arg2: bool, arg3: root::s32);
    }
    extern "C" {
        pub fn nnosFinalizeMutex(arg1: *mut root::nnosMutexType);
    }
    extern "C" {
        pub fn nnosLockMutex(arg1: *mut root::nnosMutexType);
    }
    extern "C" {
        pub fn nnosTryLockMutex(arg1: *mut root::nnosMutexType) -> bool;
    }
    extern "C" {
        pub fn nnosUnlockMutex(arg1: *mut root::nnosMutexType);
    }
    extern "C" {
        pub fn llabs(n: i64) -> i64;
    }
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const u8,
            __file: *const u8,
            __line: u32,
            __function: *const u8,
        );
    }
    extern "C" {
        pub fn __assert_perror_fail(
            __errnum: i32,
            __file: *const u8,
            __line: u32,
            __function: *const u8,
        );
    }
    extern "C" {
        pub fn __assert(
            __assertion: *const u8,
            __file: *const u8,
            __line: i32,
        );
    }

    extern "C" {
        pub fn select(
            __nfds: i32,
            __readfds: *mut root::fd_set,
            __writefds: *mut root::fd_set,
            __exceptfds: *mut root::fd_set,
            __timeout: *mut root::timeval,
        ) -> i32;
    }
    extern "C" {
        pub fn pselect(
            __nfds: i32,
            __readfds: *mut root::fd_set,
            __writefds: *mut root::fd_set,
            __exceptfds: *mut root::fd_set,
            __timeout: *const root::timespec,
            __sigmask: *const root::__sigset_t,
        ) -> i32;
    }
    extern "C" {
        pub fn __cmsg_nxthdr(
            __mhdr: *mut root::msghdr,
            __cmsg: *mut root::cmsghdr,
        ) -> *mut root::cmsghdr;
    }

    extern "C" {
        pub fn socket(
            __domain: i32,
            __type: i32,
            __protocol: i32,
        ) -> i32;
    }
    extern "C" {
        pub fn socketpair(
            __domain: i32,
            __type: i32,
            __protocol: i32,
            __fds: *mut i32,
        ) -> i32;
    }
    extern "C" {
        pub fn bind(
            __fd: i32,
            __addr: *const root::sockaddr,
            __len: root::socklen_t,
        ) -> i32;
    }
    extern "C" {
        pub fn getsockname(
            __fd: i32,
            __addr: *mut root::sockaddr,
            __len: *mut root::socklen_t,
        ) -> i32;
    }
    extern "C" {
        pub fn connect(
            __fd: i32,
            __addr: *const root::sockaddr,
            __len: root::socklen_t,
        ) -> i32;
    }
    extern "C" {
        pub fn getpeername(
            __fd: i32,
            __addr: *mut root::sockaddr,
            __len: *mut root::socklen_t,
        ) -> i32;
    }
    extern "C" {
        pub fn send(
            __fd: i32,
            __buf: *const u8,
            __n: root::size_t,
            __flags: i32,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn recv(
            __fd: i32,
            __buf: *mut u8,
            __n: root::size_t,
            __flags: i32,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn sendto(
            __fd: i32,
            __buf: *const u8,
            __n: root::size_t,
            __flags: i32,
            __addr: *const root::sockaddr,
            __addr_len: root::socklen_t,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn recvfrom(
            __fd: i32,
            __buf: *mut u8,
            __n: root::size_t,
            __flags: i32,
            __addr: *mut root::sockaddr,
            __addr_len: *mut root::socklen_t,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn sendmsg(
            __fd: i32,
            __message: *const root::msghdr,
            __flags: i32,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn sendmmsg(
            __fd: i32,
            __vmessages: *mut root::mmsghdr,
            __vlen: u32,
            __flags: i32,
        ) -> i32;
    }
    extern "C" {
        pub fn recvmsg(
            __fd: i32,
            __message: *mut root::msghdr,
            __flags: i32,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn recvmmsg(
            __fd: i32,
            __vmessages: *mut root::mmsghdr,
            __vlen: u32,
            __flags: i32,
            __tmo: *mut root::timespec,
        ) -> i32;
    }
    extern "C" {
        pub fn getsockopt(
            __fd: i32,
            __level: i32,
            __optname: i32,
            __optval: *mut u8,
            __optlen: *mut root::socklen_t,
        ) -> i32;
    }
    extern "C" {
        pub fn setsockopt(
            __fd: i32,
            __level: i32,
            __optname: i32,
            __optval: *const u8,
            __optlen: root::socklen_t,
        ) -> i32;
    }
    extern "C" {
        pub fn listen(__fd: i32, __n: i32) -> i32;
    }
    extern "C" {
        pub fn accept(
            __fd: i32,
            __addr: *mut root::sockaddr,
            __addr_len: *mut root::socklen_t,
        ) -> i32;
    }
    extern "C" {
        pub fn accept4(
            __fd: i32,
            __addr: *mut root::sockaddr,
            __addr_len: *mut root::socklen_t,
            __flags: i32,
        ) -> i32;
    }
    extern "C" {
        pub fn shutdown(__fd: i32, __how: i32) -> i32;
    }
    extern "C" {
        pub fn sockatmark(__fd: i32) -> i32;
    }
    extern "C" {
        pub fn isfdtype(__fd: i32, __fdtype: i32) -> i32;
    }

}
