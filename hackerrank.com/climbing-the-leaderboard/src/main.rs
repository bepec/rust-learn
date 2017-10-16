#[derive(Debug,Copy,Clone)]
struct Player {
    rank: u32,
    score: u32
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Read line from stdin");
    // let players_num: u32 = line.trim().parse().expect("Parse number");

    line.clear();
    std::io::stdin().read_line(&mut line).expect("Read line from stdin");
    let leaderboard = line.split_whitespace()
        .scan(Player { rank: 0, score: 0 }, |prev, s| {
            let score = s.parse::<u32>().expect("Parse number");
            if score != prev.score { prev.rank += 1; }
            prev.score = score;
            Some(*prev)
        })
        .collect::<Vec<_>>();

    line.clear();
    std::io::stdin().read_line(&mut line).expect("Read line from stdin");
    // let levels_num: u32 = line.trim().parse().expect("Parse number");

    line.clear();
    std::io::stdin().read_line(&mut line).expect("Read line from stdin");
    line.split_whitespace().map(|s| {
        let alice_score = s.parse::<u32>().expect("Parse number");
        let competitor = leaderboard.iter().rev()
            .find(|player| player.score > alice_score);
        let alice_rank = match competitor {
            Some(x) => x.rank + 1,
            None => 1
        };
        alice_rank
    }).inspect(|rank| println!("{}", rank)).count();
}
