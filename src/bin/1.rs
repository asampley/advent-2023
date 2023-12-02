fn main() {
    let sum: u32 = std::io::stdin().lines()
        // stop after a line fails
        .map_while(|l| l.ok())
        // get a number out of each line that has numbers
        .filter_map(|l| l.char_indices()
            // read the next digit or number word
            .filter_map(|(i, c)| c.to_digit(10).or_else(|| {
                let p = &l[i..];

                if p.starts_with("zero") { Some(0) }
                else if p.starts_with("one") { Some(1) }
                else if p.starts_with("two") { Some(2) }
                else if p.starts_with("three") { Some(3) }
                else if p.starts_with("four") { Some(4) }
                else if p.starts_with("five") { Some(5) }
                else if p.starts_with("six") { Some(6) }
                else if p.starts_with("seven") { Some(7) }
                else if p.starts_with("eight") { Some(8) }
                else if p.starts_with("nine") { Some(9) }
                else { None }
            }))
            // collect digits into a pair of first and last
            .fold(None, |acc, n| match acc {
                 None => Some((n, n)),
                 Some((a, _)) => Some((a, n)),
            })
            // convert to a single number
            .map(|(a, b)| a * 10 + b)
        )
        // add up the number from each line
        .sum();

    println!("{sum}");
}
