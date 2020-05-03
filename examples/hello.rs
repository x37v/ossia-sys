use ossia_sys::*;
use std::ffi::CString;

fn main() {
    unsafe {
        let proto = ossia_protocol_oscquery_server_create(1234, 5678);
        let dev = ossia_device_create(proto, CString::new("supersoftware").unwrap().as_ptr());
        let root = ossia_device_get_root_node(dev);
        let node = ossia_node_create(root, CString::new("/foo").unwrap().as_ptr());
        let p = ossia_create_parameter(
            node,
            CString::new("bar").unwrap().as_ptr(),
            CString::new("vec3").unwrap().as_ptr(),
        );
        ossia_parameter_push_3f(p, 1f32, 2f32, 3f32);
        let p = ossia_create_parameter(
            node,
            CString::new("baz").unwrap().as_ptr(),
            CString::new("i32").unwrap().as_ptr(),
        );
        ossia_parameter_push_i(p, 2084i32);
    }
    loop {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
