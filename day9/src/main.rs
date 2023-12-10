fn get_diff(input: &Vec<i32>) -> Vec<i32> {
    let mut new = Vec::with_capacity(input.len().saturating_sub(1));
    let mut iter = input.iter().peekable();
    loop {
        new.push(match (iter.next(), iter.peek()) {
            (None, _) => break,
            (Some(_), None) => break,
            (Some(a), Some(b)) => *b - *a,
        })
    }
    new
}

fn one(content: String) -> i32 {
    content
        .lines()
        .map(|line| {
            let mut hierarchy = vec![line
                .split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok())
                .collect::<Vec<_>>()];

            loop {
                hierarchy.push(match hierarchy.last() {
                    Some(last) if last.iter().all(|diff| *diff == 0) => break,
                    Some(prev) => get_diff(&prev),
                    _ => break,
                })
            }
            
            let mut tree_iterator = hierarchy.iter_mut().rev().peekable();
            while let Some(item) = tree_iterator.next() {
                if let Some(next) = tree_iterator.peek_mut() {
                    next.push(*next.last().expect("Sure bud") + *item.last().expect("Nu-uh"));
                }
            }

            *hierarchy.first().expect("Yes you do in fact exist").last().expect("So do you.")
        })
        .sum()
}

fn two(content: String) -> i32 {
    content
        .lines()
        .map(|line| {
            let mut hierarchy = vec![line
                .split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok())
                .collect::<Vec<_>>()];

            loop {
                hierarchy.push(match hierarchy.last() {
                    Some(last) if last.iter().all(|diff| *diff == 0) => break,
                    Some(prev) => get_diff(&prev),
                    _ => break,
                })
            }
            
            let mut tree_iterator = hierarchy.iter_mut().rev().peekable();
            while let Some(item) = tree_iterator.next() {
                if let Some(next) = tree_iterator.peek_mut() {
                    next.insert(0, *next.first().expect("Sure bud") - *item.first().expect("Nu-uh"));
                }
            }

            // dbg!(*hierarchy.first().expect("Yes you do in fact exist").first().expect("So do you."));
            *hierarchy.first().expect("Yes you do in fact exist").first().expect("So do you.")
        })
        .sum()
}

fn main() {
    #[derive(Debug)]
    enum AoCPart {
        One,
        Two,
    }

    #[derive(Debug)]
    enum InputType {
        Demo,
        Real,
    }

    // S^2-SD+R=0
    // Solved for S, that is:
    // S = -sqrt(-R + (D/2)^2) + D/2

    let input_type = match std::env::var("AOC_TYPE") {
        Ok(n) if n == "demo" => InputType::Demo,
        Ok(n) if n == "real" => InputType::Real,
        _ => {
            println!("No/invalid AOC_TYPE environment variable, resorting to demo file.");
            InputType::Demo
        }
    };

    let content = std::fs::read_to_string(match input_type {
        InputType::Demo => "demo",
        InputType::Real => "input",
    })
    .expect("No input file");

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
