#[allow(unused_imports)]
use self::super::root;

#[repr(C)]
pub struct DirectoryEntry {
    pub name: [u8; 769usize],
    pub _x302: [u8; 3usize],
    pub type_: u8,
    pub _x304: u8,
    pub fileSize: root::s64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FileHandle {
    pub handle: *mut u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DirectoryHandle {
    pub handle: *mut u8,
}

pub const DirectoryEntryType_DirectoryEntryType_Directory:
    root::nn::fs::DirectoryEntryType = 0;
pub const DirectoryEntryType_DirectoryEntryType_File: root::nn::fs::DirectoryEntryType =
    1;
pub type DirectoryEntryType = u32;
pub const OpenMode_OpenMode_Read: root::nn::fs::OpenMode = 1;
pub const OpenMode_OpenMode_Write: root::nn::fs::OpenMode = 2;
pub const OpenMode_OpenMode_Append: root::nn::fs::OpenMode = 4;
pub const OpenMode_OpenMode_ReadWrite: root::nn::fs::OpenMode = 3;
pub type OpenMode = u32;
pub const OpenDirectoryMode_OpenDirectoryMode_Directory:
root::nn::fs::OpenDirectoryMode = 1;
pub const OpenDirectoryMode_OpenDirectoryMode_File: root::nn::fs::OpenDirectoryMode = 2;
pub const OpenDirectoryMode_OpenDirectoryMode_All: root::nn::fs::OpenDirectoryMode = 3;
pub type OpenDirectoryMode = u32;
pub const WriteOptionFlag_WriteOptionFlag_Flush: root::nn::fs::WriteOptionFlag = 1;
pub type WriteOptionFlag = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WriteOption {
    pub flags: i32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct FileTimeStamp {
    pub create: root::nn::time::PosixTime,
    pub modify: root::nn::time::PosixTime,
    pub access: root::nn::time::PosixTime,
    pub local_time: bool,
    padding: [u8; 7],
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs22QueryMountRomCacheSizeEPm"]
    pub fn QueryMountRomCacheSize(size: *mut u64) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs22QueryMountRomCacheSizeEPmm"]
    pub fn QueryMountRomCacheSize1(
        size: *mut u64,
        arg1: root::nn::ApplicationId,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs8MountRomEPKcPvm"]
    pub fn MountRom(
        name: *const u8,
        buffer: *mut u8,
        bufferSize: root::ulong,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs19CanMountRomForDebugEv"]
    pub fn CanMountRomForDebug() -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs11CanMountRomEm"]
    pub fn CanMountRom(arg1: root::nn::ApplicationId) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs28QueryMountRomOnFileCacheSizeEPmNS0_10FileHandleE"]
    pub fn QueryMountRomOnFileCacheSize(
        arg1: *mut u64,
        arg2: root::nn::fs::FileHandle,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs14MountRomOnFileEPKcNS0_10FileHandleEPvm"]
    pub fn MountRomOnFile(
        arg1: *const u8,
        arg2: root::nn::fs::FileHandle,
        arg3: *mut u8,
        arg4: u64,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs14EnsureSaveDataERKNS_7account3UidE"]
    pub fn EnsureSaveData(arg1: *const root::nn::account::Uid) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs6CommitEPKc"]
    pub fn Commit(mount_point: *const u8) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs13MountSaveDataEPKcRKNS_7account3UidE"]
    pub fn MountSaveData(
        mount_point: *const u8,
        user_id: *const root::nn::account::Uid,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs21MountSaveDataForDebugEPKc"]
    pub fn MountSaveDataForDebug(
        mount_point: *const u8
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs12GetEntryTypeEPNS0_18DirectoryEntryTypeEPKc"]
    pub fn GetEntryType(
        type_: *mut root::nn::fs::DirectoryEntryType,
        path: *const u8,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs10CreateFileEPKcl"]
    pub fn CreateFile(filepath: *const u8, size: root::s64) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs8OpenFileEPNS0_10FileHandleEPKci"]
    pub fn OpenFile(
        arg1: *mut root::nn::fs::FileHandle,
        path: *const u8,
        arg2: root::s32,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs11SetFileSizeENS0_10FileHandleEl"]
    pub fn SetFileSize(
        fileHandle: root::nn::fs::FileHandle,
        filesize: root::s64,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs9CloseFileENS0_10FileHandleE"]
    pub fn CloseFile(fileHandle: root::nn::fs::FileHandle);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs9FlushFileENS0_10FileHandleE"]
    pub fn FlushFile(fileHandle: root::nn::fs::FileHandle) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs10DeleteFileEPKc"]
    pub fn DeleteFile(filepath: *const u8) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs15DeleteDirectoryEPKc"]
    pub fn DeleteDirectory(path: *const u8) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs26DeleteDirectoryRecursivelyEPKc"]
    pub fn DeleteDirectoryRecursively(path: *const u8) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs10RenameFileEPKcS2_"]
    pub fn RenameFile(
        old: *const u8,
        new: *const u8,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs15RenameDirectoryEPKcS2_"]
    pub fn RenameDirectory(
        old: *const u8,
        new: *const u8,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs8ReadFileEPmNS0_10FileHandleElPvmRKi"]
    pub fn ReadFile(
        outSize: *mut u64,
        handle: root::nn::fs::FileHandle,
        offset: root::s64,
        buffer: *mut u8,
        bufferSize: u64,
        arg1: *const root::s32,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs8ReadFileEPmNS0_10FileHandleElPvm"]
    pub fn ReadFile1(
        outSize: *mut u64,
        handle: root::nn::fs::FileHandle,
        offset: root::s64,
        buffer: *mut u8,
        bufferSize: u64,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs8ReadFileENS0_10FileHandleElPvm"]
    pub fn ReadFile2(
        handle: root::nn::fs::FileHandle,
        offset: root::s64,
        buffer: *mut u8,
        bufferSize: u64,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs9WriteFileENS0_10FileHandleElPKvmRKNS0_11WriteOptionE"]
    pub fn WriteFile(
        handle: root::nn::fs::FileHandle,
        fileOffset: root::s64,
        buff: *const u8,
        size: u64,
        option: *const root::nn::fs::WriteOption,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs11GetFileSizeEPlNS0_10FileHandleE"]
    pub fn GetFileSize(
        size: *mut root::s64,
        fileHandle: root::nn::fs::FileHandle,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs13OpenDirectoryEPNS0_15DirectoryHandleEPKci"]
    pub fn OpenDirectory(
        handle: *mut root::nn::fs::DirectoryHandle,
        path: *const u8,
        openMode: root::s32,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs14CloseDirectoryENS0_15DirectoryHandleE"]
    pub fn CloseDirectory(directoryHandle: root::nn::fs::DirectoryHandle);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs13ReadDirectoryEPlPNS0_14DirectoryEntryENS0_15DirectoryHandleEl"]
    pub fn ReadDirectory(
        arg1: *mut root::s64,
        arg2: *mut root::nn::fs::DirectoryEntry,
        directoryHandle: root::nn::fs::DirectoryHandle,
        arg3: root::s64,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs15CreateDirectoryEPKc"]
    pub fn CreateDirectory(directorypath: *const u8) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs22GetDirectoryEntryCountEPlNS0_15DirectoryHandleE"]
    pub fn GetDirectoryEntryCount(
        arg1: *mut root::s64,
        arg2: root::nn::fs::DirectoryHandle,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs11MountSdCardEPKc"]
    pub fn MountSdCard(arg1: *const u8) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs19MountSdCardForDebugEPKc"]
    pub fn MountSdCardForDebug(arg1: *const u8) -> root::Result;
}
extern "C" {
    #[link_name = "_ZN2nn2fs7UnmountEPKc"]
    pub fn Unmount(name: *const u8);
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs16IsSdCardInsertedEv"]
    pub fn IsSdCardInserted() -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs12FormatSdCardEv"]
    pub fn FormatSdCard() -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs18FormatSdCardDryRunEv"]
    pub fn FormatSdCardDryRun() -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs16IsExFatSupportedEv"]
    pub fn IsExFatSupported() -> bool;
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn2fs24GetFileTimeStampForDebugEPNS0_13FileTimeStampEPKc"]
    pub fn GetFileTimeStampForDebug(
        out_timestamp: *mut FileTimeStamp,
        filepath: *const u8,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "_ZN2nn2fs17MountCacheStorageEPKc"]
    pub fn MountCacheStorage(mount_point: *const u8) -> root::Result;
}
pub fn mount_cache_storage<S: AsRef<str>>(mount_point: S) -> root::Result {
    unsafe {
        MountCacheStorage([mount_point.as_ref(), "\0"].concat().as_ptr())
    }
}