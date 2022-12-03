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
    println!("Result: {}", compute(&lines));
    println!("Result (part2): {}", compute_p2(&lines, 3));
}

fn split_half<T: Into<String>>(line: T) -> (String, String) {
    let linestr: String = line.into();
    let len = linestr.len();
    let res = linestr.split_at(len / 2);
    return (res.0.into(), res.1.into());
}

fn str_to_numbers<T: Into<String>>(line: T) -> Vec<u8> {
    let linestr: String = line.into();
    linestr
        .as_bytes()
        .to_vec()
        .into_iter()
        .map(|it| it - 64)
        .map(|it| if it < 27 { it + 26 } else { it - 32 })
        .collect()
}

fn common_item_sum(lists: Vec<String>) -> i32 {
    let mut cpy = lists
        .to_vec()
        .into_iter()
        .map(str_to_numbers)
        .collect::<Vec<Vec<u8>>>();
    let init = cpy.pop().unwrap();

    let mut in_common = cpy.into_iter().fold(init, |acc, list| {
        acc.into_iter().filter(|elt| list.contains(elt)).collect()
    });

    in_common.pop().unwrap().into()
}

fn compute(lines: &Vec<String>) -> i32 {
    lines
        .into_iter()
        .map(split_half)
        .map(|(a, b)| common_item_sum(vec![a, b]))
        .fold(0, |a, b| a + b)
}

fn compute_p2(lines: &Vec<String>, batch_size: usize) -> i32 {
    lines
        .chunks(batch_size)
        .map(|x| x.to_vec())
        .map(|elts| common_item_sum(elts))
        .fold(0, |a, b| a + b)
}

#[cfg(test)]
mod tests {
    use crate::{common_item_sum, compute, compute_p2, split_half};

    #[test]
    fn example() {
        let lines = crate::file("tests/example.txt");

        let line1 = split_half(lines.get(0).unwrap());
        assert_eq!(16, common_item_sum(vec![line1.0, line1.1]));
        let line2 = split_half(lines.get(1).unwrap());
        assert_eq!(38, common_item_sum(vec![line2.0, line2.1]));
        let line3 = split_half(lines.get(2).unwrap());
        assert_eq!(42, common_item_sum(vec![line3.0, line3.1]));
        let line4 = split_half(lines.get(3).unwrap());
        assert_eq!(22, common_item_sum(vec![line4.0, line4.1]));
        let line5 = split_half(lines.get(4).unwrap());
        assert_eq!(20, common_item_sum(vec![line5.0, line5.1]));
        let line6 = split_half(lines.get(5).unwrap());
        assert_eq!(19, common_item_sum(vec![line6.0, line6.1]));

        assert_eq!(157, compute(&lines));
        assert_eq!(70, compute_p2(&lines, 3));
    }
}
