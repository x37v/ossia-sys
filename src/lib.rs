//! Automatic ffi bindings for the libossia C API.
//! See [the C API docs](https://ossia.io/libossia/html/group___c_a_p_i.html) for more details.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
