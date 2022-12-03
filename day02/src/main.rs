use core::panic;

fn file(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|it| String::from(it))
        .collect::<Vec<String>>()
}

fn main() {
    let lines = file("tests/myinput.txt");
    println!("Total score: {:?}", total_score(&lines));
    println!("Total score (part2): {:?}", total_score_part2(&lines));
}

enum AResult {
    Win,
    Lose,
    Draw,
}

impl<T> From<T> for AResult
where
    T: Into<String>,
{
    fn from(str: T) -> Self {
        let string = str.into();
        let input: &str = string.as_str();
        match input {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!(),
        }
    }
}

impl AResult {
    fn from(p1: &Sign, p2: &Sign) -> Self {
        match (p1, p2) {
            (Sign::Rock, Sign::Rock)
            | (Sign::Paper, Sign::Paper)
            | (Sign::Scissors, Sign::Scissors) => Self::Draw,
            (Sign::Rock, Sign::Scissors)
            | (Sign::Paper, Sign::Rock)
            | (Sign::Scissors, Sign::Paper) => Self::Win,
            (Sign::Rock, Sign::Paper)
            | (Sign::Paper, Sign::Scissors)
            | (Sign::Scissors, Sign::Rock) => Self::Lose,
        }
    }

    fn val(self: &Self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl<T> From<T> for Sign
where
    T: Into<String>,
{
    fn from(str: T) -> Self {
        let string = str.into();
        let input: &str = string.as_str();
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!(),
        }
    }
}

impl Sign {
    fn val(self: &Self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn from_result(self: &Self, result: &AResult) -> Self {
        match (self, result) {
            (Self::Scissors, AResult::Win)
            | (Self::Rock, AResult::Draw)
            | (Self::Paper, AResult::Lose) => Self::Rock,
            (Self::Rock, AResult::Win)
            | (Self::Paper, AResult::Draw)
            | (Self::Scissors, AResult::Lose) => Self::Paper,
            (Self::Paper, AResult::Win)
            | (Self::Scissors, AResult::Draw)
            | (Self::Rock, AResult::Lose) => Self::Scissors,
        }
    }
}

fn round_to_pair<'a>(line: String) -> Option<(Sign, Sign)> {
    let sp: Vec<&str> = line.split(" ").collect();
    if sp.len() == 2 {
        Some(((*sp.get(0).unwrap()).into(), (*sp.get(1).unwrap()).into()))
    } else {
        None
    }
}

fn round_to_pair_part2<'a>(line: String) -> Option<(Sign, AResult)> {
    let sp: Vec<&str> = line.split(" ").collect();
    if sp.len() == 2 {
        Some(((*sp.get(0).unwrap()).into(), (*sp.get(1).unwrap()).into()))
    } else {
        None
    }
}

fn eval_round(line: String) -> i32 {
    let pair = round_to_pair(line).unwrap();
    let res = AResult::from(&pair.1, &pair.0);
    return pair.1.val() + res.val();
}

fn eval_round_part2(line: String) -> i32 {
    let pair = round_to_pair_part2(line).unwrap();
    let my_sign = pair.0.from_result(&pair.1);
    return pair.1.val() + my_sign.val();
}

fn total_score(lines: &Vec<String>) -> i32 {
    lines
        .into_iter()
        .map(|it| eval_round(it.clone()))
        .fold(0, |acc, el| acc + el)
}

fn total_score_part2(lines: &Vec<String>) -> i32 {
    lines
        .into_iter()
        .map(|it| eval_round_part2(it.clone()))
        .fold(0, |acc, el| acc + el)
}

#[cfg(test)]
mod tests {
    use crate::{eval_round, eval_round_part2, total_score, total_score_part2};

    #[test]
    fn example() {
        let lines = crate::file("tests/test1.txt");
        assert_eq!(8, eval_round(lines.get(0).unwrap().clone()));
        assert_eq!(1, eval_round(lines.get(1).unwrap().clone()));
        assert_eq!(6, eval_round(lines.get(2).unwrap().clone()));

        assert_eq!(15, total_score(&lines))
    }

    #[test]
    fn example_part2() {
        let lines = crate::file("tests/test1.txt");
        assert_eq!(4, eval_round_part2(lines.get(0).unwrap().clone()));
        assert_eq!(1, eval_round_part2(lines.get(1).unwrap().clone()));
        assert_eq!(7, eval_round_part2(lines.get(2).unwrap().clone()));

        assert_eq!(12, total_score_part2(&lines))
    }
}
