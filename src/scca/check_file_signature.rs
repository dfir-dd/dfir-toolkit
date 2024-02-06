use std::{
    ffi::{c_char, CString},
    os::raw::c_int,
    ptr,
};

//use libc::wchar_t;

use crate::scca::{libscca_error_t, Error};

#[link(name = "scca")]
extern "C" {
    fn libscca_check_file_signature(
        filename: *const c_char,
        error: *mut *const libscca_error_t,
    ) -> c_int;

    /*
    fn libscca_check_file_signature_wide(
        filename: *const wchar_t,
        error: *mut *const libscca_error_t,
    ) -> c_int;
     */
}

/// Determines if a file contains a SCCA file signature
///
pub fn has_file_signature(filename: &str) -> Result<bool, Error> {
    if filename.is_ascii() {
        let c_filename = CString::new(filename).expect("unable to create CString");
        unsafe {
            let mut error = ptr::null();
            match libscca_check_file_signature(c_filename.as_ptr(), &mut error) {
                1 => Ok(true),
                0 => Ok(false),
                -1 => Err(Error::from(error)),
                _ => unimplemented!(),
            }
        }
    } else {
        /*
        let mut encoded: Vec<_> = filename.encode_utf16().map(i32::from).collect();
        encoded.push(0);
        unsafe {
            let mut error = ptr::null();
            match libscca_check_file_signature_wide(encoded.as_ptr(), &mut error) {
                1 => Ok(true),
                0 => Ok(false),
                -1 => Err(Error::from(error)),
                _ => unimplemented!(),
            }
        }
         */
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::scca::has_file_signature;

    #[test]
    fn test_missing_file() {
        let res = has_file_signature("invalid_file_name");
        assert!(res.is_err());
        let error = res.unwrap_err();
        assert_eq!("libscca_check_file_signature: unable to check file signature using a file handle.", error.to_string());
    }

    #[test]
    fn test_rundll32() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push("data");
        d.push("scca");
        d.push("RUNDLL32.EXE-411A328D.pf");
        assert!(d.exists());
        let filename = d.to_string_lossy().to_string();
        
        let res = has_file_signature(&filename).unwrap();
        assert!(res);
    }

    /*
    #[test]
    fn test_rundll32_äöü() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push("data");
        d.push("2020JimmyWilson");
        d.push("RUNDLL32-ÄÖÜ.EXE-411A328D.pf");
        assert!(d.exists());
        let filename = d.to_string_lossy().to_string();
        
        let res = has_file_signature(&filename).unwrap();
        assert!(res);
    }
     */
}
