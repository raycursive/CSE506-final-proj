use std::collections::HashSet;
use std::marker::PhantomData;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use data_structures::quick_istr::QuickIStr;

use crate::testclient::{TestClient, TestTree};

pub struct Testcases<T: TestTree<String>> {
    _phantom: PhantomData<T>,
}

impl<T: TestTree<String>> Testcases<T> {
    pub fn find<C: TestClient<String, T>>(name: &str) -> fn(&mut C, usize) {
        match name {
            "simple" => Self::test_simple,
            _ => panic!("unknown test case: {}", name),
        }
    }

    pub fn test_simple<C: TestClient<String, T>>(client: &mut C, n: usize) {
        let mut rng = StdRng::seed_from_u64((12345 + client.id()) as u64);
        let mut keys = HashSet::new();
        while keys.len() < n {
            keys.insert(rng.gen::<u32>());
        }
        let keys: Vec<_> = keys.into_iter().collect();

        let mut t = Instant::now();
        for &key in keys.iter() {
            client.put(
                QuickIStr::new(key as u64).into(),
                QuickIStr::new(key as u64 + 1).into(),
            );
        }
        let put_time_cost = t.elapsed();
        client.report("put", n, put_time_cost);
        client.wait();

        t = Instant::now();
        for &key in keys.iter() {
            client.get_check(
                QuickIStr::new(key as u64).into(),
                QuickIStr::new(key as u64 + 1).into(),
            );
        }
        let get_time_cost = t.elapsed();
        client.report("get", n, get_time_cost);
        client.wait();

        let random = (0..n).map(|_| rng.gen::<bool>()).collect::<Vec<_>>();
        let mut new_keys = HashSet::new();
        while new_keys.len() < n {
            new_keys.insert(rng.gen::<u32>());
        }
        let new_keys: Vec<_> = new_keys.into_iter().collect();

        t = Instant::now();
        for i in 0..n {
            if random[i] {
                client.put(
                    QuickIStr::new(new_keys[i] as u64).into(),
                    QuickIStr::new(new_keys[i] as u64 + 1).into(),
                );
            } else {
                client.get_check(
                    QuickIStr::new(keys[i] as u64).into(),
                    QuickIStr::new(keys[i] as u64 + 1).into(),
                );
            }
        }
        let r50_time_cost = t.elapsed();
        client.report("r50", n, r50_time_cost);
        client.wait();

        client.end();
    }
}

pub struct Testcasesi32<T: TestTree<i32>> {
    _phantom: PhantomData<T>,
}

impl<T: TestTree<i32>> Testcasesi32<T> {
    pub fn find<C: TestClient<i32, T>>(name: &str) -> fn(&mut C, size: usize) {
        match name {
            "simple" => Self::test_simple,
            _ => panic!("unknown test case: {}", name),
        }
    }

    pub fn test_simple<C: TestClient<i32, T>>(client: &mut C, n: usize) {
        // client.notice("start simple");

        let mut rng = StdRng::seed_from_u64((12345 + client.id()) as u64);
        let mut keys = HashSet::new();
        while keys.len() < n {
            keys.insert(rng.gen::<i32>());
        }
        let keys: Vec<_> = keys.into_iter().collect();

        let mut t = Instant::now();
        for &key in keys.iter() {
            client.put(key, key + 1);
        }
        let put_time_cost = t.elapsed();
        client.report("put", n, put_time_cost);
        client.wait();

        t = Instant::now();
        for &key in keys.iter() {
            client.get_check(key, key + 1);
        }
        let get_time_cost = t.elapsed();
        client.report("get", n, get_time_cost);
        client.wait();

        let random = (0..n).map(|_| rng.gen::<bool>()).collect::<Vec<_>>();
        let mut new_keys = HashSet::new();
        while new_keys.len() < n {
            new_keys.insert(rng.gen::<i32>());
        }
        let new_keys: Vec<_> = new_keys.into_iter().collect();

        t = Instant::now();
        for i in 0..n {
            if random[i] {
                client.put(new_keys[i], new_keys[i] + 1);
            } else {
                client.get_check(keys[i], keys[i] + 1);
            }
        }
        let r50_time_cost = t.elapsed();
        client.report("r50", n, r50_time_cost);
        client.wait();

        client.end();
        // client.notice(
        //     format!(
        //         "simple test done. put time cost: {:?}, get time cost: {:?}",
        //         put_time_cost, get_time_cost
        //     )
        //     .as_str(),
        // );
    }
}

pub struct TestcasesUsize<T: TestTree<usize>> {
    _phantom: PhantomData<T>,
}

impl<T: TestTree<usize>> TestcasesUsize<T> {
    pub fn find<C: TestClient<usize, T>>(name: &str) -> fn(&mut C, size: usize) {
        match name {
            "simple" => Self::test_simple,
            _ => panic!("unknown test case: {}", name),
        }
    }

    pub fn test_simple<C: TestClient<usize, T>>(client: &mut C, n: usize) {
        let mut rng = StdRng::seed_from_u64((12345 + client.id()) as u64);
        let mut keys = HashSet::new();
        while keys.len() < n {
            keys.insert(rng.gen::<usize>());
        }
        let keys: Vec<_> = keys.into_iter().collect();

        let mut t = Instant::now();
        for &key in keys.iter() {
            client.put(key, key + 1);
        }
        let put_time_cost = t.elapsed();
        client.report("put", n, put_time_cost);
        client.wait();

        t = Instant::now();
        for &key in keys.iter() {
            client.get_check(key, key + 1);
        }
        let get_time_cost = t.elapsed();
        client.report("get", n, get_time_cost);
        client.wait();

        let random = (0..n).map(|_| rng.gen::<bool>()).collect::<Vec<_>>();
        let mut new_keys = HashSet::new();
        while new_keys.len() < n {
            new_keys.insert(rng.gen::<usize>());
        }
        let new_keys: Vec<_> = new_keys.into_iter().collect();

        t = Instant::now();
        for i in 0..n {
            if random[i] {
                client.put(new_keys[i], new_keys[i] + 1);
            } else {
                client.get_check(keys[i], keys[i] + 1);
            }
        }
        let r50_time_cost = t.elapsed();
        client.report("r50", n, r50_time_cost);
        client.wait();

        client.end();
        // client.notice(
        //     format!(
        //         "simple test done. put time cost: {:?}, get time cost: {:?}",
        //         put_time_cost, get_time_cost
        //     )
        //     .as_str(),
        // );
    }
}