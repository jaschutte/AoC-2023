trait GetInGrid<T> {
    fn get_in_grid(&self, position: (usize, usize)) -> Option<&T>;
}

impl<T> GetInGrid<T> for Vec<Vec<T>> {
    fn get_in_grid(&self, (x, y): (usize, usize)) -> Option<&T> {
        self.get(y)?.get(x)
    }
}

trait SetPush<T> {
    fn set_push<P>(&mut self, index: usize, item: P)
    where
        Self: Sized,
        P: FnMut(Option<&T>) -> T;
}

impl<T> SetPush<T> for Vec<T> {
    fn set_push<P>(&mut self, index: usize, mut item: P)
    where
        Self: Sized,
        P: FnMut(Option<&T>) -> T,
    {
        match self.get_mut(index) {
            Some(val) => *val = item(Some(val)),
            None => self.push(item(None)),
        }
    }
}

fn one(content: String) -> u64 {
    let mut horizontal_check = vec![];
    let vertical_duplicated: Vec<usize> = content
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let mut is_filled = false;
            for (index, char) in line.char_indices() {
                horizontal_check.set_push(index, |current| match char {
                    '.' => *current.unwrap_or(&false),
                    _ => {
                        is_filled = true;
                        true
                    }
                })
            }
            !is_filled
        })
        .map(|(index, _)| index)
        .collect();
    let horizontal_duplicated: Vec<usize> = horizontal_check
        .iter()
        .enumerate()
        .filter(|(_, bool)| !**bool)
        .map(|(index, _)| index)
        .collect();

    let galaxies: Vec<(usize, usize)> = content
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    (c == '#').then_some((
                        x + horizontal_duplicated.iter().filter(|n| **n < x).count(),
                        y + vertical_duplicated.iter().filter(|n| **n < y).count(),
                    ))
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();

    galaxies.iter().fold(0, |sum, a| {
        sum + galaxies.iter().fold(0, |sum, b| {
            if a == b {
                return sum;
            }

            let dist_x = (a.0.max(b.0)) - (a.0.min(b.0));
            let dist_y = (a.1.max(b.1)) - (a.1.min(b.1));
            sum + dist_x + dist_y
        })
    }) as u64
        / 2
}

fn two(content: String) -> u64 {
    const MULT_FACTOR: usize = 1_000_000;

    let mut horizontal_check = vec![];
    let vertical_duplicated: Vec<usize> = content
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let mut is_filled = false;
            for (index, char) in line.char_indices() {
                horizontal_check.set_push(index, |current| match char {
                    '.' => *current.unwrap_or(&false),
                    _ => {
                        is_filled = true;
                        true
                    }
                })
            }
            !is_filled
        })
        .map(|(index, _)| index)
        .collect();
    let horizontal_duplicated: Vec<usize> = horizontal_check
        .iter()
        .enumerate()
        .filter(|(_, bool)| !**bool)
        .map(|(index, _)| index)
        .collect();

    let galaxies: Vec<(usize, usize)> = content
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    (c == '#').then_some({
                        let hor = horizontal_duplicated.iter().filter(|n| **n < x).count();
                        let ver = vertical_duplicated.iter().filter(|n| **n < y).count();
                        (x + hor * MULT_FACTOR - hor, y + ver * MULT_FACTOR - ver)
                    })
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();

    galaxies.iter().fold(0, |sum, a| {
        sum + galaxies.iter().fold(0, |sum, b| {
            if a == b {
                return sum;
            }

            let dist_x = (a.0.max(b.0)) - (a.0.min(b.0));
            let dist_y = (a.1.max(b.1)) - (a.1.min(b.1));
            sum + dist_x + dist_y
        })
    }) as u64
        / 2
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
