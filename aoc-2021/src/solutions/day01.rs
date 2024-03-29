use std::{slice::Windows, str::FromStr};

pub fn parse_input<T>(input: &[String]) -> Vec<T>
where
    T: FromStr + Default,
{
    input
        .iter()
        .map(|i| i.parse().unwrap_or_default())
        .collect()
}

#[inline]
fn solve(windows: Windows<i64>) -> usize {
    windows.filter(|x| x.first() < x.last()).count()
}

#[inline]
pub fn part_one(input: &[String]) -> String {
    let numbers = parse_input::<i64>(input);
    solve(numbers.windows(2)).to_string()
}

#[inline]
pub fn part_two(input: &[String]) -> String {
    let numbers = parse_input::<i64>(input);
    solve(numbers.windows(4)).to_string()
}
#[cfg(test)]
mod test {
    fn test_input() -> Vec<String> {
        vec![
            "199".to_string(),
            "200".to_string(),
            "208".to_string(),
            "210".to_string(),
            "200".to_string(),
            "207".to_string(),
            "240".to_string(),
            "269".to_string(),
            "260".to_string(),
            "263".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = "7";
        let actual = crate::solutions::day01::part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "5";
        let actual = crate::solutions::day01::part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
