use crc32fast::Hasher;
use std::os::raw::c_char;
use std::slice;

#[repr(C)]
pub struct HasherHandle(*mut Hasher);

#[no_mangle]
pub extern "C" fn hasher_new() -> *mut HasherHandle {
    let hasher = Box::new(Hasher::new());
    let handle = Box::new(HasherHandle(Box::into_raw(hasher)));
    Box::into_raw(handle)
}

/// # Safety
///
/// Uses unsafe method calls
#[no_mangle]
pub unsafe extern "C" fn hasher_write(handle: *mut HasherHandle, data: *const c_char, len: usize) {
    if handle.is_null() || data.is_null() {
        return;
    }

    let hasher = &mut *(*handle).0;
    let bytes = slice::from_raw_parts(data as *const u8, len);
    hasher.update(bytes);
}

/// # Safety
///
/// Uses unsafe method calls
#[no_mangle]
pub unsafe extern "C" fn hasher_finalize(handle: *mut HasherHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }

    let handle = Box::from_raw(handle);
    let hasher = Box::from_raw(handle.0);
    hasher.finalize()
}

#[no_mangle]
pub extern "C" fn crc32_hash(data: *const c_char, len: usize) -> u32 {
    if data.is_null() {
        return 0;
    }

    unsafe {
        let bytes = slice::from_raw_parts(data as *const u8, len);
        crc32fast::hash(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_hasher_lifecycle() {
        unsafe {
            let handle = hasher_new();
            assert!(!handle.is_null(), "Hasher creation failed");

            let data = b"123456789";
            hasher_write(handle, data.as_ptr() as *const c_char, data.len());

            let sum = hasher_finalize(handle);
            assert_eq!(sum, 0xCBF43926, "CRC32 calculation incorrect");
        }
    }

    #[test]
    fn test_simple_hash() {
        let data = b"123456789";
        let sum = crc32_hash(data.as_ptr() as *const c_char, data.len());
        assert_eq!(sum, 0xCBF43926, "Direct hash calculation incorrect");
    }

    #[test]
    fn test_null_handling() {
        unsafe {
            hasher_write(ptr::null_mut(), b"test".as_ptr() as *const c_char, 4);
            assert_eq!(
                hasher_finalize(ptr::null_mut()),
                0,
                "Null handle should return 0"
            );
            assert_eq!(crc32_hash(ptr::null(), 0), 0, "Null data should return 0");
        }
    }

    #[test]
    fn test_empty_data() {
        let sum = crc32_hash(b"".as_ptr() as *const c_char, 0);
        assert_eq!(sum, 0, "Empty data should produce 0");
    }

    #[test]
    fn test_incremental_update() {
        unsafe {
            let handle = hasher_new();

            // Write data incrementally
            let data = "hello world";
            for byte in data.bytes() {
                hasher_write(handle, &byte as *const u8 as *const c_char, 1);
            }

            let sum = hasher_finalize(handle);
            let direct_sum = crc32_hash(data.as_ptr() as *const c_char, data.len());
            assert_eq!(sum, direct_sum, "Incremental update failed");
        }
    }
}
