use raw::{Isolate, Local};

pub unsafe extern "C" fn new(_out: &mut Local) { unimplemented!() }

pub unsafe extern "C" fn get_own_property_names(_out: &mut Local, _object: Local) -> bool { unimplemented!() }

pub unsafe extern "C" fn get_isolate(_obj: Local) -> *mut Isolate { unimplemented!() }

pub unsafe extern "C" fn get_index(_out: &mut Local, _object: Local, _index: u32) -> bool { unimplemented!() }

pub unsafe extern "C" fn set_index(_out: &mut bool, _object: Local, _index: u32, _val: Local) -> bool { unimplemented!() }

pub unsafe extern "C" fn get_string(_out: &mut Local, _object: Local, _key: *const u8, _len: i32) -> bool { unimplemented!() }

pub unsafe extern "C" fn set_string(_out: &mut bool, _object: Local, _key: *const u8, _len: i32, _val: Local) -> bool { unimplemented!() }

pub unsafe extern "C" fn get(_out: &mut Local, _object: Local, _key: Local) -> bool { unimplemented!() }

pub unsafe extern "C" fn set(_out: &mut bool, _object: Local, _key: Local, _val: Local) -> bool { unimplemented!() }
