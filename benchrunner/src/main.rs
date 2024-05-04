#![feature(trait_alias, generic_const_exprs)]

use crate::{testcases::Testcases, testrunner::multithread_run};
use clap::Parser;
use data_structures::{binary_search_tree, interfaces::Tree};

mod testcases;
mod testclient;
mod testrunner;

#[cfg(feature = "jemalloc")]
use tikv_jemallocator::Jemalloc;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use data_structures::art::DefaultArt;
use data_structures::avl::ConcurrentAVLTree;
use data_structures::bptree::BpTree;
#[cfg(feature = "tcmalloc")]
use tcmalloc::TCMalloc;

#[cfg(feature = "tcmalloc")]
#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

use crate::testcases::TestcasesUsize;
#[cfg(feature = "hoard")]
use hoard_allocator::Hoard;

#[cfg(feature = "hoard")]
#[global_allocator]
static GLOBAL: Hoard = Hoard;

static MALLOC_NOTE: &str = if cfg!(feature = "jemalloc") {
    "jemalloc"
} else if cfg!(feature = "tcmalloc") {
    "tcmalloc"
} else if cfg!(feature = "hoard") {
    "hoard"
} else {
    "default (glibc) malloc"
};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    tree: String,

    #[arg(long)]
    testcase: String,

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

fn run<T: Tree<String, String> + 'static>(args: Args) {
    multithread_run(
        args.num_threads,
        args.size,
        args.pin,
        args.run_name,
        args.run_profiler,
        Testcases::<T>::find(&args.testcase),
    );
}

fn run_usize<T: Tree<usize, usize> + 'static>(args: Args) {
    multithread_run(
        args.num_threads,
        args.size,
        args.pin,
        args.run_name,
        args.run_profiler,
        TestcasesUsize::<T>::find(&args.testcase),
    );
}

fn main() {
    let args = Args::parse();
    println!(
        "Benchmark: test run {} threads, size: {}, pin_to_core?: {}, memory allocator: {}",
        args.num_threads, args.size, args.pin, MALLOC_NOTE
    );
    match args.tree.as_str() {
        "bst" => {
            run::<binary_search_tree::LockFreeBST>(args);
        }
        "skiplist" => {
            run::<data_structures::skiplist::SkipMapWrapper<String, String>>(args);
        }
        "bptree" => {
            run::<BpTree<String, String>>(args);
        }
        "avltree" => {
            run_usize::<ConcurrentAVLTree<usize, usize>>(args);
        }
        "art" => {
            run_usize::<DefaultArt>(args);
        }
        _ => panic!("unknown tree: {}", args.tree),
    }
}
