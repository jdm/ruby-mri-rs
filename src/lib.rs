extern crate libc;

#[allow(dead_code, non_snake_case, non_upper_case_globals, non_camel_case_types)]
mod ffi;

use self::ffi::*;
use std::ffi::CString;

pub struct Value(VALUE);

impl Value {
    pub fn from(v: VALUE) -> Value {
        Value(v)
    }

    pub fn is_nil(&self) -> bool {
        NIL_P(self.0)
    }

    pub fn is_string(&self) -> bool {
        RB_TYPE_P(self.0, T_STRING)
    }

    pub fn is_fixnum(&self) -> bool {
        RB_TYPE_P(self.0, T_FIXNUM)
    }

    pub fn is_symbol(&self) -> bool {
        RB_TYPE_P(self.0, T_SYMBOL)
    }

    pub fn is_float(&self) -> bool {
        RB_TYPE_P(self.0, T_FLOAT)
    }

    pub fn is_object(&self) -> bool {
        RB_TYPE_P(self.0, T_OBJECT)
    }
}

pub struct RubyVM;

impl RubyVM {
    pub fn new() -> RubyVM {
        unsafe {
            ruby_init();
        }

        RubyVM
    }

    pub fn eval(s: &str) -> Result<Value, ()> {
        let mut state = 0;
        let s = try!(CString::new(s).map_err(|_| ()));
        unsafe {
            let result = rb_eval_string_protect(s.as_ptr(), &mut state);
            if state != 0 {
                return Err(());
            }
            Ok(Value::from(result))
        }
    }
}

impl Drop for RubyVM {
    fn drop(&mut self) {
        unsafe {
            ruby_cleanup(0)
        }
    }
}

#[test]
fn it_works() {
    let _ = RubyVM::new();
}
