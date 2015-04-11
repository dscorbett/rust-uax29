//! This module provides a wrapper for the word break iterator over a string
//! This iterator follows the 

//#![crate_type = "dylib"]
extern crate libc;
use std::ffi::{CString, c_str_to_bytes};
use std::{mem, sync, ptr};
use defaults;
use breaks::word_break::Category;

//static GLOBAL_LOCK: sync::StaticMutex = sync::MUTEX_INIT;
//static mut client_startup_hook: Option<fn () -> Option<()>> = None;
pub type WordBreakerPtr = *const ();

///
#[no_mangle]
pub unsafe extern "C" fn create_word_breaker(txt: *const libc::c_char) -> WordBreakerPtr {
	//let _guard = GLOBAL_LOCK.lock().unwrap();
	let buf = c_str_to_bytes(&txt);
	let input = String::from_utf8(buf.to_vec()).unwrap();
	let wb = Box::new(defaults::Breaks::new(input.as_slice(),
	   defaults::make_word_break_tree()));
	mem::transmute(wb)
}

#[no_mangle]
pub unsafe extern "C" fn next_word(wb: WordBreakerPtr) -> *const libc::c_char {
	//let _guard = GLOBAL_LOCK.lock().unwrap();
	let wb: &mut defaults::Breaks<Category> = mem::transmute(wb);
	match wb.next() {
		Some(s) => CString::from_slice(s.as_bytes()).as_ptr(),
		None => ptr::null()
	}
}

#[cfg(test)]
mod test_cpp_wrapper {
    extern crate libc;
    use super::{create_word_breaker, next_word, WordBreakerPtr};
    use std::ffi::{CString, c_str_to_bytes};
    use std::{ptr};

    #[test]
    fn test_iterate() {
        unsafe {
        let txt = CString::from_slice("1.21 gigawatts.".as_bytes());
        let p: WordBreakerPtr = create_word_breaker(txt.as_ptr());
        assert_eq!(c_char_to_str(next_word(p)), "1.21");
        assert_eq!(c_char_to_str(next_word(p)), " ");
        assert_eq!(c_char_to_str(next_word(p)), "gigawatts");
        assert_eq!(c_char_to_str(next_word(p)), ".");
        assert_eq!(next_word(p), ptr::null());
        }
    }

    unsafe fn c_char_to_str(txt: *const libc::c_char) -> String {
        let buf = c_str_to_bytes(&txt);
        String::from_utf8(buf.to_vec()).unwrap()
    }
}


/*
#[no_mangle]
pub unsafe extern "C" fn set_startup_hook(hook: fn () -> Option<()>) {
    let _guard = GLOBAL_LOCK.lock().unwrap();

    unsafe {
        client_startup_hook = Some(hook);
        externals::rl_startup_hook = startup_shim;
    }
}

extern fn startup_shim() -> c_int {
    match unsafe { client_startup_hook } {
        Some(hook) => {
            match hook() {
                Some(()) => 0,
                None     => -1,
            }
        }
        None       => -1,
    }
}
*/