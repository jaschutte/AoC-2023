fn one(content: String) -> u64 {
    let (steps, nodes) = content.split_once("\n\n").expect("Invalid Input");

    let steps = steps
        .chars()
        .map(|c| match c {
            'L' => 1,
            'R' => 2,
            _ => panic!("Left or right..???? WHat is thiiiiiiiis????"),
        })
        .collect::<Vec<_>>();

    let nodes = nodes
        .lines()
        .filter_map(|line| {
            let (source, targets) = line.split_once(" = ")?;
            let (left, right) = targets.split_once(", ")?;
            let left = left.trim_start_matches("(");
            let right = right.trim_start_matches(")");

            Some([source, left, right])
        })
        .collect::<Vec<_>>();

    let mut walker = steps.iter().cycle()
    loop {

    }


    0
}

fn two(content: String) -> u64 {
    0
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
