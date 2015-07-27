extern crate ramp;
use ramp::Int;

fn main() {
    let num = Int::from(2).pow(1000);
    let str = num.to_str_radix(10, false);
    let answer = str.chars().fold(0, |sum, i| sum + i as i32 - 48);
    println!("{}", answer);
}
