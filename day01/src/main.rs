use std::fs;

fn file(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|it| String::from(it))
        .collect::<Vec<String>>()
}

fn main() {
    let groups = group_elves(file("tests/myinput.txt"));
    println!("Top calories: {}", top_calories(&groups));
    println!("Top 3 calories: {}", top_n_calories(&groups, 3));
}

fn group_elves(lines: Vec<String>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];

    let mut current = 0;
    for line in lines {
        if line.len() == 0 {
            res.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    res.push(current);

    return res;
}

fn top_calories(groups: &Vec<i32>) -> i32 {
    let mut cpy = groups.to_vec();
    cpy.sort();
    cpy.pop().unwrap()
}

fn top_n_calories(groups: &Vec<i32>, nb: usize) -> i32 {
    let mut cpy = groups.to_vec();
    cpy.sort();

    let mut sum = 0;
    for i in 0..nb {
        if let Some(n) = cpy.pop() {
            sum += n;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use crate::{file, group_elves, top_calories, top_n_calories};

    #[test]
    fn test1() {
        let groups = group_elves(file("tests/test1.txt"));
        assert_eq!(6000, *groups.get(0).unwrap());
        assert_eq!(4000, *groups.get(1).unwrap());
        assert_eq!(11000, *groups.get(2).unwrap());
        assert_eq!(24000, *groups.get(3).unwrap());
        assert_eq!(10000, *groups.get(4).unwrap());

        let top_calories_res = top_calories(&groups);
        assert_eq!(24000, top_calories_res);

        let top_3_calories_res = top_n_calories(&groups, 3);
        assert_eq!(45000, top_3_calories_res);
    }
}
