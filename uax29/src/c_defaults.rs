//! This module provides a wrapper for the word break iterator over a
//! string. The iterator follows the default UAX #29 word-breaking
//! specification.

extern crate libc;

use std::{ffi, mem, ptr, sync};

use breaks::word_break::Category;
use defaults;

static GLOBAL_LOCK: sync::StaticMutex = sync::MUTEX_INIT;

lazy_static! {
    static ref STRINGS: sync::RwLock<Vec<String>> = sync::RwLock::new(vec![]);
    static ref WORD_BREAKS: sync::RwLock<Vec<Box<()>>> =
        sync::RwLock::new(vec![]);
}

#[no_mangle]
pub unsafe extern "C" fn create_word_breaker(txt: *const libc::c_char)
    -> libc::c_ulong
{
    let _guard = GLOBAL_LOCK.lock().unwrap();
    let buf = ffi::c_str_to_bytes(&txt);
    let input = String::from_utf8(buf.to_vec()).unwrap();
    let mut vec_guard = STRINGS.write().unwrap();
    vec_guard.push(input);
    let input = vec_guard.last().unwrap();
    let wb = Box::new(defaults::Breaks::new(input.as_slice(),
                                            defaults::make_word_break_tree()));
    let mut vec_guard = WORD_BREAKS.write().unwrap();
    vec_guard.push(
        mem::transmute::<Box<defaults::Breaks<Category>>, Box<()>>(wb));
    vec_guard.len() as libc::c_ulong - 1
}

#[no_mangle]
pub unsafe extern "C" fn next_word(wb: libc::c_ulong) -> *const libc::c_char {
    let _guard = GLOBAL_LOCK.lock().unwrap();
    let mut vec_guard = WORD_BREAKS.write().unwrap();
    let boxed: &mut Box<()> = vec_guard.get_mut(wb as usize).unwrap();
    let mut wb: &mut Box<defaults::Breaks<Category>> =
        mem::transmute::<&mut Box<()>, &mut Box<defaults::Breaks<Category>>>(
        boxed);
    match wb.next() {
        Some(s) => ffi::CString::from_slice(s.as_bytes()).as_ptr(),
        None => ptr::null()
    }
}

#[cfg(test)]
mod test_cpp_wrapper {
    extern crate libc;

    use std::{ffi, ptr};
    use super::{create_word_breaker, next_word, WordBreakerPtr};

    #[test]
    fn test_iterate() {
        unsafe {
            let txt = ffi::CString::from_slice("1.21 gigawatts.".as_bytes());
            let p: WordBreakerPtr = create_word_breaker(txt.as_ptr());
            assert_eq!(c_char_to_str(next_word(p)), "1.21");
            assert_eq!(c_char_to_str(next_word(p)), " ");
            assert_eq!(c_char_to_str(next_word(p)), "gigawatts");
            assert_eq!(c_char_to_str(next_word(p)), ".");
            assert_eq!(next_word(p), ptr::null());
        }
    }

    unsafe fn c_char_to_str(txt: *const libc::c_char) -> String {
        let buf = ffi::c_str_to_bytes(&txt);
        String::from_utf8(buf.to_vec()).unwrap()
    }
}
