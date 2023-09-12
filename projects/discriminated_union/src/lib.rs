//! Simple library-like file with a public-facing API exposing an enum.
//! This enum has C representation of a tagged union.
//! Note that this may not be a great example of doing C FFI,
//! because a number of the types used here aren't FFI-Safe.

use core::ffi::c_char;
use std::{
    collections::HashMap,
    ffi::{c_int, CString},
};

/// An enum representing JSON data.
/// For a production-quality version of this, see
/// [`serde_json::value::Value`].
#[repr(C)]
#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(*const c_char),
    // Array(Vec<Value>),
    // Object(HashMap<CString, Box<Value>>),
}

#[no_mangle]
pub extern "C" fn create_number(number: f64) -> Value {
    Value::Number(number)
}

#[no_mangle]
pub extern "C" fn format_value(value: &Value) -> *const c_char {
    format!("{value:?}").as_ptr().cast()
}

extern "C" {
    fn puts(s: *const c_char) -> c_int;
}

#[no_mangle]
pub extern "C" fn puts_value(value: &Value) {
    let value: *const c_char = format_value(value).cast();
    unsafe { puts(value) };
}

/*
#[no_mangle]
pub extern "C" fn make_value() -> Value {
    Value::Object(
        [
            (
                CString::new("key1".to_string()).unwrap(),
                Box::new(Value::Number(5f64)),
            ),
            (
                CString::new("key22".to_string()).unwrap(),
                Box::new(Value::Array(vec![Value::Bool(false)])),
            ),
        ]
        .into_iter()
        .collect(),
    )
}
 */
