fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    let result = (0..n).fold(0, |diff, i| {
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        let row = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        diff + row[i] - row[n - 1 - i]
    });
    println!("{}", result.abs());
}
