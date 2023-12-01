trait MultiFind {
    type Needle;

    /// Searched for any possible needle in `&self`
    /// If a match if found, it returns which needle inside of `needles` was found
    fn find_multiple(&self, needles: &[Self::Needle]) -> Option<usize>;
}

// Implement our trait for String type
impl MultiFind for String {
    type Needle = Self;

    fn find_multiple(&self, needles: &[Self::Needle]) -> Option<usize> {
        for index in 0..self.len() {
            let buffer = &self[index..];
            for (idx, needle) in needles.iter().enumerate() {
                if buffer.starts_with(needle) {
                    return Some(idx);
                }
            }
        }
        None
    }
}

fn one(content: String) -> u32 {
    content
        .lines()
        .map(|s| {
            format!(
                "{}{}",
                s.chars().find(|c| c.is_numeric()).expect("Invalid Line"),
                s.chars()
                    .rev()
                    .find(|c| c.is_numeric())
                    .expect("Invalid Line")
            )
            .parse::<u32>()
            .expect("Invaid Number")
        })
        .sum()
}

fn two(content: String) -> u32 {
    // Get a list of valid words
    let words: Vec<String> = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let inv_words: Vec<String> = words
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect();

    content
        .lines()
        .map(|s| {
            let inv_s: String = s.chars().rev().collect();
            format!(
                "{}{}",
                match s.to_string().find_multiple(&words).expect("Invalid Line") {
                    n @ 10.. => n - 10,
                    n => n,
                },
                match inv_s.find_multiple(&inv_words).expect("Invalid Line") {
                    n @ 10.. => n - 10,
                    n => n,
                }
            )
            .parse::<u32>()
            .expect("Invaid Number")
        })
        .sum()
}

fn main() {
    let content = std::fs::read_to_string("input").expect("No input file");

    #[derive(Debug)]
    enum AoCPart {
        One,
        Two
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
