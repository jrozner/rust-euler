fn main() {
    let mut i = 0;
    let mut it = (1..).take_while(|i| factors(triangle(*i as usize)) < 500);
    while let Some(n) = it.next() {
        i = n;
    }
    println!("{}", triangle(i+1));
}

fn triangle(n: usize) -> usize {
    return (1..).take(n).fold(0, |sum, i| sum + i);
}

fn factors(n: usize) -> i32 {
    return (1..).take((n as f64).sqrt() as usize).fold(0, |factors, guess| {
        if n % guess == 0 {
            return factors + 2;
        }
        return factors;
    });
}
