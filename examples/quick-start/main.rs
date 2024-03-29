#[macro_use]
extern crate slog;
extern crate libraft_mem;

use slog::Drain;

// A simple example about how to use the Raft library in Rust.
fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain)
        .chan_size(4096)
        .overflow_strategy(slog_async::OverflowStrategy::Block)
        .build()
        .fuse();
    let logger = slog::Logger::root(drain, o!("tag" => format!("[{}]", 1)));

    info!(logger, "Start run");

    let ctx = libraft_mem::export::libraft_mem_api_create_context();
    libraft_mem::export::libraft_mem_api_destroy_context(ctx);
}
