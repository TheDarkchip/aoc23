advent_of_code::solution!(4);

use log::debug;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use regex::Regex;

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    win_numbers: Vec<Option<u32>>,
    own_numbers: Vec<Option<u32>>,
}

#[derive(Debug)]
struct Deck<'a> {
    cards: Vec<&'a Card>,
}

impl Card {
    fn calculate_worth(&self) -> u32 {
        match self
            .win_numbers
            .iter()
            .filter(|nr| self.own_numbers.contains(nr) && nr.is_some())
            .count()
        {
            0 => 0,
            amount => 2_u32.pow(amount as u32 - 1),
        }
    }
    fn calculate_worth2(&self) -> u32 {
        self.win_numbers
            .iter()
            .filter(|nr| self.own_numbers.contains(nr) && nr.is_some())
            .count() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let card =
        Regex::new(r"Card +(?P<id>\d+): +(?P<win_nrs>(?:(\d+ +)+))\| +(?P<own_nrs>(?:(\d+ *)+))")
            .unwrap();
    let read_cards = input.lines().filter_map(|line| {
        let cap = card.captures(line)?;
        let id = cap.name("id")?.as_str().parse::<u32>().ok()?;
        let win_nrs = cap
            .name("win_nrs")?
            .as_str()
            .split(' ')
            .map(|x| x.parse::<u32>().ok());

        let own_nrs = cap
            .name("own_nrs")?
            .as_str()
            .split(' ')
            .map(|x| x.parse::<u32>().ok());
        Some(Card {
            id,
            win_numbers: win_nrs.collect(),
            own_numbers: own_nrs.collect(),
        })
    });

    read_cards
        .map(|card| card.calculate_worth())
        .sum::<u32>()
        .into()
}

use nom::character::complete::u32;
pub fn part_two(input: &str) -> Option<u32> {
    fn parse_card_id(input: &str) -> IResult<&str, u32> {
        preceded(tuple((tag("Card"), many1(tag(" ")))), u32)(input)
    }
    fn parse_nr(input: &str) -> IResult<&str, u32> {
        preceded(many1(tag(" ")), u32)(input)
    }
    let mut card_parser = tuple((
        parse_card_id,
        preceded(tag(":"), many1(parse_nr)),
        preceded(tag(" |"), many1(parse_nr)),
    ));

    let read_cards = input.lines().map(|line| card_parser(line).ok()).map(|x| {
        x.map(|(_, (id, win_nrs, own_nrs))| Card {
            id,
            win_numbers: win_nrs.into_iter().map(Some).collect::<Vec<_>>(),
            own_numbers: own_nrs.into_iter().map(Some).collect::<Vec<_>>(),
        })
    });

    let mut deck = Deck { cards: vec![] };
    let cards = read_cards.flatten().collect::<Vec<_>>();

    debug!("Cards: {:?}", cards);
    for card in cards.iter() {
        debug!("Card: {:?}", card);
        let worth = card.calculate_worth2();
        debug!("Worth: {}", worth);
        let card_amount = deck.cards.iter().filter(|c2| c2.id == card.id).count() + 1;
        deck.cards.push(card);
        debug!("Card amount: {}", card_amount);
        let id_range = card.id + 1..=card.id + worth;
        debug!("Card range: {:?}", id_range);
        for id in id_range {
            debug!("ID: {}", id);
            let other_card = cards.iter().find(|c2| c2.id == id);
            debug!("Other card: {:?}", other_card);
            for _ in 1..=card_amount {
                deck.cards.push(other_card?);
            }
        }
    }

    Some(deck.cards.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
