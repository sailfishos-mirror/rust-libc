//! RISC-V-specific definitions for 32-bit linux-like values

use crate::prelude::*;
use crate::{off64_t, off_t};

pub type wchar_t = c_int;

s! {
    pub struct msqid_ds {
        pub msg_perm: crate::ipc_perm,
        pub msg_stime: crate::time_t,
        pub msg_rtime: crate::time_t,
        pub msg_ctime: crate::time_t,
        pub __msg_cbytes: c_ulong,
        pub msg_qnum: crate::msgqnum_t,
        pub msg_qbytes: crate::msglen_t,
        pub msg_lspid: crate::pid_t,
        pub msg_lrpid: crate::pid_t,
        __glibc_reserved4: c_ulong,
        __glibc_reserved5: c_ulong,
    }

    pub struct stat64 {
        pub st_dev: crate::dev_t,
        pub st_ino: crate::ino64_t,
        pub st_mode: crate::mode_t,
        pub st_nlink: crate::nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: crate::dev_t,
        pub __pad1: crate::dev_t,
        pub st_size: off64_t,
        pub st_blksize: crate::blksize_t,
        pub __pad2: c_int,
        pub st_blocks: crate::blkcnt64_t,
        pub st_atime: crate::time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: crate::time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: crate::time_t,
        pub st_ctime_nsec: c_long,
        __unused: [c_int; 2],
    }

    pub struct statfs {
        pub f_type: c_long,
        pub f_bsize: c_long,
        pub f_blocks: crate::fsblkcnt_t,
        pub f_bfree: crate::fsblkcnt_t,
        pub f_bavail: crate::fsblkcnt_t,
        pub f_files: crate::fsfilcnt_t,
        pub f_ffree: crate::fsfilcnt_t,
        pub f_fsid: crate::fsid_t,
        pub f_namelen: c_long,
        pub f_frsize: c_long,
        pub f_flags: c_long,
        pub f_spare: [c_long; 4],
    }

    pub struct statfs64 {
        pub f_type: c_long,
        pub f_bsize: c_long,
        pub f_blocks: crate::fsblkcnt64_t,
        pub f_bfree: crate::fsblkcnt64_t,
        pub f_bavail: crate::fsblkcnt64_t,
        pub f_files: crate::fsfilcnt64_t,
        pub f_ffree: crate::fsfilcnt64_t,
        pub f_fsid: crate::fsid_t,
        pub f_namelen: c_long,
        pub f_frsize: c_long,
        pub f_flags: c_long,
        pub f_spare: [c_long; 4],
    }

    pub struct statvfs64 {
        pub f_bsize: c_ulong,
        pub f_frsize: c_ulong,
        pub f_blocks: crate::fsblkcnt64_t,
        pub f_bfree: crate::fsblkcnt64_t,
        pub f_bavail: crate::fsblkcnt64_t,
        pub f_files: crate::fsfilcnt64_t,
        pub f_ffree: crate::fsfilcnt64_t,
        pub f_favail: crate::fsfilcnt64_t,
        pub f_fsid: c_ulong,
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
        pub __f_spare: [c_int; 6],
    }

    pub struct siginfo_t {
        pub si_signo: c_int,
        pub si_errno: c_int,
        pub si_code: c_int,
        #[doc(hidden)]
        #[deprecated(
            since = "0.2.54",
            note = "Please leave a comment on \
                  https://github.com/rust-lang/libc/pull/1316 if you're using \
                  this field"
        )]
        pub _pad: [c_int; 29],
        _align: [u64; 0],
    }

    pub struct stack_t {
        pub ss_sp: *mut c_void,
        pub ss_flags: c_int,
        pub ss_size: size_t,
    }

    // FIXME(1.0): This should not implement `PartialEq`
    #[allow(unpredictable_function_pointer_comparisons)]
    pub struct sigaction {
        pub sa_sigaction: crate::sighandler_t,
        pub sa_mask: crate::sigset_t,
        pub sa_flags: c_int,
        pub sa_restorer: Option<unsafe extern "C" fn()>,
    }

    pub struct ipc_perm {
        pub __key: crate::key_t,
        pub uid: crate::uid_t,
        pub gid: crate::gid_t,
        pub cuid: crate::uid_t,
        pub cgid: crate::gid_t,
        pub mode: c_ushort,
        __pad1: c_ushort,
        pub __seq: c_ushort,
        __pad2: c_ushort,
        __unused1: c_ulong,
        __unused2: c_ulong,
    }

    pub struct shmid_ds {
        pub shm_perm: crate::ipc_perm,
        pub shm_segsz: size_t,
        pub shm_atime: crate::time_t,
        pub shm_dtime: crate::time_t,
        pub shm_ctime: crate::time_t,
        pub shm_cpid: crate::pid_t,
        pub shm_lpid: crate::pid_t,
        pub shm_nattch: crate::shmatt_t,
        __unused5: c_ulong,
        __unused6: c_ulong,
    }

    pub struct flock {
        pub l_type: c_short,
        pub l_whence: c_short,
        pub l_start: off_t,
        pub l_len: off_t,
        pub l_pid: crate::pid_t,
    }

    pub struct flock64 {
        pub l_type: c_short,
        pub l_whence: c_short,
        pub l_start: off64_t,
        pub l_len: off64_t,
        pub l_pid: crate::pid_t,
    }

    pub struct user_regs_struct {
        pub pc: c_ulong,
        pub ra: c_ulong,
        pub sp: c_ulong,
        pub gp: c_ulong,
        pub tp: c_ulong,
        pub t0: c_ulong,
        pub t1: c_ulong,
        pub t2: c_ulong,
        pub s0: c_ulong,
        pub s1: c_ulong,
        pub a0: c_ulong,
        pub a1: c_ulong,
        pub a2: c_ulong,
        pub a3: c_ulong,
        pub a4: c_ulong,
        pub a5: c_ulong,
        pub a6: c_ulong,
        pub a7: c_ulong,
        pub s2: c_ulong,
        pub s3: c_ulong,
        pub s4: c_ulong,
        pub s5: c_ulong,
        pub s6: c_ulong,
        pub s7: c_ulong,
        pub s8: c_ulong,
        pub s9: c_ulong,
        pub s10: c_ulong,
        pub s11: c_ulong,
        pub t3: c_ulong,
        pub t4: c_ulong,
        pub t5: c_ulong,
        pub t6: c_ulong,
    }
}

s_no_extra_traits! {
    pub struct ucontext_t {
        pub __uc_flags: c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: crate::stack_t,
        pub uc_sigmask: crate::sigset_t,
        pub uc_mcontext: mcontext_t,
    }

    #[repr(align(16))]
    pub struct mcontext_t {
        pub __gregs: [c_ulong; 32],
        pub __fpregs: __riscv_mc_fp_state,
    }

    pub union __riscv_mc_fp_state {
        pub __f: __riscv_mc_f_ext_state,
        pub __d: __riscv_mc_d_ext_state,
        pub __q: __riscv_mc_q_ext_state,
    }

    pub struct __riscv_mc_f_ext_state {
        pub __f: [c_uint; 32],
        pub __fcsr: c_uint,
    }

    pub struct __riscv_mc_d_ext_state {
        pub __f: [c_ulonglong; 32],
        pub __fcsr: c_uint,
    }

    #[repr(align(16))]
    pub struct __riscv_mc_q_ext_state {
        pub __f: [c_ulonglong; 64],
        pub __fcsr: c_uint,
        pub __glibc_reserved: [c_uint; 3],
    }
}

pub const O_LARGEFILE: c_int = 0;
pub const VEOF: usize = 4;
pub const RTLD_DEEPBIND: c_int = 0x8;
pub const RTLD_GLOBAL: c_int = 0x100;
pub const RTLD_NOLOAD: c_int = 0x4;
pub const O_APPEND: c_int = 1024;
pub const O_CREAT: c_int = 64;
pub const O_EXCL: c_int = 128;
pub const O_NOCTTY: c_int = 256;
pub const O_NONBLOCK: c_int = 2048;
pub const O_SYNC: c_int = 1052672;
pub const O_RSYNC: c_int = 1052672;
pub const O_DSYNC: c_int = 4096;
pub const O_FSYNC: c_int = 1052672;
pub const MADV_SOFT_OFFLINE: c_int = 101;
pub const MAP_GROWSDOWN: c_int = 256;
pub const EDEADLK: c_int = 35;
pub const ENAMETOOLONG: c_int = 36;
pub const ENOLCK: c_int = 37;
pub const ENOSYS: c_int = 38;
pub const ENOTEMPTY: c_int = 39;
pub const ELOOP: c_int = 40;
pub const ENOMSG: c_int = 42;
pub const EIDRM: c_int = 43;
pub const ECHRNG: c_int = 44;
pub const EL2NSYNC: c_int = 45;
pub const EL3HLT: c_int = 46;
pub const EL3RST: c_int = 47;
pub const ELNRNG: c_int = 48;
pub const EUNATCH: c_int = 49;
pub const ENOCSI: c_int = 50;
pub const EL2HLT: c_int = 51;
pub const EBADE: c_int = 52;
pub const EBADR: c_int = 53;
pub const EXFULL: c_int = 54;
pub const ENOANO: c_int = 55;
pub const EBADRQC: c_int = 56;
pub const EBADSLT: c_int = 57;
pub const EMULTIHOP: c_int = 72;
pub const EOVERFLOW: c_int = 75;
pub const ENOTUNIQ: c_int = 76;
pub const EBADFD: c_int = 77;
pub const EBADMSG: c_int = 74;
pub const EREMCHG: c_int = 78;
pub const ELIBACC: c_int = 79;
pub const ELIBBAD: c_int = 80;
pub const ELIBSCN: c_int = 81;
pub const ELIBMAX: c_int = 82;
pub const ELIBEXEC: c_int = 83;
pub const EILSEQ: c_int = 84;
pub const ERESTART: c_int = 85;
pub const ESTRPIPE: c_int = 86;
pub const EUSERS: c_int = 87;
pub const ENOTSOCK: c_int = 88;
pub const EDESTADDRREQ: c_int = 89;
pub const EMSGSIZE: c_int = 90;
pub const EPROTOTYPE: c_int = 91;
pub const ENOPROTOOPT: c_int = 92;
pub const EPROTONOSUPPORT: c_int = 93;
pub const ESOCKTNOSUPPORT: c_int = 94;
pub const EOPNOTSUPP: c_int = 95;
pub const EPFNOSUPPORT: c_int = 96;
pub const EAFNOSUPPORT: c_int = 97;
pub const EADDRINUSE: c_int = 98;
pub const EADDRNOTAVAIL: c_int = 99;
pub const ENETDOWN: c_int = 100;
pub const ENETUNREACH: c_int = 101;
pub const ENETRESET: c_int = 102;
pub const ECONNABORTED: c_int = 103;
pub const ECONNRESET: c_int = 104;
pub const ENOBUFS: c_int = 105;
pub const EISCONN: c_int = 106;
pub const ENOTCONN: c_int = 107;
pub const ESHUTDOWN: c_int = 108;
pub const ETOOMANYREFS: c_int = 109;
pub const ETIMEDOUT: c_int = 110;
pub const ECONNREFUSED: c_int = 111;
pub const EHOSTDOWN: c_int = 112;
pub const EHOSTUNREACH: c_int = 113;
pub const EALREADY: c_int = 114;
pub const EINPROGRESS: c_int = 115;
pub const ESTALE: c_int = 116;
pub const EDQUOT: c_int = 122;
pub const ENOMEDIUM: c_int = 123;
pub const EMEDIUMTYPE: c_int = 124;
pub const ECANCELED: c_int = 125;
pub const ENOKEY: c_int = 126;
pub const EKEYEXPIRED: c_int = 127;
pub const EKEYREVOKED: c_int = 128;
pub const EKEYREJECTED: c_int = 129;
pub const EOWNERDEAD: c_int = 130;
pub const ENOTRECOVERABLE: c_int = 131;
pub const EHWPOISON: c_int = 133;
pub const ERFKILL: c_int = 132;

pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SA_SIGINFO: c_int = 4;
pub const SA_NOCLDWAIT: c_int = 2;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;
pub const SIGVTALRM: c_int = 26;
pub const SIGPROF: c_int = 27;
pub const SIGWINCH: c_int = 28;
pub const SIGCHLD: c_int = 17;
pub const SIGBUS: c_int = 7;
pub const SIGUSR1: c_int = 10;
pub const SIGUSR2: c_int = 12;
pub const SIGCONT: c_int = 18;
pub const SIGSTOP: c_int = 19;
pub const SIGTSTP: c_int = 20;
pub const SIGURG: c_int = 23;
pub const SIGIO: c_int = 29;
pub const SIGSYS: c_int = 31;
pub const SIGSTKFLT: c_int = 16;
pub const SIGPOLL: c_int = 29;
pub const SIGPWR: c_int = 30;
pub const SIG_SETMASK: c_int = 2;
pub const SIG_BLOCK: c_int = 0;
pub const SIG_UNBLOCK: c_int = 1;
pub const POLLWRNORM: c_short = 256;
pub const POLLWRBAND: c_short = 512;
pub const O_ASYNC: c_int = 8192;
pub const O_NDELAY: c_int = 2048;
pub const EFD_NONBLOCK: c_int = 2048;
pub const F_GETLK: c_int = 5;
pub const F_GETOWN: c_int = 9;
pub const F_SETOWN: c_int = 8;
pub const SFD_NONBLOCK: c_int = 2048;
pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;

pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4;
pub const O_DIRECT: c_int = 16384;
pub const O_DIRECTORY: c_int = 65536;
pub const O_NOFOLLOW: c_int = 131072;
pub const MAP_HUGETLB: c_int = 262144;
pub const MAP_LOCKED: c_int = 8192;
pub const MAP_NORESERVE: c_int = 16384;
pub const MAP_ANON: c_int = 32;
pub const MAP_ANONYMOUS: c_int = 32;
pub const MAP_DENYWRITE: c_int = 2048;
pub const MAP_EXECUTABLE: c_int = 4096;
pub const MAP_POPULATE: c_int = 32768;
pub const MAP_NONBLOCK: c_int = 65536;
pub const MAP_STACK: c_int = 131072;
pub const MAP_SYNC: c_int = 0x080000;
pub const EDEADLOCK: c_int = 35;
pub const EUCLEAN: c_int = 117;
pub const ENOTNAM: c_int = 118;
pub const ENAVAIL: c_int = 119;
pub const EISNAM: c_int = 120;
pub const EREMOTEIO: c_int = 121;
pub const MCL_CURRENT: c_int = 1;
pub const MCL_FUTURE: c_int = 2;
pub const MCL_ONFAULT: c_int = 4;
pub const SIGSTKSZ: size_t = 8192;
pub const MINSIGSTKSZ: size_t = 2048;
pub const CBAUD: crate::tcflag_t = 4111;
pub const TAB1: crate::tcflag_t = 2048;
pub const TAB2: crate::tcflag_t = 4096;
pub const TAB3: crate::tcflag_t = 6144;
pub const CR1: crate::tcflag_t = 512;
pub const CR2: crate::tcflag_t = 1024;
pub const CR3: crate::tcflag_t = 1536;
pub const FF1: crate::tcflag_t = 32768;
pub const BS1: crate::tcflag_t = 8192;
pub const VT1: crate::tcflag_t = 16384;
pub const VWERASE: usize = 14;
pub const VREPRINT: usize = 12;
pub const VSUSP: usize = 10;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VDISCARD: usize = 13;
pub const VTIME: usize = 5;
pub const IXON: crate::tcflag_t = 1024;
pub const IXOFF: crate::tcflag_t = 4096;
pub const ONLCR: crate::tcflag_t = 4;
pub const CSIZE: crate::tcflag_t = 48;
pub const CS6: crate::tcflag_t = 16;
pub const CS7: crate::tcflag_t = 32;
pub const CS8: crate::tcflag_t = 48;
pub const CSTOPB: crate::tcflag_t = 64;
pub const CREAD: crate::tcflag_t = 128;
pub const PARENB: crate::tcflag_t = 256;
pub const PARODD: crate::tcflag_t = 512;
pub const HUPCL: crate::tcflag_t = 1024;
pub const CLOCAL: crate::tcflag_t = 2048;
pub const ECHOKE: crate::tcflag_t = 2048;
pub const ECHOE: crate::tcflag_t = 16;
pub const ECHOK: crate::tcflag_t = 32;
pub const ECHONL: crate::tcflag_t = 64;
pub const ECHOPRT: crate::tcflag_t = 1024;
pub const ECHOCTL: crate::tcflag_t = 512;
pub const ISIG: crate::tcflag_t = 1;
pub const ICANON: crate::tcflag_t = 2;
pub const PENDIN: crate::tcflag_t = 16384;
pub const NOFLSH: crate::tcflag_t = 128;
pub const CIBAUD: crate::tcflag_t = 269418496;
pub const CBAUDEX: crate::tcflag_t = 4096;
pub const VSWTC: usize = 7;
pub const OLCUC: crate::tcflag_t = 2;
pub const NLDLY: crate::tcflag_t = 256;
pub const CRDLY: crate::tcflag_t = 1536;
pub const TABDLY: crate::tcflag_t = 6144;
pub const BSDLY: crate::tcflag_t = 8192;
pub const FFDLY: crate::tcflag_t = 32768;
pub const VTDLY: crate::tcflag_t = 16384;
pub const XTABS: crate::tcflag_t = 6144;
pub const B0: crate::speed_t = 0;
pub const B50: crate::speed_t = 1;
pub const B75: crate::speed_t = 2;
pub const B110: crate::speed_t = 3;
pub const B134: crate::speed_t = 4;
pub const B150: crate::speed_t = 5;
pub const B200: crate::speed_t = 6;
pub const B300: crate::speed_t = 7;
pub const B600: crate::speed_t = 8;
pub const B1200: crate::speed_t = 9;
pub const B1800: crate::speed_t = 10;
pub const B2400: crate::speed_t = 11;
pub const B4800: crate::speed_t = 12;
pub const B9600: crate::speed_t = 13;
pub const B19200: crate::speed_t = 14;
pub const B38400: crate::speed_t = 15;
pub const EXTA: crate::speed_t = 14;
pub const EXTB: crate::speed_t = 15;
pub const B57600: crate::speed_t = 4097;
pub const B115200: crate::speed_t = 4098;
pub const B230400: crate::speed_t = 4099;
pub const B460800: crate::speed_t = 4100;
pub const B500000: crate::speed_t = 4101;
pub const B576000: crate::speed_t = 4102;
pub const B921600: crate::speed_t = 4103;
pub const B1000000: crate::speed_t = 4104;
pub const B1152000: crate::speed_t = 4105;
pub const B1500000: crate::speed_t = 4106;
pub const B2000000: crate::speed_t = 4107;
pub const B2500000: crate::speed_t = 4108;
pub const B3000000: crate::speed_t = 4109;
pub const B3500000: crate::speed_t = 4110;
pub const B4000000: crate::speed_t = 4111;
pub const VEOL: usize = 11;
pub const VEOL2: usize = 16;
pub const VMIN: usize = 6;
pub const IEXTEN: crate::tcflag_t = 32768;
pub const TOSTOP: crate::tcflag_t = 256;
pub const FLUSHO: crate::tcflag_t = 4096;
pub const EXTPROC: crate::tcflag_t = 65536;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 32;
pub const NGREG: usize = 32;
pub const REG_PC: usize = 0;
pub const REG_RA: usize = 1;
pub const REG_SP: usize = 2;
pub const REG_TP: usize = 4;
pub const REG_S0: usize = 8;
pub const REG_S1: usize = 9;
pub const REG_A0: usize = 10;
pub const REG_S2: usize = 18;
pub const REG_NARGS: usize = 8;

pub const SYS_read: c_long = 63;
pub const SYS_write: c_long = 64;
pub const SYS_close: c_long = 57;
pub const SYS_fstat: c_long = 80;
pub const SYS_lseek: c_long = 62;
pub const SYS_mmap: c_long = 222;
pub const SYS_mprotect: c_long = 226;
pub const SYS_munmap: c_long = 215;
pub const SYS_brk: c_long = 214;
pub const SYS_rt_sigaction: c_long = 134;
pub const SYS_rt_sigprocmask: c_long = 135;
pub const SYS_rt_sigreturn: c_long = 139;
pub const SYS_ioctl: c_long = 29;
pub const SYS_pread64: c_long = 67;
pub const SYS_pwrite64: c_long = 68;
pub const SYS_readv: c_long = 65;
pub const SYS_writev: c_long = 66;
pub const SYS_sched_yield: c_long = 124;
pub const SYS_mremap: c_long = 216;
pub const SYS_msync: c_long = 227;
pub const SYS_mincore: c_long = 232;
pub const SYS_madvise: c_long = 233;
pub const SYS_shmget: c_long = 194;
pub const SYS_shmat: c_long = 196;
pub const SYS_shmctl: c_long = 195;
pub const SYS_dup: c_long = 23;
pub const SYS_nanosleep: c_long = 101;
pub const SYS_getitimer: c_long = 102;
pub const SYS_setitimer: c_long = 103;
pub const SYS_getpid: c_long = 172;
pub const SYS_sendfile: c_long = 71;
pub const SYS_socket: c_long = 198;
pub const SYS_connect: c_long = 203;
pub const SYS_accept: c_long = 202;
pub const SYS_sendto: c_long = 206;
pub const SYS_recvfrom: c_long = 207;
pub const SYS_sendmsg: c_long = 211;
pub const SYS_recvmsg: c_long = 212;
pub const SYS_shutdown: c_long = 210;
pub const SYS_bind: c_long = 200;
pub const SYS_listen: c_long = 201;
pub const SYS_getsockname: c_long = 204;
pub const SYS_getpeername: c_long = 205;
pub const SYS_socketpair: c_long = 199;
pub const SYS_setsockopt: c_long = 208;
pub const SYS_getsockopt: c_long = 209;
pub const SYS_clone: c_long = 220;
pub const SYS_execve: c_long = 221;
pub const SYS_exit: c_long = 93;
pub const SYS_wait4: c_long = 260;
pub const SYS_kill: c_long = 129;
pub const SYS_uname: c_long = 160;
pub const SYS_semget: c_long = 190;
pub const SYS_semop: c_long = 193;
pub const SYS_semctl: c_long = 191;
pub const SYS_shmdt: c_long = 197;
pub const SYS_msgget: c_long = 186;
pub const SYS_msgsnd: c_long = 189;
pub const SYS_msgrcv: c_long = 188;
pub const SYS_msgctl: c_long = 187;
pub const SYS_fcntl: c_long = 25;
pub const SYS_flock: c_long = 32;
pub const SYS_fsync: c_long = 82;
pub const SYS_fdatasync: c_long = 83;
pub const SYS_truncate: c_long = 45;
pub const SYS_ftruncate: c_long = 46;
pub const SYS_getcwd: c_long = 17;
pub const SYS_chdir: c_long = 49;
pub const SYS_fchdir: c_long = 50;
pub const SYS_fchmod: c_long = 52;
pub const SYS_fchown: c_long = 55;
pub const SYS_umask: c_long = 166;
pub const SYS_gettimeofday: c_long = 169;
pub const SYS_getrlimit: c_long = 163;
pub const SYS_getrusage: c_long = 165;
pub const SYS_sysinfo: c_long = 179;
pub const SYS_times: c_long = 153;
pub const SYS_ptrace: c_long = 117;
pub const SYS_getuid: c_long = 174;
pub const SYS_syslog: c_long = 116;
pub const SYS_getgid: c_long = 176;
pub const SYS_setuid: c_long = 146;
pub const SYS_setgid: c_long = 144;
pub const SYS_geteuid: c_long = 175;
pub const SYS_getegid: c_long = 177;
pub const SYS_setpgid: c_long = 154;
pub const SYS_getppid: c_long = 173;
pub const SYS_setsid: c_long = 157;
pub const SYS_setreuid: c_long = 145;
pub const SYS_setregid: c_long = 143;
pub const SYS_getgroups: c_long = 158;
pub const SYS_setgroups: c_long = 159;
pub const SYS_setresuid: c_long = 147;
pub const SYS_getresuid: c_long = 148;
pub const SYS_setresgid: c_long = 149;
pub const SYS_getresgid: c_long = 150;
pub const SYS_getpgid: c_long = 155;
pub const SYS_setfsuid: c_long = 151;
pub const SYS_setfsgid: c_long = 152;
pub const SYS_getsid: c_long = 156;
pub const SYS_capget: c_long = 90;
pub const SYS_capset: c_long = 91;
pub const SYS_rt_sigpending: c_long = 136;
pub const SYS_rt_sigtimedwait: c_long = 137;
pub const SYS_rt_sigqueueinfo: c_long = 138;
pub const SYS_rt_sigsuspend: c_long = 133;
pub const SYS_sigaltstack: c_long = 132;
pub const SYS_personality: c_long = 92;
pub const SYS_statfs: c_long = 43;
pub const SYS_fstatfs: c_long = 44;
pub const SYS_getpriority: c_long = 141;
pub const SYS_setpriority: c_long = 140;
pub const SYS_sched_setparam: c_long = 118;
pub const SYS_sched_getparam: c_long = 121;
pub const SYS_sched_setscheduler: c_long = 119;
pub const SYS_sched_getscheduler: c_long = 120;
pub const SYS_sched_get_priority_max: c_long = 125;
pub const SYS_sched_get_priority_min: c_long = 126;
pub const SYS_sched_rr_get_interval: c_long = 127;
pub const SYS_mlock: c_long = 228;
pub const SYS_munlock: c_long = 229;
pub const SYS_mlockall: c_long = 230;
pub const SYS_munlockall: c_long = 231;
pub const SYS_vhangup: c_long = 58;
pub const SYS_pivot_root: c_long = 41;
pub const SYS_prctl: c_long = 167;
pub const SYS_adjtimex: c_long = 171;
pub const SYS_setrlimit: c_long = 164;
pub const SYS_chroot: c_long = 51;
pub const SYS_sync: c_long = 81;
pub const SYS_acct: c_long = 89;
pub const SYS_settimeofday: c_long = 170;
pub const SYS_mount: c_long = 40;
pub const SYS_umount2: c_long = 39;
pub const SYS_swapon: c_long = 224;
pub const SYS_swapoff: c_long = 225;
pub const SYS_reboot: c_long = 142;
pub const SYS_sethostname: c_long = 161;
pub const SYS_setdomainname: c_long = 162;
pub const SYS_init_module: c_long = 105;
pub const SYS_delete_module: c_long = 106;
pub const SYS_quotactl: c_long = 60;
pub const SYS_nfsservctl: c_long = 42;
pub const SYS_gettid: c_long = 178;
pub const SYS_readahead: c_long = 213;
pub const SYS_setxattr: c_long = 5;
pub const SYS_lsetxattr: c_long = 6;
pub const SYS_fsetxattr: c_long = 7;
pub const SYS_getxattr: c_long = 8;
pub const SYS_lgetxattr: c_long = 9;
pub const SYS_fgetxattr: c_long = 10;
pub const SYS_listxattr: c_long = 11;
pub const SYS_llistxattr: c_long = 12;
pub const SYS_flistxattr: c_long = 13;
pub const SYS_removexattr: c_long = 14;
pub const SYS_lremovexattr: c_long = 15;
pub const SYS_fremovexattr: c_long = 16;
pub const SYS_tkill: c_long = 130;
pub const SYS_futex: c_long = 98;
pub const SYS_sched_setaffinity: c_long = 122;
pub const SYS_sched_getaffinity: c_long = 123;
pub const SYS_io_setup: c_long = 0;
pub const SYS_io_destroy: c_long = 1;
pub const SYS_io_getevents: c_long = 4;
pub const SYS_io_submit: c_long = 2;
pub const SYS_io_cancel: c_long = 3;
pub const SYS_lookup_dcookie: c_long = 18;
pub const SYS_remap_file_pages: c_long = 234;
pub const SYS_getdents64: c_long = 61;
pub const SYS_set_tid_address: c_long = 96;
pub const SYS_restart_syscall: c_long = 128;
pub const SYS_semtimedop: c_long = 192;
pub const SYS_fadvise64: c_long = 223;
pub const SYS_timer_create: c_long = 107;
pub const SYS_timer_settime: c_long = 110;
pub const SYS_timer_gettime: c_long = 108;
pub const SYS_timer_getoverrun: c_long = 109;
pub const SYS_timer_delete: c_long = 111;
pub const SYS_clock_settime: c_long = 112;
pub const SYS_clock_gettime: c_long = 113;
pub const SYS_clock_getres: c_long = 114;
pub const SYS_clock_nanosleep: c_long = 115;
pub const SYS_exit_group: c_long = 94;
pub const SYS_epoll_ctl: c_long = 21;
pub const SYS_tgkill: c_long = 131;
pub const SYS_mbind: c_long = 235;
pub const SYS_set_mempolicy: c_long = 237;
pub const SYS_get_mempolicy: c_long = 236;
pub const SYS_mq_open: c_long = 180;
pub const SYS_mq_unlink: c_long = 181;
pub const SYS_mq_timedsend: c_long = 182;
pub const SYS_mq_timedreceive: c_long = 183;
pub const SYS_mq_notify: c_long = 184;
pub const SYS_mq_getsetattr: c_long = 185;
pub const SYS_kexec_load: c_long = 104;
pub const SYS_waitid: c_long = 95;
pub const SYS_add_key: c_long = 217;
pub const SYS_request_key: c_long = 218;
pub const SYS_keyctl: c_long = 219;
pub const SYS_ioprio_set: c_long = 30;
pub const SYS_ioprio_get: c_long = 31;
pub const SYS_inotify_add_watch: c_long = 27;
pub const SYS_inotify_rm_watch: c_long = 28;
pub const SYS_migrate_pages: c_long = 238;
pub const SYS_openat: c_long = 56;
pub const SYS_mkdirat: c_long = 34;
pub const SYS_mknodat: c_long = 33;
pub const SYS_fchownat: c_long = 54;
pub const SYS_newfstatat: c_long = 79;
pub const SYS_unlinkat: c_long = 35;
pub const SYS_linkat: c_long = 37;
pub const SYS_symlinkat: c_long = 36;
pub const SYS_readlinkat: c_long = 78;
pub const SYS_fchmodat: c_long = 53;
pub const SYS_faccessat: c_long = 48;
pub const SYS_pselect6: c_long = 72;
pub const SYS_ppoll: c_long = 73;
pub const SYS_unshare: c_long = 97;
pub const SYS_set_robust_list: c_long = 99;
pub const SYS_get_robust_list: c_long = 100;
pub const SYS_splice: c_long = 76;
pub const SYS_tee: c_long = 77;
pub const SYS_sync_file_range: c_long = 84;
pub const SYS_vmsplice: c_long = 75;
pub const SYS_move_pages: c_long = 239;
pub const SYS_utimensat: c_long = 88;
pub const SYS_epoll_pwait: c_long = 22;
pub const SYS_timerfd_create: c_long = 85;
pub const SYS_fallocate: c_long = 47;
pub const SYS_timerfd_settime: c_long = 86;
pub const SYS_timerfd_gettime: c_long = 87;
pub const SYS_accept4: c_long = 242;
pub const SYS_signalfd4: c_long = 74;
pub const SYS_eventfd2: c_long = 19;
pub const SYS_epoll_create1: c_long = 20;
pub const SYS_dup3: c_long = 24;
pub const SYS_pipe2: c_long = 59;
pub const SYS_inotify_init1: c_long = 26;
pub const SYS_preadv: c_long = 69;
pub const SYS_pwritev: c_long = 70;
pub const SYS_rt_tgsigqueueinfo: c_long = 240;
pub const SYS_perf_event_open: c_long = 241;
pub const SYS_recvmmsg: c_long = 243;
pub const SYS_fanotify_init: c_long = 262;
pub const SYS_fanotify_mark: c_long = 263;
pub const SYS_prlimit64: c_long = 261;
pub const SYS_name_to_handle_at: c_long = 264;
pub const SYS_open_by_handle_at: c_long = 265;
pub const SYS_clock_adjtime: c_long = 266;
pub const SYS_syncfs: c_long = 267;
pub const SYS_sendmmsg: c_long = 269;
pub const SYS_setns: c_long = 268;
pub const SYS_getcpu: c_long = 168;
pub const SYS_process_vm_readv: c_long = 270;
pub const SYS_process_vm_writev: c_long = 271;
pub const SYS_kcmp: c_long = 272;
pub const SYS_finit_module: c_long = 273;
pub const SYS_sched_setattr: c_long = 274;
pub const SYS_sched_getattr: c_long = 275;
pub const SYS_renameat2: c_long = 276;
pub const SYS_seccomp: c_long = 277;
pub const SYS_getrandom: c_long = 278;
pub const SYS_memfd_create: c_long = 279;
pub const SYS_bpf: c_long = 280;
pub const SYS_execveat: c_long = 281;
pub const SYS_userfaultfd: c_long = 282;
pub const SYS_membarrier: c_long = 283;
pub const SYS_mlock2: c_long = 284;
pub const SYS_copy_file_range: c_long = 285;
pub const SYS_preadv2: c_long = 286;
pub const SYS_pwritev2: c_long = 287;
pub const SYS_pkey_mprotect: c_long = 288;
pub const SYS_pkey_alloc: c_long = 289;
pub const SYS_pkey_free: c_long = 290;
pub const SYS_statx: c_long = 291;
pub const SYS_rseq: c_long = 293;
pub const SYS_pidfd_send_signal: c_long = 424;
pub const SYS_io_uring_setup: c_long = 425;
pub const SYS_io_uring_enter: c_long = 426;
pub const SYS_io_uring_register: c_long = 427;
pub const SYS_open_tree: c_long = 428;
pub const SYS_move_mount: c_long = 429;
pub const SYS_fsopen: c_long = 430;
pub const SYS_fsconfig: c_long = 431;
pub const SYS_fsmount: c_long = 432;
pub const SYS_fspick: c_long = 433;
pub const SYS_pidfd_open: c_long = 434;
pub const SYS_clone3: c_long = 435;
pub const SYS_close_range: c_long = 436;
pub const SYS_openat2: c_long = 437;
pub const SYS_pidfd_getfd: c_long = 438;
pub const SYS_faccessat2: c_long = 439;
pub const SYS_process_madvise: c_long = 440;
pub const SYS_epoll_pwait2: c_long = 441;
pub const SYS_mount_setattr: c_long = 442;
pub const SYS_quotactl_fd: c_long = 443;
pub const SYS_landlock_create_ruleset: c_long = 444;
pub const SYS_landlock_add_rule: c_long = 445;
pub const SYS_landlock_restrict_self: c_long = 446;
pub const SYS_memfd_secret: c_long = 447;
pub const SYS_process_mrelease: c_long = 448;
pub const SYS_futex_waitv: c_long = 449;
pub const SYS_set_mempolicy_home_node: c_long = 450;
