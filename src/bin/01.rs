advent_of_code::solution!(1);
use regex::Regex;

fn parse_match(matched: &str, number_words: &[&str]) -> u32 {
    matched.parse::<u32>().unwrap_or_else(|_| {
        let position = number_words
            .iter()
            .position(|&word| word == matched)
            .unwrap();
        position as u32 + 1
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let reg = Regex::new(r"\d").unwrap();

    input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = reg
                .find_iter(line)
                .filter_map(|m| m.as_str().parse().ok())
                .collect();

            match numbers.as_slice() {
                [first, .., last] => Some(first * 10 + last),
                [just_one] => Some(just_one * 11),
                _ => None,
            }
        })
        .sum::<Option<u32>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    let reg = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let number_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let rev_number_words: Vec<String> = number_words
        .iter()
        .map(|&word| word.chars().rev().collect())
        .collect();
    let rev_regex_pattern = format!(r"({})|\d", rev_number_words.join("|"));
    let reg_rev = Regex::new(&rev_regex_pattern).unwrap();

    input
        .lines()
        .filter_map(|line| {
            let first_match = reg.find(line)?.as_str();
            let line_rev = line.chars().rev().collect::<String>();
            let last_match_rev = reg_rev.find(&line_rev)?.as_str();
            let last_match = last_match_rev.chars().rev().collect::<String>();

            let first_num = parse_match(first_match, &number_words);
            let last_num = parse_match(&last_match, &number_words);

            Some(first_num * 10 + last_num)
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
