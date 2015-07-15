fn main() {
    println!("{}", search(1, 21))
}

fn search(n: i32, m: i32) -> i32 {
    let mut i = n;

    'outer: loop {
        for current in n..m {
            if i % current != 0 {
                i += 1;
                break;
            }

            println!("{}", current);
            if current == m-1 {
                break 'outer
            }
        }
    }

    return i
}
