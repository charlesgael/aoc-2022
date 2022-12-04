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
    println!("Result: {}", calculate_score(&lines));
    println!("Result part2: {}", calculate_score_p2(&lines));
}

trait Extension {
    fn is_fully_enclosed(self: &Self, other: &Self) -> bool;
    fn borrows_some(self: &Self, other: &Self) -> bool;
}

impl<T: PartialEq> Extension for Vec<T> {
    fn is_fully_enclosed(self: &Self, other: &Self) -> bool {
        self.iter().all(|it| other.contains(it))
    }

    fn borrows_some(self: &Self, other: &Self) -> bool {
        self.iter().any(|it| other.contains(it))
    }
}

fn parse_range<T: Into<String>>(range: T) -> Vec<i32> {
    let entries: Vec<String> = range.into().split("-").map(String::from).collect();
    let e1 = entries.get(0).unwrap().parse::<i32>().unwrap();
    let e2 = entries.get(1).unwrap().parse::<i32>().unwrap();
    (e1..(e2 + 1)).collect()
}

fn split_line<T: Into<String>>(line: T) -> Vec<String> {
    line.into().split(",").map(String::from).collect()
}

fn parse_line<T: Into<String>>(line: T) -> Vec<Vec<i32>> {
    split_line(line.into())
        .into_iter()
        .map(parse_range)
        .collect::<Vec<Vec<i32>>>()
}

fn one_overlap(input: Vec<Vec<i32>>) -> bool {
    let e1 = input.get(0).unwrap();
    let e2 = input.get(1).unwrap();
    e1.is_fully_enclosed(e2) || e2.is_fully_enclosed(e1)
}

fn contains_some(input: Vec<Vec<i32>>) -> bool {
    let e1 = input.get(0).unwrap();
    let e2 = input.get(1).unwrap();
    e1.borrows_some(e2)
}

fn calculate_score(lines: &Vec<String>) -> i32 {
    lines
        .into_iter()
        .map(parse_line)
        .map(one_overlap)
        .fold(0, |acc, it| acc + if it { 1 } else { 0 })
}

fn calculate_score_p2(lines: &Vec<String>) -> i32 {
    lines
        .into_iter()
        .map(parse_line)
        .map(contains_some)
        .fold(0, |acc, it| acc + if it { 1 } else { 0 })
}

#[cfg(test)]
mod tests {
    use crate::{calculate_score, one_overlap, parse_line, parse_range};

    #[test]
    fn example() {
        let lines = crate::file("tests/example.txt");

        let test_range = parse_range("2-4");
        assert_eq!(vec![2, 3, 4], test_range);

        assert_eq!(
            vec![vec![2, 3, 4], vec![6, 7, 8]],
            parse_line(lines.get(0).unwrap())
        );
        assert_eq!(
            vec![vec![2, 3], vec![4, 5]],
            parse_line(lines.get(1).unwrap())
        );
        assert_eq!(
            vec![vec![5, 6, 7], vec![7, 8, 9]],
            parse_line(lines.get(2).unwrap())
        );
        assert_eq!(
            vec![vec![2, 3, 4, 5, 6, 7, 8], vec![3, 4, 5, 6, 7]],
            parse_line(lines.get(3).unwrap())
        );
        assert_eq!(
            vec![vec![6], vec![4, 5, 6]],
            parse_line(lines.get(4).unwrap())
        );
        assert_eq!(
            vec![vec![2, 3, 4, 5, 6], vec![4, 5, 6, 7, 8]],
            parse_line(lines.get(5).unwrap())
        );

        assert_eq!(false, one_overlap(parse_line(lines.get(0).unwrap())));
        assert_eq!(false, one_overlap(parse_line(lines.get(1).unwrap())));
        assert_eq!(false, one_overlap(parse_line(lines.get(2).unwrap())));
        assert_eq!(true, one_overlap(parse_line(lines.get(3).unwrap())));
        assert_eq!(true, one_overlap(parse_line(lines.get(4).unwrap())));
        assert_eq!(false, one_overlap(parse_line(lines.get(5).unwrap())));

        assert_eq!(2, calculate_score(&lines));
    }
}
