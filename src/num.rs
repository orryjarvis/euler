use std::mem;
use std::collections::{BinaryHeap, HashSet};

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
        yield 0;
        yield 1;
        let mut a  = 0_u128;
        let mut b = 1_u128;
        loop {
            yield a + b;
            mem::swap(&mut a, &mut b);
            b = a + b;
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
    let c = #[coroutine] || -> ! {
        yield 2_u128;
        let mut primes: Vec<(u128, u128)> = vec![(2_u128, 2_128)];
        let mut num = 3_u128;
        loop {
            let mut is_prime = true;
            for p in primes.iter_mut() {
                while p.1 < num {
                    p.1 += p.0
                }
                if p.1 == num {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                primes.push((num, num));
                yield num;
            }
            num += 2;
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
