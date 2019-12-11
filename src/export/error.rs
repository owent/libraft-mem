#[repr(C)]
#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code, non_camel_case_types)]
pub(crate) enum libraft_mem_error {
    LIBRAFT_MEM_ERROR_NONE,
    LIBRAFT_MEM_ERROR_NULL_PTR,
}

#[doc(hidden)]
pub(crate) type LibRaftMemResult<T> = Result<T, libraft_mem_error>;
