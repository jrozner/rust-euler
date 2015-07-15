fn main() {
    let primes = prime_factors(600851475143);
    println!("{}", primes.last().unwrap())
}

fn prime_factors(n: i64) -> Vec<i64> {
    let mut i = 2;
    let mut m = n;
    let mut factors: Vec<i64> = Vec::new();

    while i <= m {
        if m % i == 0 {
            factors.push(i);
            m /= i;
        } else {
            i += 1;
        }
    }

    return factors
}
