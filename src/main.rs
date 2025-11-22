use euler::lib;

fn main() {
    println!("{:?}", p0(554_000));
    println!("{:?}", p1(1000));
    println!("{:?}", p2(4_000_000));
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
    (1..=100)
        .map(|x| lib::fib(x))
        .filter(|x| x <= &n)
        .filter(|x| x % 2 == 0)
        .sum()
}
