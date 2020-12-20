use ossia_sys::*;
use std::ffi::CString;

fn main() {
    unsafe {
        let proto = ossia_protocol_oscquery_server_create(1234, 5678);
        let dev = ossia_device_create(proto, CString::new("ossia-sys").unwrap().as_ptr());
        let root = ossia_device_get_root_node(dev);
        let p = ossia_create_parameter(
            root,
            CString::new("foo").unwrap().as_ptr(),
            CString::new("float").unwrap().as_ptr(),
        );
        ossia_parameter_push_f(p, 0.5f32);
        let p = ossia_create_parameter(
            root,
            CString::new("/foo/bar").unwrap().as_ptr(),
            CString::new("vec2").unwrap().as_ptr(),
        );
        ossia_parameter_push_2f(p, 1f32, 2f32);

        let p = ossia_create_parameter(
            root,
            CString::new("/foo/unset").unwrap().as_ptr(),
            CString::new("float").unwrap().as_ptr(),
        );
        ossia_parameter_set_access_mode(p, ossia_access_mode::GET);
    }
    loop {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
