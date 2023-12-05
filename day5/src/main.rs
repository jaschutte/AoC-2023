use std::ops::RangeInclusive;

#[derive(Debug)]
struct TranslationEntry {
    destination: u64,
    source: u64,
    source_end: u64,
}

// 910845529
fn one(content: String) -> u64 {
    let (seeds, maps): (Vec<u64>, Vec<Vec<TranslationEntry>>) = content
        .split_once("\n\n")
        .map(|(seeds, maps)| {
            (
                seeds
                    .split_whitespace()
                    .filter_map(|num| num.parse::<u64>().ok())
                    .collect(),
                maps.split("\n\n")
                    .map(|segment| {
                        segment
                            .lines()
                            .skip(1)
                            .map(|line| {
                                let mut numbers = line
                                    .split_whitespace()
                                    .filter_map(|num| num.parse::<u64>().ok());
                                let destination = numbers.next().expect("Invalid Input");
                                let source = numbers.next().expect("Invalid Input");
                                let length = numbers.next().expect("Invalid Input");
                                TranslationEntry {
                                    destination,
                                    source,
                                    source_end: source + length - 1,
                                }
                            })
                            .collect()
                    })
                    .collect(),
            )
        })
        .expect("Invalid Input");

    *maps
        .iter()
        .fold(seeds, |numbers, ranges| {
            numbers
                .iter()
                .map(|current| {
                    ranges
                        .iter()
                        .find(|entry| current >= &entry.source && current <= &entry.source_end)
                        .map(|entry| entry.destination + current - entry.source)
                        .unwrap_or(*current)
                })
                .collect()
        })
        .iter()
        .min()
        .expect("Invalid Input")
}

trait TupleContains {
    fn has(&self, n: u64) -> bool;
}

impl TupleContains for (u64, u64) {
    #[inline]
    fn has(&self, n: u64) -> bool {
        n >= self.0 && n <= self.1
    }
}

#[derive(Debug)]
enum Propagation {
    Translated,
    WaitUntilEnd,
    Redo,
}

#[inline]
fn split_into_ranges(
    source: (u64, u64),
    splitter: (u64, u64),
    destination: u64,
) -> Vec<(u64, u64, Propagation)> {
    match (splitter.has(source.0), splitter.has(source.1)) {
        (true, true) => vec![(
            destination,
            destination + splitter.1 - splitter.0,
            Propagation::Translated,
        )],
        (true, false) => vec![
            (
                destination + source.0 - splitter.0,
                destination + splitter.1 - splitter.0,
                Propagation::Translated,
            ),
            (splitter.1 + 1, source.1, Propagation::Redo),
        ],
        (false, true) => vec![
            (source.0, splitter.0 - 1, Propagation::Redo),
            (
                destination,
                destination + source.1 - splitter.0,
                Propagation::Translated,
            ),
        ],
        (false, false) => match source.has(splitter.0) && source.has(splitter.1) {
            true => vec![
                (source.0, splitter.0 - 1, Propagation::Redo),
                (
                    destination,
                    destination + splitter.1 - splitter.0,
                    Propagation::Translated,
                ),
                (splitter.1 + 1, source.1, Propagation::Redo),
            ],
            false => vec![(source.0, source.1, Propagation::WaitUntilEnd)], // Despite this being in the same space,
                                                                            // because this will cause infinite
                                                                            // recursion, we set it to false
        },
    }
}

fn two(content: String) -> u64 {
    // dbg!("Full In:", split_into_ranges((50, 100), (60, 70), 5));
    // dbg!(
    //     "Full In Edge From Left:",
    //     split_into_ranges((50, 70), (60, 70), 5)
    // );
    // dbg!(
    //     "Full In Edge From Right:",
    //     split_into_ranges((60, 71), (60, 70), 5)
    // );
    // dbg!("Partial Left:", split_into_ranges((50, 65), (60, 70), 5));
    // dbg!("Partial Right:", split_into_ranges((65, 80), (60, 70), 5));
    // dbg!("Outside Left:", split_into_ranges((30, 40), (60, 70), 5));
    // dbg!("Outside Right:", split_into_ranges((80, 90), (60, 70), 5));

    let (seeds, maps): (Vec<u64>, Vec<Vec<TranslationEntry>>) = content
        .split_once("\n\n")
        .map(|(seeds, maps)| {
            (
                seeds
                    .split_whitespace()
                    .filter_map(|num| num.parse::<u64>().ok())
                    .collect(),
                maps.split("\n\n")
                    .map(|segment| {
                        segment
                            .lines()
                            .skip(1)
                            .map(|line| {
                                let mut numbers = line
                                    .split_whitespace()
                                    .filter_map(|num| num.parse::<u64>().ok());
                                let destination = numbers.next().expect("Invalid Input");
                                let source = numbers.next().expect("Invalid Input");
                                let length = numbers.next().expect("Invalid Input");
                                TranslationEntry {
                                    destination,
                                    source,
                                    source_end: source + length - 1,
                                }
                            })
                            .collect()
                    })
                    .collect(),
            )
        })
        .expect("Invalid Input");

    let seed_ranges = {
        let mut ranges = vec![];
        let mut seeds_iter = seeds.iter();
        while let Some(begin) = seeds_iter.next() {
            let length = seeds_iter.next().expect("Invalid Input");

            ranges.push((*begin, *begin + *length));
        }
        ranges
    };

    #[inline]
    // Output 0 should be fed back into the same function
    // Output 1 should be fed back, but if they still exist at the end, they're done
    // Output 2 are the properly translated units
    fn translate_input(
        input: &Vec<(u64, u64)>,
        translation: &TranslationEntry,
    ) -> (Vec<(u64, u64)>, Vec<(u64, u64)>, Vec<(u64, u64)>) {
        input.iter().map(|range| {
            split_into_ranges(
                *range,
                (translation.source, translation.source_end),
                translation.destination,
            )
        }).fold((vec![], vec![], vec![]), |(mut redo, mut defer, mut done), range| {
            for (min, max, propagation) in range {
                let value = (min, max);
                match propagation {
                    Propagation::Translated => done.push(value),
                    Propagation::WaitUntilEnd => defer.push(value),
                    Propagation::Redo => redo.push(value),
                }
            }
            (redo, defer, done)
        })
    }

    let arr = maps.iter().fold(seed_ranges, |mut input, translations| {
        println!("\nNext entries!\n{input:?}\n");
        let mut outputs = vec![];
        while input.len() > 0 {
            let entry_count = translations.len();
            for (index, entry) in translations.iter().enumerate() {
                let (new_input, mut defer, mut add_to_output) = translate_input(&input, entry);
                outputs.append(&mut add_to_output);
                input = new_input;

                if index == entry_count - 1 {
                    println!("Deferred: {defer:?} out of: {translations:?}");
                    outputs.append(&mut defer);
                } else {
                    input.append(&mut defer);
                }
                println!("Length of input: {}", input.len());
            }
        }
        outputs.dedup(); // Don't add TOO many duplicates!
                         // This still will have some, but that's fine
        outputs
    });

    // dbg!(&arr);

    arr.iter()
        .map(|t| [t.0, t.1])
        .flatten()
        .min()
        .expect("bruh")
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

    dbg!(split_into_ranges((45, 80), (69, 69), 1));
    dbg!(split_into_ranges((0, 17), (0, 68), 1));

    println!("Solving for part {part:?}:");
    let sum = match part {
        AoCPart::One => one(content),
        AoCPart::Two => two(content),
    };
    println!("Done!");
    println!("Sum: {sum}");
}
