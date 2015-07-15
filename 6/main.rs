fn main() {
    let sum_of_squares = (1..101).map(|i: u32| i.pow(2)).fold(0, |sum, i| sum + i);
    let square_of_sum = (1..101).fold(0, |sum, i: u32| sum + i).pow(2);

    println!("{}", square_of_sum - sum_of_squares);
}
