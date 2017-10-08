fn main() {
    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).expect("Read error.");
    input_line.clear();

    let mut array_sum: i32 = 0;
    std::io::stdin().read_line(&mut input_line).expect("Read error.");
    for number_str in input_line.split_whitespace() {
        let number = number_str.parse().expect("Failed to parse number.");
        array_sum += number;
    }
    println!("{}", array_sum);
}
