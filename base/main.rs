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
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let lines = crate::file("tests/example.txt");
    }
}
