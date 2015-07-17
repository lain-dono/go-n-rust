extern crate libc;
use std::ffi::{CStr, CString};

extern { fn HelloFromGo(name: *const libc::c_char); }

#[no_mangle]
pub extern "C" fn hello_from_rust(name: *const libc::c_char) {
	let buf_name = unsafe { CStr::from_ptr(name).to_bytes() };
	let str_name = String::from_utf8(buf_name.to_vec()).unwrap();
	println!("go\t: {}", str_name);

	let hello = CString::new("Привет :3 Кстати, тут Nim не пробегал?").unwrap();
	unsafe { HelloFromGo(hello.as_ptr()); }
}
