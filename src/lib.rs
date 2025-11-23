
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
            0 => 0,
            1 => 1,
            x => fib(x - 1) + fib(x - 2)
        };
        fib_map = FIB.lock().unwrap();
        fib_map.insert(n, computed);
        computed
    }

    pub fn is_prime(n: u128) -> bool {
        if n <= 1 {
            return false;
        }
        let upper: u128 = f64::floor(f64::sqrt(n as f64)) as u128;
        (2..=upper)
            .filter(|x| n % x == 0)
            .collect::<Vec<_>>()
            .is_empty()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_fib() {
            assert_eq!(0, fib(0));
            assert_eq!(1, fib(1));
            assert_eq!(1, fib(2));
            assert_eq!(5, fib(5));
            assert_eq!(8, fib(6));
            assert_eq!(2, fib(3));
            assert_eq!(3, fib(4));
        }

        
        #[test]
        fn test_is_prime() {
            assert_eq!(false, is_prime(0));
            assert_eq!(false, is_prime(1));
            assert_eq!(true, is_prime(2));
            assert_eq!(true, is_prime(3));
            assert_eq!(false, is_prime(4));
            assert_eq!(true, is_prime(5));
            assert_eq!(false, is_prime(6));
        }
    }
}
