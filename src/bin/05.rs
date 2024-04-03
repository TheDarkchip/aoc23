advent_of_code::solution!(5);
use nom::branch::alt;
use nom::character::complete::{alpha1, line_ending, newline, not_line_ending, space1, u64};
use nom::combinator::eof;
use nom::sequence::{separated_pair, terminated, tuple};
use nom::{bytes::complete::tag, character::complete::char, multi::many1, sequence::preceded};

fn seed_parser(input: &str) -> nom::IResult<&str, u64> {
    preceded(char(' '), u64)(input)
}

fn seed_parser2(input: &str) -> nom::IResult<&str, Vec<u64>> {
    let (input, first_num) = preceded(char(' '), u64)(input)?;
    let (input, second_num) = preceded(char(' '), u64)(input)?;
    Ok((input, (first_num..(first_num + second_num)).collect()))
}

fn advance_line(input: &str) -> nom::IResult<&str, char> {
    newline(input)
}

fn skip_rest_of_line(input: &str) -> nom::IResult<&str, &str> {
    not_line_ending(input)
}

fn from_to_parser(input: &str) -> nom::IResult<&str, Conversion> {
    separated_pair(alpha1, tag("-to-"), alpha1)(input)
        .map(|(input, (from, to))| (input, Conversion::new(from.to_string(), to.to_string())))
}

fn dst_src_range_parser(input: &str) -> nom::IResult<&str, Mapping> {
    tuple((u64, preceded(space1, u64), preceded(space1, u64)))(input)
        .map(|(input, (dst, src, range))| (input, Mapping { dst, src, range }))
}

fn mapping_parser(input: &str) -> nom::IResult<&str, Vec<Mapping>> {
    many1(terminated(dst_src_range_parser, newline))(input)
}

fn skip_and_advance_line(input: &str) -> nom::IResult<&str, char> {
    preceded(skip_rest_of_line, advance_line)(input)
}

fn conversion_parser(input: &str) -> nom::IResult<&str, Conversion> {
    from_to_parser(input).and_then(|(input, mut conv)| {
        skip_and_advance_line(input).and_then(|(input, _)| {
            mapping_parser(input).map(|(input, mappings)| {
                conv.mappings = mappings;
                (input, conv)
            })
        })
    })
}

fn all_conversions_parser(input: &str) -> nom::IResult<&str, Vec<Conversion>> {
    many1(terminated(conversion_parser, alt((line_ending, eof))))(input)
}

#[derive(Debug)]
struct Conversion {
    from: String,
    to: String,
    mappings: Vec<Mapping>,
}

impl Conversion {
    fn new(from: String, to: String) -> Self {
        Self {
            from,
            to,
            mappings: Vec::new(),
        }
    }

    fn apply(&self, input: u64) -> u64 {
        self.mappings
            .iter()
            .find(|m| m.convert(input) != input)
            .map_or(input, |m| m.convert(input))
    }
}

#[derive(Debug)]
struct Mapping {
    dst: u64,
    src: u64,
    range: u64,
}

impl Mapping {
    fn convert(&self, input: u64) -> u64 {
        if input >= self.src && input < self.src + self.range {
            self.dst + (input - self.src)
        } else {
            input
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (input, seeds) = preceded(tag("seeds:"), many1(seed_parser))(input).ok()?;
    //println!("{:?}", seeds);

    let (input, _) = advance_line(input).ok()?;
    let (input, _) = advance_line(input).ok()?;

    //let mut conversions = Vec::new();

    let (_, conv) = all_conversions_parser(input).ok()?;
    //println!("{:?}", conv);

    //println!("{:?}", input);

    seeds
        .into_iter()
        .map(|s| conv.iter().fold(s, |acc, c| c.apply(acc)))
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (input, seeds) = preceded(tag("seeds:"), many1(seed_parser2))(input).ok()?;
    //println!("{:?}", seeds);

    let seeds = seeds.concat();
    //println!("{:?}", seeds);

    let (input, _) = advance_line(input).ok()?;
    let (input, _) = advance_line(input).ok()?;

    //let mut conversions = Vec::new();

    let (input, conv) = all_conversions_parser(input).ok()?;
    //println!("{:?}", conv);

    //println!("{:?}", input);

    seeds
        .into_iter()
        .map(|s| conv.iter().fold(s, |acc, c| c.apply(acc)))
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
