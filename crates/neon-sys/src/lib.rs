use std::os::raw::c_void;
use std::mem;
use std::ptr::null_mut;

/// A V8 `Local` handle.
///
/// `Local` handles get associated to a V8 `HandleScope` container. Note: Node.js creates a
/// `HandleScope` right before calling functions in native addons.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Local {
    pub handle: *mut c_void
}

/// Represents the details of how the function was called from JavaScript.
///
/// It contains the arguments used to invoke the function, the isolate reference, the `this` object
/// the function is bound to and a mechanism to return a value to the caller.
pub type FunctionCallbackInfo = c_void;

/// Represents an instance of the V8 runtime.
pub type Isolate = c_void;

const HANDLE_SCOPE_SIZE: usize = 24;

/// A V8 `HandleScope`.
///
/// A `HandleScope` contains `Local` handles. `HandleScope`s are used by V8 to help the garbage
/// collector do its bookkeeping. Once a new `HandleScope` is created all subsequently created
/// `Local` handles will be contained in it.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HandleScope {
    pub align_to_pointer: [*mut c_void; 0],
    pub fields: [u8; HANDLE_SCOPE_SIZE]
}

impl HandleScope {
    pub fn new() -> HandleScope { unsafe { mem::zeroed() } }
}

const ESCAPABLE_HANDLE_SCOPE_SIZE: usize = 32;

/// A V8 `EscapableHandleScope`.
///
/// A `EscapableHandleScope` is like `HandleScope` but also allows us to push `Local` handles out
/// to the previous `HandleScope`, permitting the `Local` value to remain rooted longer than the
/// `EscapableHandleScope` where it was intially rooted.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EscapableHandleScope {
    pub align_to_pointer: [*mut c_void; 0],
    pub fields: [u8; ESCAPABLE_HANDLE_SCOPE_SIZE]
}

impl EscapableHandleScope {
    pub fn new() -> EscapableHandleScope { unsafe { mem::zeroed() } }
}

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

#[derive(Clone, Copy)]
pub struct InheritedHandleScope;

extern "C" {

    pub fn Neon_Array_New(out: &mut Local, isolate: *mut Isolate, length: u32);
    pub fn Neon_Array_Length(array: Local) -> u32;

    pub fn Neon_ArrayBuffer_New(out: &mut Local, isolate: *mut c_void, size: u32) -> bool;
    pub fn Neon_ArrayBuffer_Data<'a, 'b>(base_out: &'a mut *mut c_void, obj: Local) -> usize;

    pub fn Neon_Buffer_New(out: &mut Local, size: u32) -> bool;
    pub fn Neon_Buffer_Uninitialized(out: &mut Local, size: u32) -> bool;
    pub fn Neon_Buffer_Data<'a, 'b>(base_out: &'a mut *mut c_void, obj: Local) -> usize;

    pub fn Neon_Call_SetReturn(info: &FunctionCallbackInfo, value: Local);
    pub fn Neon_Call_GetIsolate(info: &FunctionCallbackInfo) -> *mut Isolate;
    pub fn Neon_Call_CurrentIsolate() -> *mut Isolate;
    pub fn Neon_Call_IsConstruct(info: &FunctionCallbackInfo) -> bool;
    pub fn Neon_Call_This(info: &FunctionCallbackInfo, out: &mut Local);
    pub fn Neon_Call_Data(info: &FunctionCallbackInfo, out: &mut Local);
    pub fn Neon_Call_Length(info: &FunctionCallbackInfo) -> i32;
    pub fn Neon_Call_Get(info: &FunctionCallbackInfo, i: i32, out: &mut Local);

    pub fn Neon_Class_GetClassMap(isolate: *mut Isolate) -> *mut c_void;
    pub fn Neon_Class_SetClassMap(isolate: *mut Isolate, map: *mut c_void, free_map: *mut c_void);
    pub fn Neon_Class_CreateBase(isolate: *mut Isolate,
                                 allocate: CCallback,
                                 construct: CCallback,
                                 call: CCallback,
                                 drop: extern "C" fn(*mut c_void)) -> *mut c_void;
    pub fn Neon_Class_GetName<'a>(base_out: &'a mut *mut u8, isolate: *mut Isolate, metadata: *const c_void) -> usize;
    pub fn Neon_Class_SetName(isolate: *mut Isolate, metadata: *mut c_void, name: *const u8, byte_length: u32) -> bool;
    pub fn Neon_Class_ThrowCallError(isolate: *mut Isolate, metadata: *mut c_void);
    pub fn Neon_Class_ThrowThisError(isolate: *mut Isolate, metadata: *mut c_void);
    pub fn Neon_Class_AddMethod(isolate: *mut Isolate, metadata: *mut c_void, name: *const u8, byte_length: u32, method: Local) -> bool;
    pub fn Neon_Class_MetadataToConstructor(out: &mut Local, isolate: *mut Isolate, metadata: *mut c_void) -> bool;
    pub fn Neon_Class_GetAllocateKernel(obj: Local) -> *mut c_void;
    pub fn Neon_Class_GetConstructKernel(obj: Local) -> *mut c_void;
    pub fn Neon_Class_GetCallKernel(obj: Local) -> *mut c_void;
    pub fn Neon_Class_Constructor(out: &mut Local, ft: Local) -> bool;
    pub fn Neon_Class_HasInstance(metadata: *mut c_void, v: Local) -> bool;
    pub fn Neon_Class_GetInstanceInternals(obj: Local) -> *mut c_void;

    pub fn Neon_Convert_ToObject(out: &mut Local, value: &Local) -> bool;
    pub fn Neon_Convert_ToString(out: &mut Local, value: Local) -> bool;

    pub fn Neon_Error_Throw(val: Local);
    pub fn Neon_Error_NewError(out: &mut Local, msg: Local);
    pub fn Neon_Error_NewTypeError(out: &mut Local, msg: Local);
    pub fn Neon_Error_NewRangeError(out: &mut Local, msg: Local);
    pub fn Neon_Error_ThrowErrorFromUtf8(msg: *const u8, len: i32);

    pub fn Neon_Fun_New(out: &mut Local, isolate: *mut c_void, callback: CCallback) -> bool;
    pub fn Neon_Fun_Template_New(out: &mut Local, isolate: *mut c_void, callback: CCallback) -> bool;
    pub fn Neon_Fun_GetDynamicCallback(obj: Local) -> *mut c_void;
    pub fn Neon_Fun_Call(out: &mut Local, isolate: *mut c_void, fun: Local, this: Local, argc: i32, argv: *mut c_void) -> bool;
    pub fn Neon_Fun_Construct(out: &mut Local, isolate: *mut c_void, fun: Local, argc: i32, argv: *mut c_void) -> bool;

    pub fn Neon_Mem_SameHandle(h1: Local, h2: Local) -> bool;

    pub fn Neon_Module_ExecKernel(kernel: *mut c_void, callback: extern fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void), exports: Local, scope: *mut c_void, vm: *mut c_void);
    pub fn Neon_Module_ExecCallback(callback: CCallback, exports: Local, vm: *mut c_void);
    pub fn Neon_Module_GetVersion() -> i32;

    pub fn Neon_Object_New(out: &mut Local);
    pub fn Neon_Object_GetOwnPropertyNames(out: &mut Local, object: Local) -> bool;
    pub fn Neon_Object_GetIsolate(obj: Local) -> *mut Isolate;
    pub fn Neon_Object_Get_Index(out: &mut Local, object: Local, index: u32) -> bool;
    pub fn Neon_Object_Set_Index(out: &mut bool, object: Local, index: u32, val: Local) -> bool;
    pub fn Neon_Object_Get_String(out: &mut Local, object: Local, key: *const u8, len: i32) -> bool;
    pub fn Neon_Object_Set_String(out: &mut bool, object: Local, key: *const u8, len: i32, val: Local) -> bool;
    pub fn Neon_Object_Get(out: &mut Local, object: Local, key: Local) -> bool;
    pub fn Neon_Object_Set(out: &mut bool, object: Local, key: Local, val: Local) -> bool;

    pub fn Neon_Primitive_Undefined(out: &mut Local);
    pub fn Neon_Primitive_Null(out: &mut Local);
    pub fn Neon_Primitive_Boolean(out: &mut Local, b: bool);
    pub fn Neon_Primitive_BooleanValue(p: Local) -> bool;
    pub fn Neon_Primitive_Integer(out: &mut Local, isolate: *mut Isolate, x: i32);
    pub fn Neon_Primitive_IsUint32(p: Local) -> bool;
    pub fn Neon_Primitive_IsInt32(p: Local) -> bool;
    pub fn Neon_Primitive_IntegerValue(p: Local) -> i64;
    pub fn Neon_Primitive_Number(out: &mut Local, isolate: *mut Isolate, v: f64);
    pub fn Neon_Primitive_NumberValue(p: Local) -> f64;

    pub fn Neon_Scope_Escape(out: &mut Local, scope: *mut EscapableHandleScope, value: Local);
    pub fn Neon_Scope_Chained(out: *mut c_void, closure: *mut c_void, callback: extern fn(&mut c_void, *mut c_void, *mut c_void, *mut c_void), parent_scope: *mut c_void);
    pub fn Neon_Scope_Nested(out: *mut c_void, closure: *mut c_void, callback: extern fn(&mut c_void, *mut c_void, *mut c_void), realm: *mut c_void);
    pub fn Neon_Scope_Enter(scope: &mut HandleScope, isolate: *mut c_void);
    pub fn Neon_Scope_Exit(scope: &mut HandleScope);
    pub fn Neon_Scope_Enter_Escapable(scope: &mut EscapableHandleScope, isolate: *mut c_void);
    pub fn Neon_Scope_Exit_Escapable(scope: &mut EscapableHandleScope);
    pub fn Neon_Scope_Sizeof() -> usize;
    pub fn Neon_Scope_Alignof() -> usize;
    pub fn Neon_Scope_SizeofEscapable() -> usize;
    pub fn Neon_Scope_AlignofEscapable() -> usize;
    pub fn Neon_Scope_GetGlobal(isolate: *mut c_void, out: &mut Local);

    pub fn Neon_String_New(out: &mut Local, isolate: *mut Isolate, data: *const u8, len: i32) -> bool;
    pub fn Neon_String_Utf8Length(str: Local) -> isize;
    pub fn Neon_String_Data(out: *mut u8, len: isize, str: Local) -> isize;

    pub fn Neon_Tag_IsUndefined(val: Local) -> bool;
    pub fn Neon_Tag_IsNull(val: Local) -> bool;
    pub fn Neon_Tag_IsNumber(val: Local) -> bool;
    pub fn Neon_Tag_IsBoolean(val: Local) -> bool;
    pub fn Neon_Tag_IsString(val: Local) -> bool;
    pub fn Neon_Tag_IsObject(val: Local) -> bool;
    pub fn Neon_Tag_IsArray(val: Local) -> bool;
    pub fn Neon_Tag_IsFunction(val: Local) -> bool;
    pub fn Neon_Tag_IsError(val: Local) -> bool;
    pub fn Neon_Tag_IsBuffer(obj: Local) -> bool;
    pub fn Neon_Tag_IsArrayBuffer(obj: Local) -> bool;

    pub fn Neon_Task_Schedule(task: *mut c_void,
                              perform: unsafe extern fn(*mut c_void) -> *mut c_void,
                              complete: unsafe extern fn(*mut c_void, *mut c_void, &mut Local),
                              callback: Local);

}
