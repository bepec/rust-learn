fn read_nums() -> Vec<i32> {
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).expect("Read error.");
    input_str.split_whitespace()
             .map(|s| s.parse::<i32>().expect("Failed to parse number."))
             .collect()
}

fn main() {
    let (alice, bob) = (read_nums(), read_nums());

    let result = alice.iter().zip(bob.iter())
        .fold((0, 0), |r, (a, b)| {
            if a < b { return (r.0, r.1 + 1); }
            if a > b { return (r.0 + 1, r.1); }
            (r.0, r.1)
        });

    println!("{} {}", result.0, result.1);
}
