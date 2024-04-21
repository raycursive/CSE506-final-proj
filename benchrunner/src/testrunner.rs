use std::{sync::Arc, thread};

use core_affinity::get_core_ids;
use cpuprofiler::PROFILER;

use crate::testclient::{MultiThreadClient, MultiThreadShmClient, TestTree};

#[allow(dead_code)]
pub fn multithread_run<D, T>(
    nthreads: usize,
    size: usize,
    pin_to_thread: bool,
    run_name: String,
    run_profiler: bool,
    test_fn: fn(&mut MultiThreadClient<D, T>, size: usize) -> (),
) where
    D: 'static,
    T: TestTree<D> + 'static,
    Arc<T>: Send,
{
    let tree = Arc::new(T::new());
    let core_ids = get_core_ids().unwrap()[..(nthreads)].to_vec();

    if run_profiler {
        PROFILER
            .lock()
            .expect("Failed to lock profiler")
            .start(format!("{run_name}_j{nthreads}.profile"))
            .expect("profiler failed to start");
    }

    let handles = core_ids
        .into_iter()
        .map(|core_id| {
            let _tree = tree.clone();
            thread::spawn(move || {
                if pin_to_thread {
                    let res = core_affinity::set_for_current(core_id);
                    if !res {
                        panic!("Failed to set core affinity");
                    }
                }
                let mut client = MultiThreadClient::<D, T>::new_multithread(core_id.id, nthreads);
                client.set_tree(_tree);
                test_fn(&mut client, size / nthreads);
            })
        })
        .collect::<Vec<_>>();
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }

    if run_profiler {
        PROFILER
            .lock()
            .expect("Failed to lock profiler")
            .stop()
            .expect("profiler failed to stop");
    }
}
