/* automatically generated by rust-bindgen 0.66.1 */

pub type __s8 = crate::ctypes::c_schar;
pub type __u8 = crate::ctypes::c_uchar;
pub type __s16 = crate::ctypes::c_short;
pub type __u16 = crate::ctypes::c_ushort;
pub type __s32 = crate::ctypes::c_int;
pub type __u32 = crate::ctypes::c_uint;
pub type __s64 = crate::ctypes::c_longlong;
pub type __u64 = crate::ctypes::c_ulonglong;
pub type addr_t = crate::ctypes::c_ulong;
pub type saddr_t = crate::ctypes::c_long;
pub type __kernel_key_t = crate::ctypes::c_int;
pub type __kernel_mqd_t = crate::ctypes::c_int;
pub type __kernel_size_t = crate::ctypes::c_ulong;
pub type __kernel_ssize_t = crate::ctypes::c_long;
pub type __kernel_old_dev_t = crate::ctypes::c_ushort;
pub type __kernel_ino_t = crate::ctypes::c_uint;
pub type __kernel_mode_t = crate::ctypes::c_uint;
pub type __kernel_ipc_pid_t = crate::ctypes::c_int;
pub type __kernel_uid_t = crate::ctypes::c_uint;
pub type __kernel_gid_t = crate::ctypes::c_uint;
pub type __kernel_ptrdiff_t = crate::ctypes::c_long;
pub type __kernel_sigset_t = crate::ctypes::c_ulong;
pub type __kernel_long_t = crate::ctypes::c_long;
pub type __kernel_ulong_t = crate::ctypes::c_ulong;
pub type __kernel_pid_t = crate::ctypes::c_int;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = crate::ctypes::c_int;
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
pub type __s128 = i128;
pub type __u128 = u128;
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
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct __vector128 {
pub __bindgen_anon_1: __vector128__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __vector128__bindgen_ty_1__bindgen_ty_1 {
pub high: __u64,
pub low: __u64,
}
#[repr(C)]
#[derive(Debug)]
pub struct sysinfo {
pub uptime: __kernel_long_t,
pub loads: [__kernel_ulong_t; 3usize],
pub totalram: __kernel_ulong_t,
pub freeram: __kernel_ulong_t,
pub sharedram: __kernel_ulong_t,
pub bufferram: __kernel_ulong_t,
pub totalswap: __kernel_ulong_t,
pub freeswap: __kernel_ulong_t,
pub procs: __u16,
pub pad: __u16,
pub totalhigh: __kernel_ulong_t,
pub freehigh: __kernel_ulong_t,
pub mem_unit: __u32,
pub _f: __IncompleteArrayField<crate::ctypes::c_char>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct oldold_utsname {
pub sysname: [crate::ctypes::c_char; 9usize],
pub nodename: [crate::ctypes::c_char; 9usize],
pub release: [crate::ctypes::c_char; 9usize],
pub version: [crate::ctypes::c_char; 9usize],
pub machine: [crate::ctypes::c_char; 9usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct old_utsname {
pub sysname: [crate::ctypes::c_char; 65usize],
pub nodename: [crate::ctypes::c_char; 65usize],
pub release: [crate::ctypes::c_char; 65usize],
pub version: [crate::ctypes::c_char; 65usize],
pub machine: [crate::ctypes::c_char; 65usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct new_utsname {
pub sysname: [crate::ctypes::c_char; 65usize],
pub nodename: [crate::ctypes::c_char; 65usize],
pub release: [crate::ctypes::c_char; 65usize],
pub version: [crate::ctypes::c_char; 65usize],
pub machine: [crate::ctypes::c_char; 65usize],
pub domainname: [crate::ctypes::c_char; 65usize],
}
pub const SI_LOAD_SHIFT: u32 = 16;
pub const __OLD_UTS_LEN: u32 = 8;
pub const __NEW_UTS_LEN: u32 = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __vector128__bindgen_ty_1 {
pub __bindgen_anon_1: __vector128__bindgen_ty_1__bindgen_ty_1,
pub u: [__u32; 4usize],
}
impl<T> __IncompleteArrayField<T> {
#[inline]
pub const fn new() -> Self {
__IncompleteArrayField(::core::marker::PhantomData, [])
}
#[inline]
pub fn as_ptr(&self) -> *const T {
self as *const _ as *const T
}
#[inline]
pub fn as_mut_ptr(&mut self) -> *mut T {
self as *mut _ as *mut T
}
#[inline]
pub unsafe fn as_slice(&self, len: usize) -> &[T] {
::core::slice::from_raw_parts(self.as_ptr(), len)
}
#[inline]
pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
}
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
fmt.write_str("__IncompleteArrayField")
}
}
