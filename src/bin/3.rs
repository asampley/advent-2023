use std::cmp::{max, min};
use std::fmt::{self, Debug};

use advent_2023::RingBuffer;

#[derive(Clone, Eq, PartialEq)]
pub enum Symbol {
    Digit(u8),
    Symbol(u8),
    None,
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Digit(d) => write!(f, "{d}"),
            Self::Symbol(s) => write!(f, "{s}"),
            Self::None => write!(f, "."),
        }
    }
}

fn symbols(s: &[u8]) -> Vec<Symbol> {
    s.iter()
        .map(|&u| match u {
            b'.' => Symbol::None,
            d if d >= b'0' && d <= b'9' => Symbol::Digit(d - b'0'),
            s => Symbol::Symbol(s),
        })
        .collect()
}

fn main() {
    let mut ring_buffer = RingBuffer::<_, 3>::default();

    let mut len = 0;

    let mut sum = 0;
    let mut gear_sum = 0;

    for line in std::io::stdin()
        .lines()
        .map_while(|line| Some(line.ok()))
        // on more iteration to finish gear evaluation
        .chain(std::iter::once(None))
    {
        match line {
            Some(line) => {
                let parsed = symbols(line.as_bytes());
                ring_buffer.push(Some(parsed))
            }
            None => ring_buffer.push(None),
        };

        if len == 0 {
            len = ring_buffer[0_usize].len();
        }

        // check the middle line (-1) against the two adjacent for gears, and numbers
        if ring_buffer.get_isize(-1).is_none() {
            continue;
        }

        // check each index for a symbol
        for i in 0..len {
            let symbol = match ring_buffer[-1_isize][i] {
                Symbol::Symbol(s) => s,
                _ => continue,
            };

            let mut gear_count = 0;
            let mut gear = 1;

            for b1 in [-2_isize, -1, 0] {
                if ring_buffer.get_isize(b1).is_none() {
                    continue;
                }

                // since we aren't removing digits, we must track how far right we've read
                // to avoid duplicates
                let mut after_l = 0;

                // check one left, same column, and one right, as "adjacent"
                for mut l in (max(1, i) - 1)..=min(len - 1, i + 1) {
                    // we've already read this digit
                    if l < after_l {
                        continue;
                    }

                    if !matches!(ring_buffer[b1].get(l), Some(Symbol::Digit(_))) {
                        continue;
                    }

                    gear_count += 1;

                    let mut num: u64 = 0;

                    // get the furthest left matching digit
                    while let Some(Symbol::Digit(_)) =
                        l.checked_sub(1).and_then(|b| ring_buffer[b1].get(b))
                    {
                        l -= 1;
                    }

                    // take digits as long as we can
                    while let Some(Symbol::Digit(d)) = ring_buffer[b1].get_mut(l) {
                        num *= 10;
                        num += *d as u64;

                        l += 1
                    }

                    if symbol == b'*' && gear_count <= 2 {
                        gear *= num;
                    }

                    sum += num;

                    after_l = l + 1
                }
            }

            if gear_count == 2 {
                gear_sum += gear;
            }
        }
    }

    println!("{sum}");
    println!("{gear_sum}");
}
