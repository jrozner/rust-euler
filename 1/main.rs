fn main() {
    let sum = (0..1000).fold(0, |acc, i| {
        match i {
            i if i % 3 == 0 => acc + i,
            i if i % 5 == 0 => acc + i,
            _ => acc
        }
    });

    println!("{}", sum);
}
