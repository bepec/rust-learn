fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    let mut diff = 0;
    for i in 0..n {
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        if i * 2 == n - 1 { continue; }
        let row_diff = line.split_whitespace().enumerate()
            .filter(|&(j, _)| j == i || j == n - 1 - i)
            .fold(0, |sum, (j, s)| {
                let mut val = s.parse::<i32>().unwrap();
                if i != j { val = -val };
                sum + val
            });
        diff += row_diff;
    }
    println!("{}", diff.abs());
}
