use libc;
use std::mem;

#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
pub struct VALUE(libc::size_t);
#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
pub struct ID(libc::size_t);

#[repr(C)]
struct RBasic {
    flags: VALUE,
    klass: VALUE,
}

/*#[repr(C)]
struct RObject {
    basic: RBasic,
    
}

#[repr(C)]
struct RClass {
    basic: RBasic,
    super: VALUE,
    ptr: *const rb_classext_t,
    m_tbl_wrapper: *const method_table_wrapper,
}*/

extern "C" {
    pub fn ruby_init();
    pub fn ruby_cleanup(code: i32);
    pub fn ruby_setup() -> i32;
    pub fn ruby_finalize();
    pub fn ruby_stop(code: i32) -> !;

    pub fn ruby_script(name: *const libc::c_char);

    pub fn rb_eval_string(s: *const libc::c_char) -> VALUE;
    pub fn rb_eval_string_protect(s: *const libc::c_char, state: *mut i32) -> VALUE;
    pub fn rb_eval_string_wrap(s: *const libc::c_char, state: *mut i32) -> VALUE;
    pub fn rb_funcall(env: VALUE, name: ID, argc: i32, ...);
    pub fn rb_funcallv(env: VALUE, name: ID, argc: i32, argv: *const VALUE);
    pub fn rb_funcallv_public(env: VALUE, name: ID, argc: i32, argv: *const VALUE);

    pub fn rb_sym2id(symbol: VALUE) -> ID;
    pub fn rb_id2sym(id: ID) -> VALUE;

    pub fn rb_type(obj: VALUE) -> ruby_value_type;
    pub fn rb_check_type(obj: VALUE, type_: i32);

    pub fn rb_str_new_cstr(s: *const libc::c_char) -> VALUE;
    pub fn rb_sprintf(fmt: *const libc::c_char, ...) -> VALUE;

    pub fn rb_intern(s: *const libc::c_char) -> ID;
    pub fn rb_intern2(s: *const libc::c_char, l: i32) -> ID;
    pub fn rb_intern_str(s: VALUE) -> ID;
    pub fn rb_id2name(id: ID) -> *const libc::c_char;
    pub fn rb_to_id(v: VALUE) -> ID;
    pub fn rb_id2str(id: ID) -> VALUE;
    pub fn rb_sym2str(id: ID) -> VALUE;
    pub fn rb_to_symbol(name: VALUE) -> VALUE;

    pub fn rb_class2name(v: VALUE) -> *const libc::c_char;
    pub fn rb_obj_classname(v: VALUE) -> *const libc::c_char;

    pub fn rb_p(v: VALUE);
}

#[repr(i32)]
#[derive(PartialEq)]
pub enum ruby_value_type {
    RUBY_T_NONE   = 0x00,

    RUBY_T_OBJECT = 0x01,
    RUBY_T_CLASS  = 0x02,
    RUBY_T_MODULE = 0x03,
    RUBY_T_FLOAT  = 0x04,
    RUBY_T_STRING = 0x05,
    RUBY_T_REGEXP = 0x06,
    RUBY_T_ARRAY  = 0x07,
    RUBY_T_HASH   = 0x08,
    RUBY_T_STRUCT = 0x09,
    RUBY_T_BIGNUM = 0x0a,
    RUBY_T_FILE   = 0x0b,
    RUBY_T_DATA   = 0x0c,
    RUBY_T_MATCH  = 0x0d,
    RUBY_T_COMPLEX  = 0x0e,
    RUBY_T_RATIONAL = 0x0f,

    RUBY_T_NIL    = 0x11,
    RUBY_T_TRUE   = 0x12,
    RUBY_T_FALSE  = 0x13,
    RUBY_T_SYMBOL = 0x14,
    RUBY_T_FIXNUM = 0x15,

    RUBY_T_UNDEF  = 0x1b,
    RUBY_T_NODE   = 0x1c,
    RUBY_T_ICLASS = 0x1d,
    RUBY_T_ZOMBIE = 0x1e,

    RUBY_T_MASK   = 0x1f
}

pub const T_NONE: ruby_value_type = ruby_value_type::RUBY_T_NONE;
pub const T_NIL: ruby_value_type = ruby_value_type::RUBY_T_NIL;
pub const T_OBJECT: ruby_value_type = ruby_value_type::RUBY_T_OBJECT;
pub const T_CLASS: ruby_value_type = ruby_value_type::RUBY_T_CLASS;
pub const T_ICLASS: ruby_value_type = ruby_value_type::RUBY_T_ICLASS;
pub const T_MODULE: ruby_value_type = ruby_value_type::RUBY_T_MODULE;
pub const T_FLOAT: ruby_value_type = ruby_value_type::RUBY_T_FLOAT;
pub const T_STRING: ruby_value_type = ruby_value_type::RUBY_T_STRING;
pub const T_REGEXP: ruby_value_type = ruby_value_type::RUBY_T_REGEXP;
pub const T_ARRAY: ruby_value_type = ruby_value_type::RUBY_T_ARRAY;
pub const T_HASH: ruby_value_type = ruby_value_type::RUBY_T_HASH;
pub const T_STRUCT: ruby_value_type = ruby_value_type::RUBY_T_STRUCT;
pub const T_BIGNUM: ruby_value_type = ruby_value_type::RUBY_T_BIGNUM;
pub const T_FILE: ruby_value_type = ruby_value_type::RUBY_T_FILE;
pub const T_FIXNUM: ruby_value_type = ruby_value_type::RUBY_T_FIXNUM;
pub const T_TRUE: ruby_value_type = ruby_value_type::RUBY_T_TRUE;
pub const T_FALSE: ruby_value_type = ruby_value_type::RUBY_T_FALSE;
pub const T_DATA: ruby_value_type = ruby_value_type::RUBY_T_DATA;
pub const T_MATCH: ruby_value_type = ruby_value_type::RUBY_T_MATCH;
pub const T_SYMBOL: ruby_value_type = ruby_value_type::RUBY_T_SYMBOL;
pub const T_RATIONAL: ruby_value_type = ruby_value_type::RUBY_T_RATIONAL;
pub const T_COMPLEX: ruby_value_type = ruby_value_type::RUBY_T_COMPLEX;
pub const T_UNDEF: ruby_value_type = ruby_value_type::RUBY_T_UNDEF;
pub const T_NODE: ruby_value_type = ruby_value_type::RUBY_T_NODE;
pub const T_ZOMBIE: ruby_value_type = ruby_value_type::RUBY_T_ZOMBIE;
pub const T_MASK: ruby_value_type = ruby_value_type::RUBY_T_MASK;

mod ruby_special_consts {
    pub const RUBY_Qfalse: i32 = 0;
    pub const RUBY_Qtrue: i32 = 2;
    pub const RUBY_Qnil: i32 = 4;
    pub const RUBY_Qundef: i32 = 6;

    pub const RUBY_IMMEDIATE_MASK: i32 = 0x03;
    pub const RUBY_FIXNUM_FLAG: i32 = 0x01;	/* ...xxxx xxx1 */
    pub const RUBY_FLONUM_MASK: i32 = 0x00;	/* any values ANDed with FLONUM_MASK cannot be FLONUM_FLAG */
    pub const RUBY_FLONUM_FLAG: i32 = 0x02;
    pub const RUBY_SYMBOL_FLAG: i32 = 0x0e;	/* ...0000 1110 */

    pub const RUBY_SPECIAL_SHIFT: i32 = 8;
}

const Qfalse: VALUE = VALUE(ruby_special_consts::RUBY_Qfalse as libc::size_t);
const Qtrue: VALUE = VALUE(ruby_special_consts::RUBY_Qtrue as libc::size_t);
const Qnil: VALUE = VALUE(ruby_special_consts::RUBY_Qnil as libc::size_t);
const Qundef: VALUE = VALUE(ruby_special_consts::RUBY_Qundef as libc::size_t);
const IMMEDIATE_MASK: libc::size_t = ruby_special_consts::RUBY_IMMEDIATE_MASK as libc::size_t;
const FIXNUM_FLAG: libc::size_t = ruby_special_consts::RUBY_FIXNUM_FLAG as libc::size_t;
const SYMBOL_FLAG: libc::size_t = ruby_special_consts::RUBY_SYMBOL_FLAG as libc::size_t;

pub fn RB_TYPE_P(obj: VALUE, ty: ruby_value_type) -> bool {
    match ty {
        ruby_value_type::RUBY_T_FIXNUM => FIXNUM_P(obj),
        ruby_value_type::RUBY_T_TRUE => obj == Qtrue,
        ruby_value_type::RUBY_T_FALSE => obj == Qfalse,
        ruby_value_type::RUBY_T_NIL => obj == Qnil,
        ruby_value_type::RUBY_T_UNDEF => obj == Qundef,
        ruby_value_type::RUBY_T_SYMBOL => SYMBOL_P(obj),
        ruby_value_type::RUBY_T_FLOAT => RB_FLOAT_TYPE_P(obj),
        _ => !SPECIAL_CONST_P(obj) && BUILTIN_TYPE(obj) == ty
    }
}

pub fn RB_FLOAT_TYPE_P(obj: VALUE) -> bool {
    FLONUM_P(obj) || (!SPECIAL_CONST_P(obj) &&
                      BUILTIN_TYPE(obj) == T_FLOAT)
}

pub fn BUILTIN_TYPE(x: VALUE) -> ruby_value_type {
    unsafe {
        let basic: *const RBasic = mem::transmute(x);
        let masked = (*basic).flags.0 & (T_MASK as libc::size_t);
        mem::transmute(masked as u32)
    }
}

pub fn FLONUM_P(_x: VALUE) -> bool {
    false
    //x & FLONUM_MASK == FLONUM_FLAG
}

pub fn FIXNUM_P(f: VALUE) -> bool {
    (f.0 & FIXNUM_FLAG) != 0
}

pub fn DYNAMIC_SYM_P(_x: VALUE) -> bool {
    false
}

pub fn STATIC_SYM_P(_x: VALUE) -> bool {
    false
}

pub fn SYMBOL_P(x: VALUE) -> bool {
    STATIC_SYM_P(x) || DYNAMIC_SYM_P(x)
}

pub fn SPECIAL_CONST_P(x: VALUE) -> bool {
    IMMEDIATE_P(x) || !RTEST(x)
}

pub fn IMMEDIATE_P(x: VALUE) -> bool {
    (x.0 & IMMEDIATE_MASK) != 0
}

pub fn RTEST(v: VALUE) -> bool {
    (v.0 & !Qnil.0) != 0
}

pub fn NIL_P(v: VALUE) -> bool {
    v == Qnil
}
