fn main() {
    let primes = sieve_eratosthenes(2_000_001);
    let mut sum = 0;
    let mut i: usize = 0;

    while i < primes.len() {
        if primes[i] == true {
            // need to add 1 to each because vector is 0 indexed
            sum += i;
        }

        i += 1;
    }

    println!("{}", sum)
}

fn sieve_eratosthenes(size: usize) -> Vec<bool> {
    let mut vec: Vec<bool> = Vec::with_capacity(size as usize);
    let mut p: usize = 2;

    // set 0 and 1 to false, we know they aren't
    vec.push(false);
    vec.push(false);

    // init all the rest to possibly true
    for _i in 2..size {
        vec.push(true)
    }

    let mut mul = 2;
    while p < size {
        let mut i: usize = p * mul;
        while i < size {
            vec[i] = false;
            mul += 1;
            i = p * mul;
        }

        // move p forward 1
        p += 1;

        // find next p
        while p < size && vec[p] != true {
           p += 1;
        }

        // reset mul to 2
        mul = 2;
    }

    return vec;
}
