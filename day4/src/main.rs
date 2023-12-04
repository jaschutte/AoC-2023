
fn one(content: String) -> u32 {
    content
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(":").expect("Invalid Input");
            let (ours, winning) = line.split_once("|").expect("Invalid Input");

            let set = winning
                .split(" ")
                .filter_map(|numbers| numbers.parse::<u32>().ok())
                .collect::<Vec<_>>();

            ours.split(" ")
                .filter_map(|numbers| numbers.parse::<u32>().ok())
                .fold(0, |sum, number| match (sum, set.contains(&number)) {
                    (0, true) => 1,
                    (_, true) => sum * 2,
                    (_, false) => sum,
                })
        })
        .sum()
}

fn two(content: String) -> u32 {
    let mut loop_count = Vec::new();

    let data: Vec<(Vec<u32>, Vec<u32>)> = content
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(":").expect("Invalid Input");
            let (ours, winning) = line.split_once("|").expect("Invalid Input");
            loop_count.push(1);
            (
                ours.split(" ")
                    .filter_map(|numbers| numbers.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
                winning
                    .split(" ")
                    .filter_map(|numbers| numbers.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
            )
        })
        .collect();

    for (data_index, (ours, set)) in data.iter().enumerate() {
        for insert_index in data_index + 1
            ..data_index + 1 + ours.iter().filter(|number| set.contains(&number)).count()
        {
            loop_count[insert_index] = loop_count[insert_index] + loop_count[data_index];
        }
    }

    loop_count.iter().sum()
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
