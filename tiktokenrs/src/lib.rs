use std::os::raw::c_char;
use std::ffi::{CStr, CString};

use tiktoken_rs::CoreBPE;
use anyhow::{Result, anyhow};
use std::sync::Arc;
use parking_lot::Mutex;
use tiktoken_rs::tokenizer::{Tokenizer, get_tokenizer};

// We need to receive the data from the C ling so rust can understand
// This function is "public" any another language can read it.
// #[no_mangle] means that when the program be compiled, the name of the function will be preserved
#[no_mangle]
pub extern "C" fn hello_to_my_name(name: *const c_char) -> *mut c_char {
    // converting the name data from C to Rust string;
    let name = unsafe { CStr::from_ptr(name).to_str().unwrap() };
    let result = format!("Hello, {}", name);
    let result = CString::new(result).unwrap();
    
    // converting back to C;
    return result.into_raw();
}

// With the `Tokenizer` we can figure it out what is its BPE.
// Atomic Reference Counting(Arc) it works in thread safe to count any thing. It does not affect others threads.
// Mutual Exclusion(Mutex) Only one thread can change a data at a time. 
pub fn get_token_bpe(t: Tokenizer) -> Result<Arc<Mutex<CoreBPE>>> {
    let token_bpe = match t {
        Tokenizer::Cl100kBase => tiktoken_rs::cl100k_base_singleton(),
        Tokenizer::P50kBase => tiktoken_rs::p50k_base_singleton(),
        Tokenizer::R50kBase => tiktoken_rs::r50k_base_singleton(),
        Tokenizer::P50kEdit => tiktoken_rs::p50k_edit_singleton(),
        Tokenizer::Gpt2 => tiktoken_rs::r50k_base_singleton()
    };

    Ok(token_bpe)
}

#[no_mangle]
pub extern "C" fn get_qtd_tokens(model_name: *const libc::c_char, txt: *const libc::c_char) -> libc::c_uint {
    let model_name = unsafe {CStr::from_ptr(model_name).to_str().unwrap()};
    let txt = unsafe {CStr::from_ptr(txt).to_str().unwrap()};
    // if it gets an error it will return the message `Model {} does not exists` other else it will return the `Tokenizer`.
    let tokenizer = get_tokenizer(model_name).ok_or_else(||anyhow!("Model {} does not exists", model_name));

    if tokenizer.is_err() {
        return 0 as libc::c_uint
    }

    let bpe = get_token_bpe(tokenizer.unwrap()).unwrap();
    // bpe.lock() it locks the thread
    let result = bpe.lock().encode_with_special_tokens(txt).len();

    return result as libc::c_uint;
}

mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_get_qtd_tokens() {
        let model_name = CString::new("gpt-3.5-turbo").unwrap();
        let txt = CString::new("Hello World").unwrap();

        assert_eq!(get_qtd_tokens(model_name.as_ptr(), txt.as_ptr()), 2);

        let model_name = CString::new("nothing").unwrap();
        let txt = CString::new("Hello World").unwrap();

        assert_eq!(get_qtd_tokens(model_name.as_ptr(), txt.as_ptr()), 0);
    }
}