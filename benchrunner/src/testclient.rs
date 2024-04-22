use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{collections::HashMap, marker::PhantomData, sync::{Arc, Mutex}, time::Duration};

use data_structures::interfaces::{GetType, KeyType, Tree};

pub trait TestTree<T: KeyType> = Tree<T, T> + Sized;

pub struct ReportEntry {
    pub metric: String,
    pub size: usize,
    pub elapsed: Duration,
}

impl ReportEntry {
    pub fn new(metric: &str, size: usize, elapsed: Duration) -> Self {
        Self {
            metric: metric.to_string(),
            size,
            elapsed,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "{}: {}, {}_per_sec: {}",
            self.metric,
            self.size,
            self.metric,
            self.size as f64 / self.elapsed.as_secs_f64()
        )
    }
}

pub trait TestClient<D, T: TestTree<D>> {
    fn new() -> Self;
    fn put(&self, key: D, value: D);
    fn get_check(&self, key: D, value: D);
    fn get_check_absent(&self, key: D);

    fn id(&self) -> usize {
        0
    }
    fn notice(&self, msg: &str) {
        println!("{}", msg)
    }
    fn report(&self, base: &str, num: usize, cost: Duration) {
        println!("{}", ReportEntry::new(base, num, cost).format());
    }
    fn wait(&self) {}
    fn end(&self) {}
}

pub static STAT_MAP: Lazy<Mutex<HashMap<String, Vec<(usize, ReportEntry)>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub static WAIT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub trait MultiThreadShmClient<D, T: TestTree<D>>: TestClient<D, T> {
    fn new_multithread(id: usize, nthreads: usize) -> Self;
    fn set_tree(&mut self, tree: Arc<T>);
    fn get_tree(&self) -> &T;
    fn nthreads(&self) -> usize;
}

pub struct MultiThreadClient<D, T: TestTree<D>> {
    _tree: Option<Arc<T>>,
    nthreads: usize,
    thread_id: usize,
    _phantom_d: PhantomData<D>,
    _phantom_t: PhantomData<T>,
}

impl<D, T: TestTree<D>> MultiThreadShmClient<D, T> for MultiThreadClient<D, T> {
    fn new_multithread(id: usize, nthreads: usize) -> Self {
        MultiThreadClient {
            _tree: None,
            nthreads,
            thread_id: id,
            _phantom_d: PhantomData,
            _phantom_t: PhantomData,
        }
    }

    fn nthreads(&self) -> usize {
        self.nthreads
    }

    fn set_tree(&mut self, tree: Arc<T>) {
        self._tree = Some(tree)
    }

    fn get_tree(&self) -> &T {
        self._tree.as_ref().unwrap().as_ref()
    }
}

impl<D, T: TestTree<D>> TestClient<D, T> for MultiThreadClient<D, T> {
    fn id(&self) -> usize {
        self.thread_id
    }

    #[inline]
    fn put(&self, key: D, value: D) {
        self.get_tree().put(key, value);
    }

    #[inline]
    fn get_check(&self, key: D, value: D) {
        match T::GET_TYPE {
            GetType::GetVal => assert_eq!(self.get_tree().get_val(key).expect("key not found"), value),
            GetType::GetRef => assert_eq!(self.get_tree().get(key).expect("key not found"), &value)
        };
    }

    fn get_check_absent(&self, key: D) {
        match T::GET_TYPE {
            GetType::GetVal => assert!(self.get_tree().get_val(key).is_none()),
            GetType::GetRef => assert!(self.get_tree().get(key).is_none())
        };
    }

    fn new() -> Self {
        panic!("MultiThreadClient::new() should not be called");
    }

    fn notice(&self, msg: &str) {
        println!("[{}]: {}", self.id(), msg);
    }

    fn report(&self, base: &str, num: usize, cost: std::time::Duration) {
        let mut stat_map = STAT_MAP.lock().unwrap();
        let entry = ReportEntry::new(base, num, cost);
        let v = stat_map.entry(base.to_string()).or_insert(vec![]);
        v.push((self.id(), entry));
    }

    fn wait(&self) {
        WAIT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut curr = WAIT.load(std::sync::atomic::Ordering::SeqCst);
        while curr % self.nthreads() != 0 {
            std::thread::yield_now();
            curr = WAIT.load(std::sync::atomic::Ordering::SeqCst);
        }
    }

    fn end(&self) {
        let mut stat_map = STAT_MAP.lock().unwrap();

        for metric in stat_map.keys().sorted() {
            let entries = &stat_map[metric];
            let mut total_cost = std::time::Duration::new(0, 0);
            let mut total_num = 0;
            let num_thread = entries.len();
            let mut max_time_cost = std::time::Duration::new(0, 0);

            for (id, entry) in entries.iter() {
                total_num += entry.size;
                total_cost += entry.elapsed;
                if max_time_cost < entry.elapsed {
                    max_time_cost = entry.elapsed;
                }
                // println!("[{id}]: {}", entry.format());
            }

            let total_throughput = total_num as f64 / max_time_cost.as_secs_f64();
            let avg_throughput = total_num as f64 / total_cost.as_secs_f64();
            println!(
                "metric: {}, #threads: {}, total throughput: {}, avg throughput: {}",
                metric, num_thread, total_throughput, avg_throughput,
            );
        }
        stat_map.clear();
    }
}
