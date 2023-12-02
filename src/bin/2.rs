use std::cmp::max;

#[derive(Debug, Default)]
struct Cubes {
    r: usize,
    g: usize,
    b: usize,
}

impl Cubes {
    fn power(&self) -> usize {
        self.r * self.g * self.b
    }
}

fn main() {
    let (id_sum, power_sum) = std::io::stdin().lines()
        .map(|line| line.unwrap())
        // parse
        .filter_map(|line| game_id(&line).and_then(|(game, rest)| {
            // split each draw and get a count
            rest.split(";").filter_map(|draw| cube_count(draw))
                // and find the maximum
                .reduce(|a, b| Cubes { r: max(a.r, b.r), g: max(a.g, b.g), b: max(a.b, b.b) })
                .map(|max_count| (game, max_count))
        }))
        // zero games ids with more than 12 red, 13 green, or 14 blue
        .map(|(game, max_count)| if max_count.r <= 12 && max_count.g <= 13 && max_count.b <= 14 { (game, max_count) } else { (0, max_count) })
        // sum the game ids and powers
        .fold((0, 0), |(id_sum, power_sum), (game, max_count)| (id_sum + game, power_sum + max_count.power()));

    println!("{id_sum}");
    println!("{power_sum}");
}

// Extract the game id from the line and leave the rest
//
// Expects: "Game X: ..." where X is a number.
fn game_id(line: &str) -> Option<(usize, &str)> {
    let (game, rest) = line.split_once(":")?;

    let game = usize::from_str_radix(game.strip_prefix("Game")?.trim(), 10).ok()?;

    Some((game, rest))
}

// Extract the cube count from a single pull
//
// Expects: " X red, Y blue, Z green", where X, Y and Z are numbers, and the order is arbitrary"
fn cube_count(draw: &str) -> Option<Cubes> {
    let mut cubes = Cubes::default();

    for single in draw.split(",") {
        let (count, color) = single.trim().split_once(" ")?;

        let count = usize::from_str_radix(count, 10).ok()?;

        match color {
            "red" => cubes.r += count,
            "green" => cubes.g += count,
            "blue" => cubes.b += count,
            _ => return None,
        }
    }

    Some(cubes)
}
