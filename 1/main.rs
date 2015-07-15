fn main() {
    let mut nums: Vec<i32> = Vec::new();

    for i in 0..1000 {
        match i {
            i if i % 3 == 0 => nums.push(i),
            i if i % 5 == 0 => nums.push(i),
            _ => continue
        }
    }

    let sum = nums.iter().fold(0, |acc: i32, i: &i32| acc + i);
    println!("{}", sum);
}
