trait GetInGrid {
    type Item;
    fn get_in_grid(&self, position: (usize, usize)) -> Option<&Self::Item>;
}

impl<T> GetInGrid for Vec<Vec<T>> {
    type Item = T;

    fn get_in_grid(&self, (x, y): (usize, usize)) -> Option<&Self::Item> {
        self.get(y)?.get(x)
    }
}

#[derive(Debug)]
struct Digit {
    index: usize,
    near_part: bool,
    digit: char,
}

// A very much, non-rust approach lmao
fn find_digit(
    grid: &Vec<Vec<char>>,
    ignore_list: &mut Vec<(usize, usize)>,
    (mut x, y): (usize, usize),
) -> Option<u32> {
    if ignore_list.contains(&(x, y)) {
        return None;
    }
    if !grid.get_in_grid((x, y))?.is_numeric() {
        return None;
    };
    // Go back until no number is found
    while x > 0 && grid.get_in_grid((x - 1, y)).is_some_and(|c| c.is_numeric()) {
        x -= 1;
    }

    let mut n = 0_u32;
    while grid.get_in_grid((x, y)).is_some_and(|c| c.is_numeric()) {
        ignore_list.push((x, y));
        n = n * 10 + *grid.get_in_grid((x, y)).expect("Literally what") as u32 - '0' as u32;
        x += 1;
    }
    Some(n)
}

fn get_in_grid(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Option<char> {
    match grid.get(y)?.get(x)? {
        '.' => None,
        c if c.is_numeric() => None, // Numbers are not parts
        c => Some(*c),
    }
}

fn one(content: String) -> u32 {
    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.as_bytes().iter().map(|byte| *byte as char).collect())
        .collect();

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            let mut prev_index = None;
            let mut numbers = vec![(false, String::new())];
            for digit in
                row.iter()
                    .enumerate()
                    .filter(|(_, c)| c.is_numeric())
                    .map(|(x, number)| Digit {
                        index: x,
                        digit: *number,
                        near_part: (x.saturating_sub(1)..=x.saturating_add(1)).any(|real_x| {
                            (y.saturating_sub(1)..=y.saturating_add(1))
                                .any(|real_y| get_in_grid(&grid, real_x, real_y).is_some())
                        }),
                    })
            {
                match prev_index {
                    Some(n) if n + 1 == digit.index => {
                        let current_number = numbers.last_mut().expect("This is impossible");
                        current_number.0 = current_number.0 || digit.near_part;
                        current_number.1.push(digit.digit)
                    }
                    None => {
                        let current_number = numbers.last_mut().expect("This is impossible");
                        current_number.0 = current_number.0 || digit.near_part;
                        current_number.1.push(digit.digit)
                    }
                    Some(_) => numbers.push((digit.near_part, format!("{}", digit.digit))),
                }
                prev_index = Some(digit.index)
            }
            numbers
                .iter()
                .filter_map(|(near_part, number)| match near_part {
                    true => number.parse::<u32>().ok(),
                    false => None,
                })
                .sum::<u32>()
        })
        .sum()
}

fn two(content: String) -> u32 {
    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.as_bytes().iter().map(|byte| *byte as char).collect())
        .collect();

    grid.iter()
        .enumerate()
        // Find all locations with a gear
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, char)| **char == '*')
                .map(|(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()

        // For each gear, search for surrounding digits
        .map(|(center_x, center_y)| {
            let mut ignore_list: Vec<(usize, usize)> = vec![(center_x, center_y)];
            (center_x.saturating_sub(1)..=center_x.saturating_add(1))
                .map(|x| {
                    (center_y.saturating_sub(1)..=center_y.saturating_add(1))
                        .filter_map(|y| find_digit(&grid, &mut ignore_list, (x, y)))
                        .collect::<Vec<u32>>()
                })
                .flatten()
                .collect::<Vec<u32>>()
        })
        .filter(|gears| gears.len() == 2)
        .map(|gears| gears.iter().product::<u32>())
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
