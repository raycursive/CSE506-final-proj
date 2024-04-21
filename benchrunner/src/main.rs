#![feature(trait_alias, generic_const_exprs)]

use clap::Parser;
use data_structures::binary_search_tree;
use crate::{
    testcases::Testcases,
    testrunner::multithread_run,
};

mod testclient;
mod testcases;
mod testrunner;

#[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))]
use tikv_jemallocator::Jemalloc;
#[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

static MALLOC_NOTE: &str = if cfg!(feature="jemalloc") {
    "jemalloc"
} else if cfg!(feature="tcmalloc") {
    "tcmalloc"
} else if cfg!(feature="hoard") {
    "hoard"
} else {
    "default (glibc) malloc"
};

#[derive(Parser)]
struct Args {
    #[arg(short = 'p', long, default_value_t = false)]
    pin: bool,

    #[arg(short = 'j', long, default_value = "4")]
    num_threads: usize,

    #[arg(short = 's', long, default_value = "5000000")]
    size: usize,

    #[arg(short = 'n', default_value = "my_test")]
    run_name: String,

    #[arg(long = "profile", default_value_t = false)]
    run_profiler: bool,
}

fn main() {
    let args = Args::parse();
    println!(
        "Benchmark: test run {} threads, size: {}, pin_to_core?: {}, memory allocator: {}",
        args.num_threads, args.size, args.pin, MALLOC_NOTE
    );
    multithread_run(
        args.num_threads,
        args.size,
        args.pin,
        args.run_name,
        args.run_profiler,
        Testcases::<binary_search_tree::LockFreeBST>::find("simple"),
    );
}

