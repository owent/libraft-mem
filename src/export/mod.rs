// https://doc.rust-lang.org/nomicon/ffi.html

// use std::panic::catch_unwind;

mod error;

use error::{libraft_mem_error, LibRaftMemResult};

#[allow(dead_code)]
pub(crate) fn sanitize_ptr_for_mut_ref<'a, T>(ptr: *mut T) -> LibRaftMemResult<&'a mut T> {
    if ptr.is_null() {
        return Err(libraft_mem_error::LIBRAFT_MEM_ERROR_NULL_PTR);
    }
    let obj_ref: &mut T = unsafe { &mut *ptr };
    Ok(obj_ref)
}

#[allow(dead_code)]
pub(crate) fn sanitize_ptr_for_const_ref<'a, T>(ptr: *const T) -> LibRaftMemResult<&'a T> {
    let ptr = ptr as *mut T;
    sanitize_ptr_for_mut_ref(ptr).map(|r| r as &'a T)
}

#[repr(C)]
pub struct libraft_mem_context {}

#[no_mangle]
pub extern "C" fn libraft_mem_api_create_context() -> *mut libraft_mem_context {
    Box::into_raw(Box::new(libraft_mem_context {})) as *mut libraft_mem_context
}

#[no_mangle]
pub extern "C" fn libraft_mem_api_destroy_context(stack_ptr: *mut libraft_mem_context) {
    if !stack_ptr.is_null() {
        let _ = unsafe { Box::from_raw(stack_ptr) };
    }
}
