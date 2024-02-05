use std::{
    ffi::{c_int, CString},
    marker::PhantomData,
    ptr,
};

use chrono::{DateTime, Utc};
use libc::{c_char, size_t};
use winstructs::timestamp::WinTimestamp;

use crate::scca::read_string::ReadString;
use crate::scca::{libscca_error_t, AccessFlags};

#[allow(non_camel_case_types)]
pub type libscca_file_t = *const c_int;

#[link(name = "scca")]
extern "C" {
    /// Creates a file
    fn libscca_file_initialize(
        file: *mut *const libscca_file_t,
        error: *mut *const libscca_error_t,
    ) -> c_int;

    /// Frees a file
    fn libscca_file_free(
        file: *mut *const libscca_file_t,
        error: *mut *const libscca_error_t,
    ) -> c_int;

    /// Opens a file
    fn libscca_file_open(
        file: *mut libscca_file_t,
        filename: *const c_char,
        access_flags: c_int,
        error: *mut *const libscca_error_t,
    ) -> c_int;

    /// Closes a file
    fn libscca_file_close(file: *mut libscca_file_t, error: *mut *const libscca_error_t) -> c_int;

    fn libscca_file_get_format_version(
        file: *const libscca_file_t,
        format_version: *mut u32,
        error: *mut *const libscca_error_t,
    ) -> c_int;

    fn libscca_file_get_utf8_executable_filename_size(
        file: *const libscca_file_t,
        utf8_string_size: *mut size_t,
        error: *mut *const libscca_error_t,
    ) -> c_int;

    fn libscca_file_get_utf8_executable_filename(
        file: *const libscca_file_t,
        utf8_string: *mut u8,
        utf8_string_size: size_t,
        error: *mut *const libscca_error_t,
    ) -> c_int;

    fn libscca_file_get_utf16_executable_filename_size(
        file: *const libscca_file_t,
        utf16_string_size: *mut size_t,
        error: *mut *const libscca_error_t,
    ) -> c_int;

    fn libscca_file_get_utf16_executable_filename(
        file: *const libscca_file_t,
        utf16_string: *mut u16,
        utf16_string_size: size_t,
        error: *mut *const libscca_error_t,
    ) -> c_int;

    fn libscca_file_get_last_run_time(
        file: *const libscca_file_t,
        last_run_time_index: c_int,
        filetime: *mut u64,
        error: *mut *const libscca_error_t,
    ) -> c_int;

    fn libscca_file_get_run_count(
        file: *const libscca_file_t,
        run_count: *mut u32,
        error: *mut *const libscca_error_t,
    ) -> c_int;
}

pub struct File<'f> {
    file: *const libscca_file_t,
    phantom: PhantomData<&'f ()>,
}

impl<'f> File<'f> {
    /// Opens a prefetch file
    pub fn open(filename: &str) -> Result<Self, crate::scca::Error> {
        let mut file = ptr::null();
        let mut error = ptr::null();
        unsafe {
            if 1 != libscca_file_initialize(&mut file, &mut error) {
                return Err(error.into());
            }

            let c_filename = CString::new(filename).expect("unable to create CString");

            error = ptr::null();
            if 1 != libscca_file_open(
                file.cast_mut(),
                c_filename.as_ptr(),
                AccessFlags::AccessRead as c_int,
                &mut error,
            ) {
                return Err(error.into());
            }
        }

        Ok(Self {
            file,
            phantom: PhantomData,
        })
    }

    pub(crate) fn file(&self) -> *const libscca_file_t {
        self.file
    }

    /// Retrieves the format version
    pub fn format_version(&self) -> Result<u32, crate::scca::Error> {
        unsafe {
            let mut error = ptr::null();
            let mut version = 0;
            if 1 != libscca_file_get_format_version(self.file, &mut version, &mut error) {
                return Err(error.into());
            }
            Ok(version)
        }
    }

    /// Retrieves the run count
    pub fn run_count(&self) -> Result<u32, crate::scca::Error> {
        unsafe {
            let mut error = ptr::null();
            let mut count = 0;
            if 1 != libscca_file_get_run_count(self.file, &mut count, &mut error) {
                return Err(error.into());
            }
            Ok(count)
        }
    }

    /// Retrieves a specific last run time
    ///
    /// The timestamp is a 64-bit FILETIME date and time value
    ///
    /// Files of format version 23 and earlier contain a single last run time
    ///
    /// Files of format version 26 and later contain up to 8 last run time
    pub fn last_run_times(&self) -> Result<Vec<DateTime<Utc>>, crate::scca::Error> {
        let max_run_counts = if self.format_version()? < 26 { 1 } else { 8 };

        let mut times = Vec::new();
        for index in 0..max_run_counts {
            let time = unsafe {
                let mut error = ptr::null();
                let mut filetime = 0;
                if 1 != libscca_file_get_last_run_time(self.file, index, &mut filetime, &mut error)
                {
                    //return Err(error.into());
                    break;
                }
                filetime
            };
            if time != 0 {
                times.push(
                    WinTimestamp::new(&time.to_le_bytes())
                        .unwrap()
                        .to_datetime(),
                );
            }
        }
        Ok(times)
    }

    /// Retrieves a specific UTF-8 encoded executable filename
    pub fn utf8_executable_filename(&self) -> Result<String, crate::scca::Error> {
        self.read_string(
            libscca_file_get_utf8_executable_filename_size,
            libscca_file_get_utf8_executable_filename,
        )
    }

    /// Retrieves a specific UTF-16 encoded executable filename
    pub fn utf16_executable_filename(&self) -> Result<String, crate::scca::Error> {
        self.read_string(
            libscca_file_get_utf16_executable_filename_size,
            libscca_file_get_utf16_executable_filename,
        )
    }
}

impl<'f> Drop for File<'f> {
    fn drop(&mut self) {
        unsafe {
            let mut error = ptr::null();
            if 1 != libscca_file_close(self.file.cast_mut(), &mut error) {
                log::info!("{}", crate::scca::Error::from(error));
            }

            error = ptr::null();
            if 1 != libscca_file_free(&mut self.file, &mut error) {
                panic!("{}", crate::scca::Error::from(error));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use chrono::{DateTime, Utc};

    use crate::scca::File;

    #[test]
    fn test_open_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push("data");
        d.push("scca");
        d.push("RUNDLL32.EXE-411A328D.pf");
        assert!(d.exists());
        let filename = d.to_string_lossy().to_string();

        let file = File::open(&filename).unwrap();
        assert_eq!(file.format_version().unwrap(), 23);
        assert_eq!(file.utf8_executable_filename().unwrap(), "RUNDLL32.EXE");
        assert_eq!(file.utf16_executable_filename().unwrap(), "RUNDLL32.EXE");
        for time in file.last_run_times().unwrap() {
            assert_ne!(time, DateTime::<Utc>::default());
        }
    }
}
