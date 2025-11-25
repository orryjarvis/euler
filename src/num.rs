use std::collections::{BinaryHeap, HashSet, HashMap};

use crate::generator;

pub fn fib(n: i32) -> u128 {
    let computed = match n {
        0 => 0,
        1 => 1,
        x => fib(x - 1) + fib(x - 2)
    };
    computed
}

pub fn fib_largest_lte(n: u128) -> i32 {
    let x = f32::sqrt(5.0) * (n as f32 + 0.5);
    x.log(std::f32::consts::PHI).floor() as i32
}

pub fn fib_iterator() -> impl Iterator<Item=u128> {
    let c = #[coroutine] || -> ! {
        let mut a: u128 = 0;
        let mut b: u128 = 1;

        loop {
            yield a;
            let next = a + b;
            a = b;
            b = next;
        }
    };
    return generator::create_infinite_generator(c);
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

pub fn prime_iterator() -> impl Iterator<Item=u128> {
    let c = #[coroutine]
    || -> ! {
        // Yield 2 separately so we can skip all even numbers afterward.
        yield 2_u128;

        // Map: composite -> step (the increment to get the next composite for a given prime)
        let mut composites: HashMap<u128, u128> = HashMap::new();

        // Start from the first odd candidate
        let mut n: u128 = 3;

        loop {
            if let Some(step) = composites.remove(&n) {
                // n is composite; move this prime's marker to its next composite
                let mut next = n + step;

                // Avoid collisions: if some other prime already marked `next`,
                // keep stepping until we find a free slot
                while composites.contains_key(&next) {
                    next += step;
                }

                composites.insert(next, step);
            } else {
                // n is not marked composite → it's prime
                yield n;

                // First composite we mark for this prime.
                // Using n*n (classic Eratosthenes optimization):
                // all smaller composites of n will already have a smaller prime factor.
                let start = n.saturating_mul(n);
                // Step is 2*n because we only visit odd numbers (n is odd)
                let step = 2 * n;

                // Only insert if we didn't overflow n*n
                if start > n {
                    // If multiple primes try to set the same composite,
                    // we just keep the first step – duplicates are handled by the while loop above.
                    if !composites.contains_key(&start) {
                        composites.insert(start, step);
                    } else {
                        // If there's already something there, find the next free composite
                        let mut next = start + step;
                        while composites.contains_key(&next) {
                            next += step;
                        }
                        composites.insert(next, step);
                    }
                } else {
                    // Overflow zone for u128; at this point you can't safely do n*n anyway.
                    // You could `break` here or just continue yielding primes by trial division.
                }
            }
            n += 2;
        }
    };
    generator::create_infinite_generator(c)
}

pub fn descending_product_iterator(digits: u32) -> impl Iterator<Item=(u32, u32)>
{    
    let c = #[coroutine] move || {
        let max = 10_u32.pow(digits) - 1;
        
        // Max heap of (product, i, j) tuples
        // BinaryHeap is max-heap by default, perfect for descending order
        let mut heap: BinaryHeap<(u32, u32, u32)> = BinaryHeap::new();
        let mut seen: HashSet<(u32, u32)> = HashSet::new();
        
        // Start with the maximum product
        heap.push((max * max, max, max));
        seen.insert((max, max));
        
        while let Some((_product, i, j)) = heap.pop() {
            yield (i, j);
            
            // Add neighbors: (i-1, j) and (i, j-1)
            // We only generate pairs where i >= j to avoid duplicates
            if i > 1 && j <= i - 1 && !seen.contains(&(i - 1, j)) {
                seen.insert((i - 1, j));
                heap.push(((i - 1) * j, i - 1, j));
            }
            
            if j > 1 && !seen.contains(&(i, j - 1)) {
                seen.insert((i, j - 1));
                heap.push((i * (j - 1), i, j - 1));
            }
        }
    };
    generator::create_generator(c)
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
    fn test_fib_largest_lte() {
        assert_eq!(0, fib_largest_lte(0));
        assert_eq!(2, fib_largest_lte(1));
        assert_eq!(4, fib_largest_lte(3));
        assert_eq!(6, fib_largest_lte(10));
        assert_eq!(7, fib_largest_lte(15));
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

    
    #[test]
    fn test_prime_iterator() {
        let mut i = prime_iterator();
        assert_eq!(Some(2), i.next());
        assert_eq!(Some(3), i.next());
        assert_eq!(Some(5), i.next());
        assert_eq!(Some(7), i.next());
        assert_eq!(Some(11), i.next());
        assert_eq!(Some(13), i.next());
    }
    
    #[test]
    fn test_fib_iterator() {
        let mut i = fib_iterator();
        assert_eq!(Some(0), i.next());
        assert_eq!(Some(1), i.next());
        assert_eq!(Some(1), i.next());
        assert_eq!(Some(2), i.next());
        assert_eq!(Some(3), i.next());
        assert_eq!(Some(5), i.next());
        assert_eq!(Some(8), i.next());
        assert_eq!(Some(13), i.next());
        assert_eq!(Some(21), i.next());
    }
    
    #[test]
    fn test_descending_product_iterator_order() {
        let tuples: Vec<(u32, u32)> = descending_product_iterator(2).collect();
        println!("{:?}", tuples);
        let products = tuples
            .iter()
            .map(|(a,b)| a * b)
            .collect::<Vec<_>>();
        println!("{:?}", products);
        assert!(products.iter().rev().is_sorted())
    }
}
