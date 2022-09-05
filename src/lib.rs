#![no_std]
#![feature(repr_simd)]

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
        pub fn main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
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
        pub fn llabs(n: libc::c_longlong) -> libc::c_longlong;
    }
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const libc::c_char,
            __file: *const libc::c_char,
            __line: libc::c_uint,
            __function: *const libc::c_char,
        );
    }
    extern "C" {
        pub fn __assert_perror_fail(
            __errnum: libc::c_int,
            __file: *const libc::c_char,
            __line: libc::c_uint,
            __function: *const libc::c_char,
        );
    }
    extern "C" {
        pub fn __assert(
            __assertion: *const libc::c_char,
            __file: *const libc::c_char,
            __line: libc::c_int,
        );
    }

    extern "C" {
        pub fn select(
            __nfds: libc::c_int,
            __readfds: *mut root::fd_set,
            __writefds: *mut root::fd_set,
            __exceptfds: *mut root::fd_set,
            __timeout: *mut root::timeval,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn pselect(
            __nfds: libc::c_int,
            __readfds: *mut root::fd_set,
            __writefds: *mut root::fd_set,
            __exceptfds: *mut root::fd_set,
            __timeout: *const root::timespec,
            __sigmask: *const root::__sigset_t,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn __cmsg_nxthdr(
            __mhdr: *mut root::msghdr,
            __cmsg: *mut root::cmsghdr,
        ) -> *mut root::cmsghdr;
    }

    extern "C" {
        pub fn socket(
            __domain: libc::c_int,
            __type: libc::c_int,
            __protocol: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn socketpair(
            __domain: libc::c_int,
            __type: libc::c_int,
            __protocol: libc::c_int,
            __fds: *mut libc::c_int,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn bind(
            __fd: libc::c_int,
            __addr: *const root::sockaddr,
            __len: root::socklen_t,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn getsockname(
            __fd: libc::c_int,
            __addr: *mut root::sockaddr,
            __len: *mut root::socklen_t,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn connect(
            __fd: libc::c_int,
            __addr: *const root::sockaddr,
            __len: root::socklen_t,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn getpeername(
            __fd: libc::c_int,
            __addr: *mut root::sockaddr,
            __len: *mut root::socklen_t,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn send(
            __fd: libc::c_int,
            __buf: *const libc::c_void,
            __n: root::size_t,
            __flags: libc::c_int,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn recv(
            __fd: libc::c_int,
            __buf: *mut libc::c_void,
            __n: root::size_t,
            __flags: libc::c_int,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn sendto(
            __fd: libc::c_int,
            __buf: *const libc::c_void,
            __n: root::size_t,
            __flags: libc::c_int,
            __addr: *const root::sockaddr,
            __addr_len: root::socklen_t,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn recvfrom(
            __fd: libc::c_int,
            __buf: *mut libc::c_void,
            __n: root::size_t,
            __flags: libc::c_int,
            __addr: *mut root::sockaddr,
            __addr_len: *mut root::socklen_t,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn sendmsg(
            __fd: libc::c_int,
            __message: *const root::msghdr,
            __flags: libc::c_int,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn sendmmsg(
            __fd: libc::c_int,
            __vmessages: *mut root::mmsghdr,
            __vlen: libc::c_uint,
            __flags: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn recvmsg(
            __fd: libc::c_int,
            __message: *mut root::msghdr,
            __flags: libc::c_int,
        ) -> root::ssize_t;
    }
    extern "C" {
        pub fn recvmmsg(
            __fd: libc::c_int,
            __vmessages: *mut root::mmsghdr,
            __vlen: libc::c_uint,
            __flags: libc::c_int,
            __tmo: *mut root::timespec,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn getsockopt(
            __fd: libc::c_int,
            __level: libc::c_int,
            __optname: libc::c_int,
            __optval: *mut libc::c_void,
            __optlen: *mut root::socklen_t,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn setsockopt(
            __fd: libc::c_int,
            __level: libc::c_int,
            __optname: libc::c_int,
            __optval: *const libc::c_void,
            __optlen: root::socklen_t,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    }
    extern "C" {
        pub fn accept(
            __fd: libc::c_int,
            __addr: *mut root::sockaddr,
            __addr_len: *mut root::socklen_t,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn accept4(
            __fd: libc::c_int,
            __addr: *mut root::sockaddr,
            __addr_len: *mut root::socklen_t,
            __flags: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C" {
        pub fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    }
    extern "C" {
        pub fn sockatmark(__fd: libc::c_int) -> libc::c_int;
    }
    extern "C" {
        pub fn isfdtype(__fd: libc::c_int, __fdtype: libc::c_int) -> libc::c_int;
    }
    pub type __int128_t = i128;
    pub type __uint128_t = u128;
}
