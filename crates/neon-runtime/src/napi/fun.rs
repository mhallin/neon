use std::os::raw::c_void;
use call::CCallback;
use raw::Local;

pub unsafe extern "C" fn new(_out: &mut Local, _isolate: *mut c_void, _callback: CCallback) -> bool { unimplemented!() }

pub unsafe extern "C" fn new_template(_out: &mut Local, _isolate: *mut c_void, _callback: CCallback) -> bool { unimplemented!() }

pub unsafe extern "C" fn get_dynamic_callback(_obj: Local) -> *mut c_void { unimplemented!() }

pub unsafe extern "C" fn call(_out: &mut Local, _isolate: *mut c_void, _fun: Local, _this: Local, _argc: i32, _argv: *mut c_void) -> bool { unimplemented!() }

pub unsafe extern "C" fn construct(_out: &mut Local, _isolate: *mut c_void, _fun: Local, _argc: i32, _argv: *mut c_void) -> bool { unimplemented!() }
