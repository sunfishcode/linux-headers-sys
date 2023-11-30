/* automatically generated by rust-bindgen 0.66.1 */

pub const EPERM: u32 = 1;
pub const ENOENT: u32 = 2;
pub const ESRCH: u32 = 3;
pub const EINTR: u32 = 4;
pub const EIO: u32 = 5;
pub const ENXIO: u32 = 6;
pub const E2BIG: u32 = 7;
pub const ENOEXEC: u32 = 8;
pub const EBADF: u32 = 9;
pub const ECHILD: u32 = 10;
pub const EAGAIN: u32 = 11;
pub const ENOMEM: u32 = 12;
pub const EACCES: u32 = 13;
pub const EFAULT: u32 = 14;
pub const ENOTBLK: u32 = 15;
pub const EBUSY: u32 = 16;
pub const EEXIST: u32 = 17;
pub const EXDEV: u32 = 18;
pub const ENODEV: u32 = 19;
pub const ENOTDIR: u32 = 20;
pub const EISDIR: u32 = 21;
pub const EINVAL: u32 = 22;
pub const ENFILE: u32 = 23;
pub const EMFILE: u32 = 24;
pub const ENOTTY: u32 = 25;
pub const ETXTBSY: u32 = 26;
pub const EFBIG: u32 = 27;
pub const ENOSPC: u32 = 28;
pub const ESPIPE: u32 = 29;
pub const EROFS: u32 = 30;
pub const EMLINK: u32 = 31;
pub const EPIPE: u32 = 32;
pub const EDOM: u32 = 33;
pub const ERANGE: u32 = 34;
pub const ENOMSG: u32 = 35;
pub const EIDRM: u32 = 36;
pub const ECHRNG: u32 = 37;
pub const EL2NSYNC: u32 = 38;
pub const EL3HLT: u32 = 39;
pub const EL3RST: u32 = 40;
pub const ELNRNG: u32 = 41;
pub const EUNATCH: u32 = 42;
pub const ENOCSI: u32 = 43;
pub const EL2HLT: u32 = 44;
pub const EDEADLK: u32 = 45;
pub const ENOLCK: u32 = 46;
pub const EBADE: u32 = 50;
pub const EBADR: u32 = 51;
pub const EXFULL: u32 = 52;
pub const ENOANO: u32 = 53;
pub const EBADRQC: u32 = 54;
pub const EBADSLT: u32 = 55;
pub const EDEADLOCK: u32 = 56;
pub const EBFONT: u32 = 59;
pub const ENOSTR: u32 = 60;
pub const ENODATA: u32 = 61;
pub const ETIME: u32 = 62;
pub const ENOSR: u32 = 63;
pub const ENONET: u32 = 64;
pub const ENOPKG: u32 = 65;
pub const EREMOTE: u32 = 66;
pub const ENOLINK: u32 = 67;
pub const EADV: u32 = 68;
pub const ESRMNT: u32 = 69;
pub const ECOMM: u32 = 70;
pub const EPROTO: u32 = 71;
pub const EDOTDOT: u32 = 73;
pub const EMULTIHOP: u32 = 74;
pub const EBADMSG: u32 = 77;
pub const ENAMETOOLONG: u32 = 78;
pub const EOVERFLOW: u32 = 79;
pub const ENOTUNIQ: u32 = 80;
pub const EBADFD: u32 = 81;
pub const EREMCHG: u32 = 82;
pub const ELIBACC: u32 = 83;
pub const ELIBBAD: u32 = 84;
pub const ELIBSCN: u32 = 85;
pub const ELIBMAX: u32 = 86;
pub const ELIBEXEC: u32 = 87;
pub const EILSEQ: u32 = 88;
pub const ENOSYS: u32 = 89;
pub const ELOOP: u32 = 90;
pub const ERESTART: u32 = 91;
pub const ESTRPIPE: u32 = 92;
pub const ENOTEMPTY: u32 = 93;
pub const EUSERS: u32 = 94;
pub const ENOTSOCK: u32 = 95;
pub const EDESTADDRREQ: u32 = 96;
pub const EMSGSIZE: u32 = 97;
pub const EPROTOTYPE: u32 = 98;
pub const ENOPROTOOPT: u32 = 99;
pub const EPROTONOSUPPORT: u32 = 120;
pub const ESOCKTNOSUPPORT: u32 = 121;
pub const EOPNOTSUPP: u32 = 122;
pub const EPFNOSUPPORT: u32 = 123;
pub const EAFNOSUPPORT: u32 = 124;
pub const EADDRINUSE: u32 = 125;
pub const EADDRNOTAVAIL: u32 = 126;
pub const ENETDOWN: u32 = 127;
pub const ENETUNREACH: u32 = 128;
pub const ENETRESET: u32 = 129;
pub const ECONNABORTED: u32 = 130;
pub const ECONNRESET: u32 = 131;
pub const ENOBUFS: u32 = 132;
pub const EISCONN: u32 = 133;
pub const ENOTCONN: u32 = 134;
pub const EUCLEAN: u32 = 135;
pub const ENOTNAM: u32 = 137;
pub const ENAVAIL: u32 = 138;
pub const EISNAM: u32 = 139;
pub const EREMOTEIO: u32 = 140;
pub const EINIT: u32 = 141;
pub const EREMDEV: u32 = 142;
pub const ESHUTDOWN: u32 = 143;
pub const ETOOMANYREFS: u32 = 144;
pub const ETIMEDOUT: u32 = 145;
pub const ECONNREFUSED: u32 = 146;
pub const EHOSTDOWN: u32 = 147;
pub const EHOSTUNREACH: u32 = 148;
pub const EWOULDBLOCK: u32 = 11;
pub const EALREADY: u32 = 149;
pub const EINPROGRESS: u32 = 150;
pub const ESTALE: u32 = 151;
pub const ECANCELED: u32 = 158;
pub const ENOMEDIUM: u32 = 159;
pub const EMEDIUMTYPE: u32 = 160;
pub const ENOKEY: u32 = 161;
pub const EKEYEXPIRED: u32 = 162;
pub const EKEYREVOKED: u32 = 163;
pub const EKEYREJECTED: u32 = 164;
pub const EOWNERDEAD: u32 = 165;
pub const ENOTRECOVERABLE: u32 = 166;
pub const ERFKILL: u32 = 167;
pub const EHWPOISON: u32 = 168;
pub const EDQUOT: u32 = 1133;
pub const MPOL_F_STATIC_NODES: u32 = 32768;
pub const MPOL_F_RELATIVE_NODES: u32 = 16384;
pub const MPOL_F_NUMA_BALANCING: u32 = 8192;
pub const MPOL_MODE_FLAGS: u32 = 57344;
pub const MPOL_F_NODE: u32 = 1;
pub const MPOL_F_ADDR: u32 = 2;
pub const MPOL_F_MEMS_ALLOWED: u32 = 4;
pub const MPOL_MF_STRICT: u32 = 1;
pub const MPOL_MF_MOVE: u32 = 2;
pub const MPOL_MF_MOVE_ALL: u32 = 4;
pub const MPOL_MF_LAZY: u32 = 8;
pub const MPOL_MF_INTERNAL: u32 = 16;
pub const MPOL_MF_VALID: u32 = 7;
pub const MPOL_F_SHARED: u32 = 1;
pub const MPOL_F_MOF: u32 = 8;
pub const MPOL_F_MORON: u32 = 16;
pub const RECLAIM_ZONE: u32 = 1;
pub const RECLAIM_WRITE: u32 = 2;
pub const RECLAIM_UNMAP: u32 = 4;
pub const MPOL_DEFAULT: _bindgen_ty_1 = _bindgen_ty_1::MPOL_DEFAULT;
pub const MPOL_PREFERRED: _bindgen_ty_1 = _bindgen_ty_1::MPOL_PREFERRED;
pub const MPOL_BIND: _bindgen_ty_1 = _bindgen_ty_1::MPOL_BIND;
pub const MPOL_INTERLEAVE: _bindgen_ty_1 = _bindgen_ty_1::MPOL_INTERLEAVE;
pub const MPOL_LOCAL: _bindgen_ty_1 = _bindgen_ty_1::MPOL_LOCAL;
pub const MPOL_PREFERRED_MANY: _bindgen_ty_1 = _bindgen_ty_1::MPOL_PREFERRED_MANY;
pub const MPOL_MAX: _bindgen_ty_1 = _bindgen_ty_1::MPOL_MAX;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
MPOL_DEFAULT = 0,
MPOL_PREFERRED = 1,
MPOL_BIND = 2,
MPOL_INTERLEAVE = 3,
MPOL_LOCAL = 4,
MPOL_PREFERRED_MANY = 5,
MPOL_MAX = 6,
}
