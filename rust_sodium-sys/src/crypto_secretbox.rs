pub const crypto_secretbox_KEYBYTES: usize = 32;
pub const crypto_secretbox_NONCEBYTES: usize = 24;
pub const crypto_secretbox_MACBYTES: usize = 16;
pub const crypto_secretbox_PRIMITIVE: *const c_char = (b"xsalsa20poly1305\0" as *const u8) as
                                                      *const c_char;

extern "C" {
    pub fn crypto_secretbox_easy(c: *mut u8,
                                 m: *const u8,
                                 mlen: c_ulonglong,
                                 n: *const u8,
                                 k: *const u8)
                                 -> c_int;
    pub fn crypto_secretbox_open_easy(m: *mut u8,
                                      c: *const u8,
                                      clen: c_ulonglong,
                                      n: *const u8,
                                      k: *const u8)
                                      -> c_int;
    pub fn crypto_secretbox_detached(c: *mut u8,
                                     mac: *mut u8,
                                     m: *const u8,
                                     mlen: c_ulonglong,
                                     n: *const u8,
                                     k: *const u8)
                                     -> c_int;
    pub fn crypto_secretbox_open_detached(m: *mut u8,
                                          c: *const u8,
                                          mac: *const u8,
                                          clen: c_ulonglong,
                                          n: *const u8,
                                          k: *const u8)
                                          -> c_int;
    pub fn crypto_secretbox_keybytes() -> size_t;
    pub fn crypto_secretbox_noncebytes() -> size_t;
    pub fn crypto_secretbox_macbytes() -> size_t;
    pub fn crypto_secretbox_primitive() -> *const c_char;
}

#[test]
fn test_crypto_secretbox_keybytes() {
    assert!(unsafe { crypto_secretbox_keybytes() as usize } == crypto_secretbox_KEYBYTES)
}

#[test]
fn test_crypto_secretbox_noncebytes() {
    assert!(unsafe { crypto_secretbox_noncebytes() as usize } == crypto_secretbox_NONCEBYTES)
}

#[test]
fn test_crypto_secretbox_macbytes() {
    assert!(unsafe { crypto_secretbox_macbytes() as usize } == crypto_secretbox_MACBYTES)
}

#[test]
fn test_crypto_secretbox_primitive() {
    use std::ffi::CStr;
    unsafe {
        assert_eq!(CStr::from_ptr(crypto_secretbox_PRIMITIVE),
                   CStr::from_ptr(crypto_secretbox_primitive()));
    }
}
