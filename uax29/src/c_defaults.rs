
#![crate_type = "dylib"]
extern crate libc;
//use libc::{c_char};
use std::ffi::{CString, c_str_to_bytes};
use std::{mem, sync, ptr};
use defaults;
use breaks::word_break::Category;

static GLOBAL_LOCK: sync::StaticMutex = sync::MUTEX_INIT;
pub type word_breaker_ptr = *const ();

#[no_mangle]
pub unsafe extern "C" fn create_word_breaker(txt: *const libc::c_char) -> word_breaker_ptr {
	let buf = c_str_to_bytes(&txt);
	let input = String::from_utf8(buf.to_vec()).unwrap();
	let wb = Box::new(defaults::Breaks::new(input.as_slice(),
	defaults::make_word_break_tree()));
	mem::transmute(wb)
}

#[no_mangle]
pub unsafe extern "C" fn next_word(wb: word_breaker_ptr) -> *const libc::c_char {
	let wb: &mut defaults::Breaks<Category> = mem::transmute(wb);
	match wb.next() {
		Some(s) => CString::from_slice(s.as_bytes()).as_ptr(),
		None => ptr::null()
	}
}
