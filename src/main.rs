use euler::{num, string};

fn main() {
    println!("p0: {:?}", p0(554_000));
    println!("p1: {:?}", p1(1000));
    println!("p2: {:?}", p2(4_000_000));
    println!("p3: {:?}", p3(600_851_475_143));
    println!("p4: {:?}", p4(3));
}

fn p0(n: u128) -> u128 {
    (1..=n)
        .map(|x| x * x)
        .filter(|x| x % 2 == 1)
        .sum()
}

fn p1(n: u128) -> u128 {
    (1..n)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

fn p2(n: u128) -> u128 {
    (1..=num::fib_largest_lte(n))
        .map(|x| num::fib(x))
        .filter(|x| x % 2 == 0)
        .sum()
}

fn p3(n: u128) -> u128 {
    let upper: u128 = f64::floor(f64::sqrt(n as f64)) as u128;
    (1..=upper)
        .filter(|x| n % x == 0)
        .filter(|x| num::is_prime(*x))
        .last().unwrap()
}



fn p4(n: u32) -> u32 {
    let mut palindromes = num::descending_product_iterator(n)
        .map(|(a, b)| a * b)
        .filter(|x| string::is_palindrome(x.to_string().as_str()))
        .peekable();
    return palindromes.peek().unwrap().to_owned();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p0() {
        assert_eq!(28338577333241000, p0(554_000))
    }

    #[test]
    fn test_p1() {
        assert_eq!(233168, p1(1000))
    }

    #[test]
    fn test_p2() {
        assert_eq!(4613732, p2(4_000_000))
    }

    #[test]
    fn test_p3() {
        assert_eq!(6857, p3(600_851_475_143))
    }
}
