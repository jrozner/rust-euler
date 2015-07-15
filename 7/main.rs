fn main() {
    println!("{}", n_prime(10_001));
}

fn n_prime(n: u32) -> u32 {
    let mut found = 0;
    let mut last = 0;
    let mut guess = 2;

    while found < n {
        for i in 1..guess+1 {
            match i {
                1 => continue,
                _ if i == guess => {
                    found += 1;
                    last = guess;
                    guess += 1;
                    break;
                },
                _ => {
                    if guess % i == 0 {
                        guess += 1;
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    return last;
}
