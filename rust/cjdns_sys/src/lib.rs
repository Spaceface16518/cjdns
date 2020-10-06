use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

// This is needed to make sure the linker will pull in sodium here
pub fn init_sodium() {
    sodiumoxide::init().unwrap();
}

pub fn c_args() -> Vec<*mut c_char> {
    std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .map(|arg| arg.into_raw())
        .collect::<Vec<_>>()
}

#[macro_export]
macro_rules! c_main {
    ($cmain:ident) => {
        extern "C" {
            fn $cmain(argc: c_int, argv: *const *mut c_char);
        }

        pub fn main() {
            cjdns_sys::init_sodium();
            let c_args = cjdns_sys::c_args();
            unsafe { $cmain(c_args.len() as c_int, c_args.as_ptr()) }
        }
    };
}
