use std::collections::HashSet;
use std::str::FromStr;
use itertools::Itertools;
use crate::err::InputError;
use crate::util::Solution;


#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl FromStr for Card {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::Ace), "K" => Ok(Card::King), "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack), "T" => Ok(Card::Ten), "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight), "7" => Ok(Card::Seven), "6" => Ok(Card::Six),
            "5" => Ok(Card::Five), "4" => Ok(Card::Four), "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => Err(InputError::WrongFormat(format!("Invalid card: {}", s)))
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Type {
    FiveOfAKind(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card),
    ThreeOfAKind(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    HighCard(Card)
}

impl From<&Hand> for Type {
    fn from(hand: &Hand) -> Self {
        let cards_set = hand.cards.iter()
            .collect::<HashSet<&Card>>();
        match hand.cards {
            card if cards_set.len() == 1 => Type::FiveOfAKind(card[0]),
            card if cards_set.len() == 2
                && Day::count_cards(&card)[0].1 == 4 => Type::FourOfAKind(card[0]),
            card if cards_set.len() == 2 => Type::FullHouse(card[0], card[1]),
            card if cards_set.len() == 3
                && Day::count_cards(&card)[0].1 == 3 => Type::ThreeOfAKind(card[0]),
            card if cards_set.len() == 3 => Type::TwoPair(card[0], card[1]),
            card if cards_set.len() == 4 => Type::OnePair(card[0]),
            card => Type::HighCard(card[0])
        }
    }
}

pub struct Hand {
    cards: [Card; 5],
    bid: u32
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = Type::from(self);
        let other_type = Type::from(other);
        if self_type == other_type {
            self.cards.iter().cmp(other.cards.iter())
        } else {
            self_type.cmp(&other_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl FromStr for Hand {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [cards, bid] = s.trim().split_whitespace().collect::<Vec<&str>>()[..] {
            let bid = bid.parse().unwrap();
            let cards = cards.chars()
                .map(|c| c.to_string().parse())
                .collect::<Result<Vec<Card>, _>>()?;
            return Ok(Hand {
                cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
                bid
            })
        }
        Err(InputError::WrongFormat("Invalid hand".to_string()))
    }
}

pub struct Day;

impl Day {

    fn count_cards_of(cards: &[Card], card: Card) -> u32 {
        cards.iter().filter(|c| **c == card).count() as u32
    }

    fn count_cards(cards: &[Card]) -> Vec<(Card, u32)> {
        let mut counts = vec![];
        for card in cards {
            if let Some((_, count)) = counts.iter_mut().find(|(c, _)| c == card) {
                *count += 1;
            } else {
                counts.push((*card, 1));
            }
        }
        counts.sort_by(|(_, c1), (_, c2)| c2.cmp(c1));
        counts
    }
}

impl<'a> Solution<'a> for Day {
    type Input = Vec<Hand>;
    type Output = u32;
    const DAY: &'a str = "Day07";

    fn part1(input: &Self::Input) -> Self::Output {
        input.iter()
            .sorted_by(|h1, h2| h2.cmp(&h1))
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum::<u32>()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        0
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input.iter()
            .map(|s| s.parse().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".lines().map(|l| l.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day::part1(&Day::parse_input(&get_input())), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day::part2(&Day::parse_input(&get_input())), 0);
    }
}