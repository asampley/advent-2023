use std::collections::BinaryHeap;

fn main() {
    let (score, cards, _) = std::io::stdin()
        .lines()
        .map_while(|line| line.ok())
        .map(|line| {
            let (_, card) = line.split_once(":").unwrap();
            let (our_nums, winning_nums) = card.split_once("|").unwrap();

            // parse all our numbers
            let our_nums: Vec<u32> = our_nums
                .split(" ")
                .filter_map(|n| str::parse(n).ok())
                .collect();

            // count winning numbers
            winning_nums
                .split(" ")
                .filter_map(|n| str::parse(n).ok())
                .filter(|n| our_nums.contains(n))
                .count() as u32
        })
        // give each card their index
        .enumerate()
        .fold(
            (0, 0, BinaryHeap::new()),
            |(mut score, mut card_instances, mut heap), next| {
                let (card_num, winning_count) = next;

                // regular score
                score += match winning_count {
                    0 => 0,
                    x => 2_u64.pow(x - 1),
                };

                // remove cards lower than where multiplier ends
                while heap
                    .peek()
                    .map(|(n, _)| *n > -(card_num as isize))
                    .unwrap_or(false)
                {
                    heap.pop();
                }

                // calculate instances of card as the sum of all duplications
                let instances = 1 + heap.iter().map(|(_, inst)| inst).sum::<u64>();

                // add up the instances of cards
                card_instances += instances;

                // keep track of how many bonuses, and to what position
                heap.push((-((card_num + winning_count as usize) as isize), instances));

                (score, card_instances, heap)
            },
        );

    println!("{score}");
    println!("{cards}");
}
