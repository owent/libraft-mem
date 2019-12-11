extern crate libraft_mem;

#[test]
fn export_create_and_destroy() {
    let ctx = libraft_mem::export::libraft_mem_api_create_context();
    assert!(!ctx.is_null());
    libraft_mem::export::libraft_mem_api_destroy_context(ctx);
}
