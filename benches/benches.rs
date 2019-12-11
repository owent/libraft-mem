#![allow(dead_code)] // Due to criterion we need this to avoid warnings.
#![cfg_attr(feature = "cargo-clippy", allow(clippy::let_and_return))] // Benches often artificially return values. Allow it.

use criterion::Criterion;
use std::time::Duration;

mod suites;

fn main() {
    let c = Criterion::default()
        // Configure defaults before overriding with args.
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1))
        .configure_from_args();

    // suites::bench_raft(&mut c);

    c.final_summary();
}
