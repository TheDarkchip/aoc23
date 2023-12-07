advent_of_code::solution!(2);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let game_id = Regex::new(r"Game (?P<id>\d+):").unwrap();
    let sets = Regex::new(r"((?:(?:\d+) (?:red|green|blue)(?:, )?)+)").unwrap();
    let number_color = Regex::new(r"(?P<number>\d+) (?P<color>red|green|blue)").unwrap();

    input
        .lines()
        .filter_map(|line| {
            let id = game_id
                .captures(line)?
                .name("id")?
                .as_str()
                .parse::<u32>()
                .ok()?;
            sets.find_iter(line)
                .fold(true, |acc, m| {
                    number_color
                        .captures_iter(m.as_str())
                        .map(|cap| match cap.extract().1 {
                            [number, "red"] => Some(number.parse::<u32>().ok()? <= 12),
                            [number, "green"] => Some(number.parse::<u32>().ok()? <= 13),
                            [number, "blue"] => Some(number.parse::<u32>().ok()? <= 14),
                            _ => panic!("Invalid color"),
                        })
                        .all(|x| x.is_some())
                        && acc
                })
                .then_some(id)
        })
        .reduce(|acc, x: u32| acc + x)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sets = Regex::new(r"((?:(?:\d+) (?:red|green|blue)(?:, )?)+)").unwrap();
    let number_color = Regex::new(r"(?P<number>\d+) (?P<color>red|green|blue)").unwrap();

    input
        .lines()
        .filter_map(|line| {
            Some(
                sets.find_iter(line)
                    .map(|m| {
                        number_color
                            .captures_iter(m.as_str())
                            .map(|cap| {
                                let [number, color] = cap.extract().1;
                                (number.parse().ok(), color)
                            })
                            .collect::<Vec<_>>()
                    })
                    .fold(
                        (0, 0, 0),
                        |acc: (u32, u32, u32), set: Vec<(Option<u32>, &str)>| {
                            (
                                acc.0.max(
                                    set.iter()
                                        .filter_map(|color| (color.1 == "red").then_some(color.0))
                                        .max()
                                        .flatten()
                                        .unwrap_or(0),
                                ),
                                acc.1.max(
                                    set.iter()
                                        .filter_map(|color| (color.1 == "green").then_some(color.0))
                                        .max()
                                        .flatten()
                                        .unwrap_or(0),
                                ),
                                acc.2.max(
                                    set.iter()
                                        .filter_map(|color| (color.1 == "blue").then_some(color.0))
                                        .max()
                                        .flatten()
                                        .unwrap_or(0),
                                ),
                            )
                        },
                    ),
            )
            .map(|x| x.0 * x.1 * x.2)
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
