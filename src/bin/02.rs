use std::{iter::Peekable, str::Chars, cmp::max};

use itertools::Itertools;
use regex::Regex;
advent_of_code::solution!(2);

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = input
        .trim_end()
        .split("\n")
        .map(|line| {
            let mut chars = line.chars().filter(|c| !c.is_whitespace()).peekable();
            fn eat_until<I: Iterator<Item = char>>(until: char, line: &mut I) {
                while line.next().unwrap() != until {}
            }

            fn parse_num<I: Iterator<Item = char>>(line: &mut Peekable<I>) -> u32 {
                let mut num = 0;
                while let Some(digit) = line.peek().unwrap().to_digit(10) {
                    num = num * 10 + digit;
                    line.next();
                }
                num
            }
            fn parse_draw<I: Iterator<Item = char>>(line: &mut Peekable<I>) -> bool {
                let num = parse_num(line);
                let possible = match line.peek().unwrap() {
                    'r' => num <= 12,
                    'g' => num <= 13,
                    'b' => num <= 14,
                    _ => panic!("Expected color, got: {}", line.peek().unwrap()),
                };
                while line.peek().filter(|c| c.is_alphabetic()).is_some() {
                    line.next();
                }
                possible
            }
            eat_until('e', &mut chars);
            let game_id = parse_num(&mut chars);
            eat_until(':', &mut chars);

            while chars.peek().is_some() {
                let possible = parse_draw(&mut chars);
                if !possible {
                    return 0;
                }
                chars.next();
            }
            game_id
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum: u32 = input
        .trim_end()
        .split("\n")
        .map(|line| {
            let mut chars = line.chars().filter(|c| !c.is_whitespace()).peekable();
            fn eat_until<I: Iterator<Item = char>>(until: char, line: &mut I) {
                while line.next().unwrap() != until {}
            }

            fn parse_num<I: Iterator<Item = char>>(line: &mut Peekable<I>) -> u32 {
                let mut num = 0;
                while let Some(digit) = line.peek().unwrap().to_digit(10) {
                    num = num * 10 + digit;
                    line.next();
                }
                num
            }
            fn parse_draw<I: Iterator<Item = char>>(line: &mut Peekable<I>) -> (u32, Color) {
                let num = parse_num(line);
                let color = match line.peek().unwrap() {
                    'r' => Color::Red,
                    'g' => Color::Green,
                    'b' => Color::Blue,
                    _ => panic!("Expected color, got: {}", line.peek().unwrap()),
                };
                while line.peek().filter(|c| c.is_alphabetic()).is_some() {
                    line.next();
                }
                (num, color)
            }
            eat_until(':', &mut chars);

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            while chars.peek().is_some() {
                let (num, color) = parse_draw(&mut chars);
                match color {
                    Color::Red => red = max(red, num),
                    Color::Green => green = max(green, num),
                    Color::Blue => blue = max(blue, num),
                }
                chars.next();
            }
            red * green * blue
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
