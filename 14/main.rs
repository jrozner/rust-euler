fn main() {
    let mut biggest: Option<(u64,u64)> = None;

    for i in (1..1_000_000) {
        let res = collatz_seq(i);

        if biggest == None {
            biggest = Some(res);
            continue;
        }

        if res.1 > biggest.unwrap().1 {
            biggest = Some(res);
        }
    }

    println!("{:?}", biggest.unwrap());
}

fn collatz_seq(n: u64) -> (u64, u64) {
    let mut length = 1;
    let mut i = n;
    while let Some(j) = collatz(i) {
        i = j;
        length += 1;
    }

    return (n, length);
}

fn collatz(n: u64) -> Option<u64> {
    return match n % 2 {
        _ if n == 1 => None,
        0 => Some(n / 2),
        _ => Some(n * 3 + 1)
    }
}
