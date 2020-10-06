// This is needed to make sure the linker will pull in sodium here
pub fn init_sodium() {
    sodiumoxide::init().unwrap();
}

#[macro_export]
macro_rules! c_main {
    ($cmain:ident) => {
        use std::ffi::CString;
        use std::os::raw::c_char;
        use std::os::raw::c_int;
        
        extern "C" {
            fn $cmain(argc: c_int, argv: *const *mut c_char);
        }
        
        #[no_mangle]
        pub extern "C" fn main(argc: c_int, argv: *const *mut c_char) -> c_int {
            cjdns_sys::init_sodium();
            unsafe { $cmain(c_args.len() as c_int, c_args.as_ptr()) }
            0
        }
    };
}
