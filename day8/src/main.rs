fn one(content: String) -> u64 {
    let (steps, nodes) = content.split_once("\n\n").expect("Invalid Input");

    let steps = steps
        .chars()
        .map(|c| match c {
            'L' => 0_usize,
            'R' => 1_usize,
            _ => panic!("Left or right..???? WHat is thiiiiiiiis????"),
        })
        .collect::<Vec<_>>();

    let ref_nodes = nodes
        .lines()
        .filter_map(|line| {
            let (source, targets) = line.split_once(" = ")?;
            let (left, right) = targets.split_once(", ")?;
            let left = left.trim_start_matches("(");
            let right = right.trim_end_matches(")");

            Some([source, left, right])
        })
        .collect::<Vec<_>>();

    struct Node<'a> {
        directions: [usize; 2],
        label: &'a str,
    }

    let nodes = ref_nodes
        .iter()
        .map(|this_node| {
            let left = ref_nodes
                .iter()
                .position(|find| find[0] == this_node[1])
                .expect("Invalid");
            let right = ref_nodes
                .iter()
                .position(|find| find[0] == this_node[2])
                .expect("Invalid");
            Node {
                directions: [left, right],
                label: this_node[0],
            }
        })
        .collect::<Vec<Node>>();

    let mut walker = steps.iter().cycle();
    let mut current_node = nodes
        .iter()
        .find(|n| n.label == "AAA")
        .expect("Invalid Input");
    let mut iteration = 0;
    loop {
        iteration += 1;
        let direction = *walker
            .next()
            .expect("We're cycling, so how is this failing?");

        let target_node = &nodes[current_node.directions[direction]];
        if target_node.label == "ZZZ" {
            break;
        }

        current_node = target_node;
    }
    iteration
}

#[inline]
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

// Euclidian GCD
#[inline]
fn gcd(a: u64, b: u64) -> u64 {
    let (mut min, mut max) = (a.min(b), a.max(b));
    loop {
        match max % min {
            0 => break min,
            mod_result => {
                max = min;
                min = mod_result;
            }
        }
    }
}

fn two(content: String) -> u64 {
    let (steps, nodes) = content.split_once("\n\n").expect("Invalid Input");

    let steps = steps
        .chars()
        .map(|c| match c {
            'L' => 0_usize,
            'R' => 1_usize,
            _ => panic!("Left or right..???? WHat is thiiiiiiiis????"),
        })
        .collect::<Vec<_>>();

    let ref_nodes = nodes
        .lines()
        .filter_map(|line| {
            let (source, targets) = line.split_once(" = ")?;
            let (left, right) = targets.split_once(", ")?;
            let left = left.trim_start_matches("(");
            let right = right.trim_end_matches(")");

            Some([source, left, right])
        })
        .collect::<Vec<_>>();

    struct Node<'a> {
        directions: [usize; 2],
        label: &'a str,
    }

    let nodes = ref_nodes
        .iter()
        .map(|this_node| {
            let left = ref_nodes
                .iter()
                .position(|find| find[0] == this_node[1])
                .expect("Invalid");
            let right = ref_nodes
                .iter()
                .position(|find| find[0] == this_node[2])
                .expect("Invalid");
            Node {
                directions: [left, right],
                label: this_node[0],
            }
        })
        .collect::<Vec<Node>>();

    let mut walker = steps.iter().cycle();
    let mut current_nodes: Vec<&Node<'_>> =
        nodes.iter().filter(|n| n.label.ends_with("A")).collect();
    let mut patterns: Vec<Vec<u64>> = current_nodes.iter().map(|_| vec![]).collect();
    let mut found: Vec<Option<u64>> = current_nodes.iter().map(|_| None).collect();

    let mut iteration = 0_u64;
    loop {
        iteration += 1;
        let direction = *walker
            .next()
            .expect("We're cycling, so how is this failing?");

        for (index, current_node) in current_nodes.iter_mut().enumerate() {
            *current_node = &nodes[current_node.directions[direction]];

            if current_node.label.ends_with("Z") && found[index].is_none() {
                patterns[index].push(iteration);
                found[index] = patterns[index]
                    .starts_with(&patterns[index][(patterns[index].len() / 2)..])
                    .then_some(iteration);
            }
        }

        if found.iter().all(|found| found.is_some()) {
            break;
        }
    }

    // Calculate the LCM of the list, which is just doing LCM(old, LCM(old, LCM(old, ...))) 6 times
    found.iter().fold(1, |acc, n| {
        lcm(acc, n.expect("This is literally impossible"))
    })
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
