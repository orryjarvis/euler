
pub mod lib {
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use std::sync::Mutex;

    lazy_static! {
        static ref FIB: Mutex<HashMap<u128, u128>> = {
            Mutex::new(HashMap::new())
        };
    }

    pub fn fib(n: u128) -> u128 {
        let mut fib_map = FIB.lock().unwrap();
        if let Some(cache_hit) = fib_map.get(&n) {
            return *cache_hit;
        }
        drop(fib_map);
        let computed = match n {
            1 => 1,
            2 => 2,
            x => fib(x - 1) + fib(x - 2)
        };
        fib_map = FIB.lock().unwrap();
        fib_map.insert(n, computed);
        computed
    }
}
