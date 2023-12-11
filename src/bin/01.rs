advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    input.trim_end().split("\n").for_each(|line| {
        let mut forward = line.chars();
        let mut backward = line.chars().rev();
        let first = forward.find_map(|c| c.to_digit(10)).unwrap();
        let last = backward.find_map(|c| c.to_digit(10)).unwrap();

        sum += first * 10 + last;
    });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input.trim_end().split("\n").for_each(|line| {
        let mut line = line;
        sum += 'forward: loop {
            let digit = line.chars().nth(0).unwrap().to_digit(10);
            if let Some(digit) = digit {
                break digit * 10;
            }
            for (pos, number_string) in numbers.iter().enumerate() {
                if line.starts_with(number_string) {
                    break 'forward (pos as u32 + 1) * 10;
                }
            }
            line = &line[1..];
        };
        sum += 'backward: loop {
            let digit = line.chars().last().unwrap().to_digit(10);
            if let Some(digit) = digit {
                break digit;
            }
            for (pos, number_string) in numbers.iter().enumerate() {
                if line.ends_with(number_string) {
                    break 'backward pos as u32 + 1;
                }
            }
            line = &line[0..line.len() - 1];
        }
    });
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
