use std::ffi::{c_char, CStr};

#[link(name = "scca")]
extern "C" {
    fn libscca_get_version() -> *const c_char;
}

pub fn get_version() -> String {
    unsafe {
        let slice = CStr::from_ptr(libscca_get_version());
        slice.to_str().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::scca::get_version;

    #[test]
    fn test_get_version() {
        assert!(get_version().starts_with("20"));
    }
}
