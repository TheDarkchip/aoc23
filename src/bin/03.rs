advent_of_code::solution!(3);
use regex::Regex;
use std::ops::RangeInclusive;

#[derive(PartialEq)]
struct Position {
    x: RangeInclusive<usize>,
    y: usize,
}

#[derive(PartialEq)]
enum Kind {
    Machine,
    Number,
}
#[derive(PartialEq)]
struct Symbol<'a> {
    position: Position,
    kind: Kind,
    value: &'a str,
}

impl Symbol<'_> {
    fn close_to(&self, other: &Symbol) -> bool {
        let self_x_start = *self.position.x.start();
        let self_x_end = *self.position.x.end();
        let other_x_start = *other.position.x.start();
        let other_x_end = *other.position.x.end();

        let x_overlap = self_x_start <= other_x_end && other_x_start <= self_x_end;
        let y_close = (other.position.y as i32 - self.position.y as i32).abs() <= 1;

        x_overlap && y_close
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map_reg = Regex::new(r"(\d+|[^.\d])").unwrap();
    let symbols: Vec<Symbol> = input
        .lines()
        .enumerate()
        .map(|line_symbol| (map_reg.find_iter(line_symbol.1), line_symbol.0))
        .flat_map(move |x| x.0.map(move |y| (y.as_str(), y.start()..=y.end(), x.1)))
        .map(|s| Symbol {
            position: Position { x: s.1, y: s.2 },
            kind: match s.0.contains(|c: char| c.is_numeric()) {
                true => Kind::Number,
                false => Kind::Machine,
            },
            value: s.0,
        })
        .collect();
    let digits = symbols.iter().filter(|s| s.kind == Kind::Number);
    let machine_sym: Vec<&Symbol> = symbols.iter().filter(|s| s.kind == Kind::Machine).collect();
    digits
        .filter(|s| {
            machine_sym
                .iter()
                .filter(|m_s| (m_s.position.y - 1..=m_s.position.y + 1).contains(&s.position.y))
                .any(|m_s| s.close_to(m_s))
        })
        .map(|s| s.value.parse::<u32>().ok())
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let map_reg = Regex::new(r"(\d+|[^.\d])").unwrap();
    let symbols: Vec<Symbol> = input
        .lines()
        .enumerate()
        .map(|line_symbol| (map_reg.find_iter(line_symbol.1), line_symbol.0))
        .flat_map(move |x| x.0.map(move |y| (y.as_str(), y.start()..=y.end(), x.1)))
        .map(|s| Symbol {
            position: Position { x: s.1, y: s.2 },
            kind: match s.0.contains(|c: char| c.is_numeric()) {
                true => Kind::Number,
                false => Kind::Machine,
            },
            value: s.0,
        })
        .collect();
    let digits = symbols.iter().filter(|s| s.kind == Kind::Number);
    let machine_sym: Vec<&Symbol> = symbols.iter().filter(|s| s.kind == Kind::Machine).collect();
    let number_gear = digits
        .filter_map(|s| {
            machine_sym
                .iter()
                .filter(|m_s| {
                    m_s.value == "*"
                        && (m_s.position.y - 1..=m_s.position.y + 1).contains(&s.position.y)
                })
                .find_map(|m_s| s.close_to(m_s).then_some((s, m_s)))
        })
        .collect::<Vec<_>>();

    number_gear
        .iter()
        .filter_map(|num| {
            number_gear
                .iter()
                .find_map(|num2| (num.1 == num2.1 && num.0 != num2.0).then_some((num.0, num2.0)))
        })
        .try_fold(0u32, |acc, syms| -> Result<u32, std::num::ParseIntError> {
            let value1 = syms.0.value.parse::<u32>()?;
            let value2 = syms.1.value.parse::<u32>()?;
            Ok(acc + value1 * value2)
        })
        .map(|res| res / 2)
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
