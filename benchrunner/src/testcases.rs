use std::marker::PhantomData;
use std::time::Instant;

use rand::distributions::Alphanumeric;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::testclient::{TestClient, TestTree};

pub struct Testcases<T: TestTree<String>> {
    _phantom: PhantomData<T>,
}
impl<T: TestTree<String>> Testcases<T> {
    pub fn find<C: TestClient<String, T>>(name: &str) -> fn(&mut C, usize) {
        match name {
            "put_s" => Self::put_s,
            "put_m" => Self::put_m,
            "put_l" => Self::put_l,
            _ => panic!("unknown test case: {}", name),
        }
    }

    fn put_with_value_size<C: TestClient<String, T>>(client: &mut C, n: usize, value_size: usize) {
        let mut rng = StdRng::seed_from_u64((12345 + client.id()) as u64);
        let t = Instant::now();
        for _ in 0..n {
            let key = rng.gen::<u32>().to_string();
            let value: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(value_size)
                .map(char::from)
                .collect();

            client.put(key, value);
        }
        let put_time_cost = t.elapsed();
        client.report(&format!("put_{value_size}"), n, put_time_cost);
        client.wait();
        client.end();
    }

    pub fn put_s<C: TestClient<String, T>>(client: &mut C, n: usize) {
        Self::put_with_value_size(client, n, 8)
    }

    pub fn put_m<C: TestClient<String, T>>(client: &mut C, n: usize) {
        Self::put_with_value_size(client, n, 32)
    }

    pub fn put_l<C: TestClient<String, T>>(client: &mut C, n: usize) {
        Self::put_with_value_size(client, n, 128)
    }
}
