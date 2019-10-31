use std::os::raw::c_void;
use std::ptr::null_mut;
use raw::{FunctionCallbackInfo, Isolate, Local};

#[repr(C)]
pub struct CCallback {
    pub static_callback: *mut c_void,
    pub dynamic_callback: *mut c_void
}

impl Default for CCallback {
    fn default() -> Self {
        CCallback {
            static_callback: null_mut(),
            dynamic_callback: null_mut()
        }
    }
}

pub unsafe extern "C" fn set_return(_info: &FunctionCallbackInfo, _value: Local) { unimplemented!() }

pub unsafe extern "C" fn get_isolate(_info: &FunctionCallbackInfo) -> *mut Isolate { unimplemented!() }

pub unsafe extern "C" fn current_isolate() -> *mut Isolate { unimplemented!() }

pub unsafe extern "C" fn is_construct(_info: &FunctionCallbackInfo) -> bool { unimplemented!() }

pub unsafe extern "C" fn this(_info: &FunctionCallbackInfo, _out: &mut Local) { unimplemented!() }

pub unsafe extern "C" fn data(_info: &FunctionCallbackInfo, _out: &mut Local) { unimplemented!() }

pub unsafe extern "C" fn len(_info: &FunctionCallbackInfo) -> i32 { unimplemented!() }

pub unsafe extern "C" fn get(_info: &FunctionCallbackInfo, _i: i32, _out: &mut Local) { unimplemented!() }
