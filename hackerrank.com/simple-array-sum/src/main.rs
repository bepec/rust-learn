fn main() {
    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).expect("Read error.");
    input_line.clear();

    std::io::stdin().read_line(&mut input_line).expect("Read error.");
    let array_sum = input_line.split_whitespace()
                              .map(|s| s.parse::<i32>().expect("Failed to parse number."))
                              .fold(0, |sum, n| sum + n);
    println!("{}", array_sum);
}
