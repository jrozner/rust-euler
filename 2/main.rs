fn main() {
    let nums = fib();
    let sum = nums.iter().fold(0, add_even);
    println!("{}", sum);
}

fn add_even(acc: i32, item: &i32) -> i32 {
    if item % 2 == 0 {
        return acc + item;
    }

    acc
}

fn fib() -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();

    loop {
        match nums.len() {
            0 => nums.push(1),
            1 => nums.push(2),
            i => {
                let j = nums[i-1] + nums[i-2];
                if j <= 4_000_000 {
                    nums.push(j)
                } else {
                    break
                }
            }
        }
    }

    return nums
}
