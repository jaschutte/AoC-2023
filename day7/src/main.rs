type CardStrength = usize;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind([u8; 5]),
    FourOfAKind([u8; 5]),
    FullHouse([u8; 5]),
    ThreeOfAKind([u8; 5]),
    TwoPairs([u8; 5]),
    OnePair([u8; 5]),
    HighCard([u8; 5]),
}

fn one(content: String) -> u64 {
    let mut hands = content
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(hand, bid)| (hand.trim(), bid.trim()))
        .filter_map(|(hand, bid)| {
            let mut cards = [0_u8; 13];
            let cards_identifiers = [
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ];

            let transformed_hand = hand
                .chars()
                .map(|c| {
                    let index = cards_identifiers
                        .iter()
                        .position(|find| *find == c)
                        .expect("Invalid Input");
                    cards[index] += 1;
                    index as u8
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let bid = bid.parse::<u64>().ok()?;

            let mut cards = cards.into_iter().filter(|n| *n != 0).collect::<Vec<u8>>();
            cards.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

            Some((
                match cards.as_slice() {
                    [5] => HandType::FiveOfAKind(transformed_hand),
                    [4, 1] => HandType::FourOfAKind(transformed_hand),
                    [3, 2] => HandType::FullHouse(transformed_hand),
                    [3, 1, 1] => HandType::ThreeOfAKind(transformed_hand),
                    [2, 2, 1] => HandType::TwoPairs(transformed_hand),
                    [2, 1, 1, 1] => HandType::OnePair(transformed_hand),
                    [1, 1, 1, 1, 1] => HandType::HighCard(transformed_hand),
                    invalid => panic!("Invalid Input: {invalid:?}"),
                },
                bid,
            ))
        })
        .collect::<Vec<_>>();
    hands.sort_unstable_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    hands
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (multiplier, (_, bid))| {
            sum + bid * (multiplier as u64 + 1)
        })
}

fn two(content: String) -> u64 {
    let mut hands = content
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(hand, bid)| (hand.trim(), bid.trim()))
        .filter_map(|(hand, bid)| {
            let mut cards = [0_u8; 14];
            let cards_identifiers = [
                'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
            ];

            let transformed_hand = hand
                .chars()
                .map(|c| {
                    let index = cards_identifiers
                        .iter()
                        .position(|find| *find == c)
                        .expect("Invalid Input");
                    cards[index] += 1;
                    index as u8
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let bid = bid.parse::<u64>().ok()?;

            let mut cards = cards.into_iter().filter(|n| *n != 0).collect::<Vec<u8>>();
            cards.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

            let joker_count = hand.chars().filter(|c| *c == 'J').count() as u8;
            // If we got jokers, add them to the most important one (being the top)
            if joker_count > 0 {
                // Make sure to get rid of the joker cards
                let (index, joker_cards) = cards
                    .iter_mut()
                    .enumerate()
                    .find(|(_, n)| **n == joker_count)
                    .expect("You must be *JOKING* me right?");
                *joker_cards -= joker_count;
                // If the card amount is 0, just remove it
                if *joker_cards == 0 {
                    cards.remove(index);
                }

                // At last, add the jokers!
                match cards.first_mut() {
                    Some(card) => *card += joker_count,
                    None => cards = vec![joker_count],
                }
            }

            Some((
                match cards.as_slice() {
                    [5] => HandType::FiveOfAKind(transformed_hand),
                    [4, 1] => HandType::FourOfAKind(transformed_hand),
                    [3, 2] => HandType::FullHouse(transformed_hand),
                    [3, 1, 1] => HandType::ThreeOfAKind(transformed_hand),
                    [2, 2, 1] => HandType::TwoPairs(transformed_hand),
                    [2, 1, 1, 1] => HandType::OnePair(transformed_hand),
                    [1, 1, 1, 1, 1] => HandType::HighCard(transformed_hand),
                    invalid => panic!("Invalid Input: {invalid:?}"),
                },
                bid,
            ))
        })
        .collect::<Vec<_>>();
    hands.sort_unstable_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    hands
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (multiplier, (_, bid))| {
            sum + bid * (multiplier as u64 + 1)
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
