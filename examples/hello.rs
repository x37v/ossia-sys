use ossia_sys::*;
use std::ffi::CString;

fn main() {
    unsafe {
        let proto = ossia_protocol_oscquery_server_create(1234, 5678);
        let dev = ossia_device_create(proto, CString::new("supersoftware").unwrap().as_ptr());
        let root = ossia_device_get_root_node(dev);
        let a_node = ossia_node_create(root, CString::new("/foo/blu").unwrap().as_ptr());
    }
    loop {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
