fn main() {
    println!("{:?}", p0(554_000));
}

fn p0(n: u128) -> u128 {
    (1..=n)
        .map(|x| x * x)
        .filter(|x| x % 2 == 1)
        .sum()
}
