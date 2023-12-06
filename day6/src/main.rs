fn calc_bounds_for_record(max_time: f32, record: f32) -> (f32, f32) {
    let half_time = max_time / 2.0;
    let shared = (-record + half_time.powf(2.0)).sqrt();
    (-shared + half_time, shared + half_time)
}

fn one(content: String) -> u64 {
    let (time, record) = content.split_once('\n').expect("Invalid Input");

    time.split_whitespace()
        .filter_map(|n| n.parse::<u64>().ok())
        .zip(
            record
                .split_whitespace()
                .filter_map(|n| n.parse::<u64>().ok()),
        )
        .map(|(time, record)| {
            let (lower, upper) = calc_bounds_for_record(time as f32, record as f32 + 0.5);
            dbg!(lower, upper);
            (upper.floor() - lower.floor()) as u64
        })
        .product()
}

fn two(content: String) -> u64 {
    let (time, record) = content.split_once('\n').expect("Invalid Input");

    let time = time
        .replace(' ', "")
        .trim_start_matches("Time:")
        .parse::<u64>()
        .expect("Invalid Time");
    let record = record
        .replace(' ', "")
        .trim_start_matches("Distance:")
        .trim()
        .parse::<u64>()
        .expect("Invalid Distance");

    let (lower, upper) = calc_bounds_for_record(time as f32, record as f32 + 0.5);
    (upper.floor() - lower.floor()) as u64
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
