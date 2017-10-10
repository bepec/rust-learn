fn main() {
    let mut alice_str = String::new();
    std::io::stdin().read_line(&mut alice_str).expect("Read error.");
    let alice = alice_str.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse number."));

    let mut bob_str = String::new();
    std::io::stdin().read_line(&mut bob_str).expect("Read error.");
    let bob = bob_str.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse number."));

    let result = alice.zip(bob)
        .fold((0, 0), |r, (a, b)| {
            if a < b { return (r.0, r.1 + 1); }
            if a > b { return (r.0 + 1, r.1); }
            (r.0, r.1)
        });

    println!("{} {}", result.0, result.1);
}
