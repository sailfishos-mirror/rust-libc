//! Common types used by most newlib platforms

s! {
    pub struct sigset_t {
        #[cfg(target_os = "horizon")]
        __val: [::c_ulong; 16],
        #[cfg(not(target_os = "horizon"))]
        __val: u32,
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_atime: ::time_t,
        pub st_spare1: ::c_long,
        pub st_mtime: ::time_t,
        pub st_spare2: ::c_long,
        pub st_ctime: ::time_t,
        pub st_spare3: ::c_long,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_spare4: [::c_long; 2usize],
    }

    pub struct dirent {
        pub d_ino: ::ino_t,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256usize],
    }
}
