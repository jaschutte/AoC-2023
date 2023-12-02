#[derive(Debug)]
enum ParsedStr<'a> {
    Str(&'a str),
    Number(u32),
}

fn one(content: String) -> u32 {
    content
        .lines()
        .map(|line| {
            line.split([':', ',', ';']).fold(0, |id, segment| {
                match segment.trim().split_once(' ').map(|(a, b)| {
                    (
                        a.parse::<u32>()
                            .map_or(ParsedStr::Str(a), |n| ParsedStr::Number(n)),
                        b.parse::<u32>()
                            .map_or(ParsedStr::Str(b), |n| ParsedStr::Number(n)),
                    )
                }) {
                    Some((ParsedStr::Str("Game"), ParsedStr::Number(new_id))) => new_id,
                    Some((ParsedStr::Number(n), ParsedStr::Str("red"))) if n > 12 => 0,
                    Some((ParsedStr::Number(_), ParsedStr::Str("red"))) => id,
                    Some((ParsedStr::Number(n), ParsedStr::Str("green"))) if n > 13 => 0,
                    Some((ParsedStr::Number(_), ParsedStr::Str("green"))) => id,
                    Some((ParsedStr::Number(n), ParsedStr::Str("blue"))) if n > 14 => 0,
                    Some((ParsedStr::Number(_), ParsedStr::Str("blue"))) => id,
                    _ => panic!("Invalid Input"),
                }
            })
        })
        .sum()
}

fn two(content: String) -> u32 {
    content
        .lines()
        .map(|line| {
            line.split([':', ',', ';']).fold(
                [0, 0, 0],
                |[red, green, blue], segment| {
                    match segment.trim().split_once(' ').map(|(a, b)| {
                        (
                            a.parse::<u32>()
                                .map_or(ParsedStr::Str(a), |n| ParsedStr::Number(n)),
                            b.parse::<u32>()
                                .map_or(ParsedStr::Str(b), |n| ParsedStr::Number(n)),
                        )
                    }) {
                        Some((ParsedStr::Str("Game"), ParsedStr::Number(_))) => [red, green, blue],
                        Some((ParsedStr::Number(r), ParsedStr::Str("red"))) => [red.max(r), green, blue],
                        Some((ParsedStr::Number(g), ParsedStr::Str("green"))) => [red, green.max(g), blue],
                        Some((ParsedStr::Number(b), ParsedStr::Str("blue"))) => [red, green, blue.max(b)],
                        _ => panic!("Invalid Input"),
                    }
                }
            ).iter().product::<u32>()
        })
        .sum()
}

fn main() {
    let content = std::fs::read_to_string("input").expect("No input file");

    #[derive(Debug)]
    enum AoCPart {
        One,
        Two,
    }

    let part = match std::env::var("AOC_PART") {
        Ok(n) if n == "1" => AoCPart::One,
        Ok(n) if n == "2" => AoCPart::Two,
        Ok(_) => {
            println!("Invalid part is specified, resorting to part 1.");
            AoCPart::One
        }
        Err(_) => {
            println!("No AOC_PART variable, resorting to part 1.");
            AoCPart::One
        }
    };

    println!("Solving for part {part:?}:");
    let sum = match part {
        AoCPart::One => one(content),
        AoCPart::Two => two(content),
    };
    println!("Done!");
    println!("Sum: {sum}");
}

