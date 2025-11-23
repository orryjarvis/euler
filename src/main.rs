use euler::lib;

fn main() {
    println!("p0: {:?}", p0(554_000));
    println!("p1: {:?}", p1(1000));
    println!("p2: {:?}", p2(4_000_000));
    println!("p3: {:?}", p3(600_851_475_143));
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
    // 100 is sort of magic to prevent overflow.. which
    // is a sign that this sort of solution is not what 
    // the problem author had intended :)
    (1..=100)
        .map(|x| lib::fib(x))
        .filter(|x| x <= &n)
        .filter(|x| x % 2 == 0)
        .sum()
}

fn p3(n: u128) -> u128 {
    let upper: u128 = f64::floor(f64::sqrt(n as f64)) as u128;
    (1..=upper)
        .filter(|x| n % x == 0)
        .filter(|x| lib::is_prime(*x))
        .last().unwrap()
}
