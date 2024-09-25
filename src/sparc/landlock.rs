/* automatically generated by rust-bindgen 0.70.1 */

pub type __s8 = crate::ctypes::c_schar;
pub type __u8 = crate::ctypes::c_uchar;
pub type __s16 = crate::ctypes::c_short;
pub type __u16 = crate::ctypes::c_ushort;
pub type __s32 = crate::ctypes::c_int;
pub type __u32 = crate::ctypes::c_uint;
pub type __s64 = crate::ctypes::c_longlong;
pub type __u64 = crate::ctypes::c_ulonglong;
pub type __kernel_key_t = crate::ctypes::c_int;
pub type __kernel_mqd_t = crate::ctypes::c_int;
pub type __kernel_size_t = crate::ctypes::c_uint;
pub type __kernel_ssize_t = crate::ctypes::c_int;
pub type __kernel_ptrdiff_t = crate::ctypes::c_long;
pub type __kernel_ipc_pid_t = crate::ctypes::c_ushort;
pub type __kernel_uid_t = crate::ctypes::c_ushort;
pub type __kernel_gid_t = crate::ctypes::c_ushort;
pub type __kernel_mode_t = crate::ctypes::c_ushort;
pub type __kernel_daddr_t = crate::ctypes::c_long;
pub type __kernel_old_dev_t = crate::ctypes::c_ushort;
pub type __kernel_long_t = crate::ctypes::c_long;
pub type __kernel_ulong_t = crate::ctypes::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_pid_t = crate::ctypes::c_int;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_uid32_t = crate::ctypes::c_uint;
pub type __kernel_gid32_t = crate::ctypes::c_uint;
pub type __kernel_old_uid_t = __kernel_uid_t;
pub type __kernel_old_gid_t = __kernel_gid_t;
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = crate::ctypes::c_longlong;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = crate::ctypes::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = crate::ctypes::c_int;
pub type __kernel_clockid_t = crate::ctypes::c_int;
pub type __kernel_caddr_t = *mut crate::ctypes::c_char;
pub type __kernel_uid16_t = crate::ctypes::c_ushort;
pub type __kernel_gid16_t = crate::ctypes::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = crate::ctypes::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct landlock_ruleset_attr {
pub handled_access_fs: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct landlock_path_beneath_attr {
pub allowed_access: __u64,
pub parent_fd: __s32,
}
pub const LANDLOCK_CREATE_RULESET_VERSION: u32 = 1;
pub const LANDLOCK_ACCESS_FS_EXECUTE: u32 = 1;
pub const LANDLOCK_ACCESS_FS_WRITE_FILE: u32 = 2;
pub const LANDLOCK_ACCESS_FS_READ_FILE: u32 = 4;
pub const LANDLOCK_ACCESS_FS_READ_DIR: u32 = 8;
pub const LANDLOCK_ACCESS_FS_REMOVE_DIR: u32 = 16;
pub const LANDLOCK_ACCESS_FS_REMOVE_FILE: u32 = 32;
pub const LANDLOCK_ACCESS_FS_MAKE_CHAR: u32 = 64;
pub const LANDLOCK_ACCESS_FS_MAKE_DIR: u32 = 128;
pub const LANDLOCK_ACCESS_FS_MAKE_REG: u32 = 256;
pub const LANDLOCK_ACCESS_FS_MAKE_SOCK: u32 = 512;
pub const LANDLOCK_ACCESS_FS_MAKE_FIFO: u32 = 1024;
pub const LANDLOCK_ACCESS_FS_MAKE_BLOCK: u32 = 2048;
pub const LANDLOCK_ACCESS_FS_MAKE_SYM: u32 = 4096;
pub const LANDLOCK_ACCESS_FS_REFER: u32 = 8192;
pub const LANDLOCK_ACCESS_FS_TRUNCATE: u32 = 16384;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum landlock_rule_type {
LANDLOCK_RULE_PATH_BENEATH = 1,
}
