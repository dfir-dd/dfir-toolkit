#![allow(non_camel_case_types)]
use std::{
    ffi::{c_char, CStr},
    fmt::Display,
    marker::PhantomData,
    os::raw::c_int,
};

use libc::size_t;

#[link(name = "scca")]
extern "C" {
    /// Frees an error
    fn libscca_error_free(error: *const *const libscca_error_t);

    /// Prints a descriptive string of the error to the string
    /// The end-of-string character is not included in the return value
    /// Returns the number of printed characters if successful or -1 on error
    fn libscca_error_sprint(
        error: *const libscca_error_t,
        string: *mut c_char,
        size: size_t,
    ) -> c_int;
}

pub type libscca_error_t = *const c_int;

pub struct Error<'a> {
    error: *const libscca_error_t,
    phantom: PhantomData<&'a ()>,
}

impl<'a> From<*const libscca_error_t> for Error<'a> {
    fn from(error: *const libscca_error_t) -> Self {
        Self {
            error,
            phantom: PhantomData,
        }
    }
}

impl<'a> Drop for Error<'a> {
    fn drop(&mut self) {
        unsafe { libscca_error_free(&self.error) }
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const BUFFER_SIZE: usize = 1024;
        let mut buffer = vec![0; BUFFER_SIZE];

        unsafe {
            libscca_error_sprint(self.error, buffer.as_mut_ptr(), BUFFER_SIZE);
            buffer[BUFFER_SIZE - 1] = 0;
            CStr::from_ptr(buffer.as_ptr()).to_string_lossy().fmt(f)
        }
    }
}


impl<'a> std::fmt::Debug for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const BUFFER_SIZE: usize = 1024;
        let mut buffer = vec![0; BUFFER_SIZE];

        unsafe {
            libscca_error_sprint(self.error, buffer.as_mut_ptr(), BUFFER_SIZE);
            buffer[BUFFER_SIZE - 1] = 0;
            std::fmt::Display::fmt(&CStr::from_ptr(buffer.as_ptr()).to_string_lossy(), f)
        }
    }
}

impl<'a> From<Error<'a>> for anyhow::Error {
    fn from(value: Error<'a>) -> Self {
        anyhow::anyhow!("{value}")
    }
}