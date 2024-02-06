use std::{ffi::c_int, ptr};

use libc::size_t;
use num::Zero;

use crate::scca::{libscca_error_t, libscca_file_t, File};


pub (crate) trait CharacterType: Zero + Clone + Eq + PartialEq {}
impl CharacterType for u8 {}
impl CharacterType for u16 {}
pub (crate) trait ReadString<T: CharacterType> {
    fn read_string_with_conversion(
        &self,
        read_size: unsafe extern "C" fn(
            file: *const libscca_file_t,
            string_size: *mut size_t,
            error: *mut *const libscca_error_t,
        ) -> c_int,
        read_value: unsafe extern "C"  fn(
            file: *const libscca_file_t,
            value: *mut T,
            string_size: size_t,
            error: *mut *const libscca_error_t,
        ) -> c_int,
        file: *const libscca_file_t,
        conversion: fn(&[T]) -> String,
    ) -> Result<String, crate::scca::Error> {
        let string_size = unsafe {
            let mut error = ptr::null();
            let mut string_size = 0;
            if 1 != read_size(file, &mut string_size, &mut error) {
                return Err(error.into());
            }
            string_size
        };

        let mut buffer = unsafe {
            let mut buffer: Vec<T> = vec![T::zero(); string_size];
            let mut error = ptr::null();
            if 1 != read_value(file, buffer.as_mut_ptr(), string_size, &mut error) {
                return Err(error.into());
            }
            buffer
        };

        assert!(buffer.last().unwrap().is_zero());
        buffer.pop().unwrap();

        Ok(conversion(&buffer[..]))
    }

    fn read_string(
        &self,
        read_size: unsafe extern "C" fn(
            file: *const libscca_file_t,
            string_size: *mut size_t,
            error: *mut *const libscca_error_t,
        ) -> c_int,
        read_value: unsafe extern "C" fn(
            file: *const libscca_file_t,
            value: *mut T,
            string_size: size_t,
            error: *mut *const libscca_error_t,
        ) -> c_int,
    ) -> Result<String, crate::scca::Error>;
}

impl<'f> ReadString<u8> for File<'f> {
    fn read_string(
        &self,
        read_size: unsafe extern "C" fn(
            file: *const libscca_file_t,
            string_size: *mut size_t,
            error: *mut *const libscca_error_t,
        ) -> c_int,
        read_value: unsafe extern "C" fn(
            file: *const libscca_file_t,
            value: *mut u8,
            string_size: size_t,
            error: *mut *const libscca_error_t,
        ) -> c_int,
    ) -> Result<String, crate::scca::Error> {
        self.read_string_with_conversion(read_size, read_value, self.file(), |s| {
            String::from_utf8_lossy(s).to_string()
        })
    }
}

impl<'f> ReadString<u16> for File<'f> {
    fn read_string(
        &self,
        read_size: unsafe extern "C" fn(
            file: *const libscca_file_t,
            string_size: *mut size_t,
            error: *mut *const libscca_error_t,
        ) -> c_int,
        read_value: unsafe extern "C" fn(
            file: *const libscca_file_t,
            value: *mut u16,
            string_size: size_t,
            error: *mut *const libscca_error_t,
        ) -> c_int,
    ) -> Result<String, crate::scca::Error> {
        self.read_string_with_conversion(read_size, read_value, self.file(), |s| {
            String::from_utf16(s).expect("unable to convert from UTF-16")
        })
    }
}
