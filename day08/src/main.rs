use std::{
    fmt::{self, write},
    ops::Index,
};

fn file(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|it| String::from(it))
        .collect::<Vec<String>>()
}

struct Forest {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}

impl Forest {
    fn new() -> Self {
        Self {
            rows: 0,
            cols: 0,
            data: Vec::<u8>::new(),
        }
    }

    fn push_row(&mut self, row: Vec<u8>) {
        if self.cols != 0 && row.len() != self.cols {
            panic!(
                "row is not matching matrice, trying to add {}, matrice is {}",
                row.len(),
                self.cols
            );
        }

        if self.cols == 0 {
            self.cols = row.len();
        }

        self.data.extend(row);
        self.rows += 1;
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        // print!("is_visible {} {}", row, col);
        if row == 0 || row + 1 == self.rows || col == 0 || col + 1 == self.cols {
            // println!(" border");
            true
        } else {
            let value = self.data[col + row * self.cols];
            // println!("  value {}", value);

            let col_v = self.data[col..]
                .iter()
                .step_by(self.cols)
                .collect::<Vec<_>>();
            let row_v = self.data[row * self.cols..(row + 1) * self.cols]
                .iter()
                .collect::<Vec<_>>();

            // println!("  col {:?}  |  {}  |  {:?}", col_v[0..row].iter().collect::<Vec<_>>(), col_v[row], col_v[row+1..self.cols].iter().collect::<Vec<_>>());
            // println!("  row {:?}  |  {}  |  {:?}", row_v[0..col].iter().collect::<Vec<_>>(), row_v[col], row_v[col+1..self.cols].iter().collect::<Vec<_>>());

            return
                // from left
                row_v[0..col].iter().all(|it| **it < value)
                ||
                // from right
                row_v[col+1..self.cols].iter().all(|it| **it < value)
                ||
                // from top
                col_v[0..row].iter().all(|it| **it < value)
                ||
                // from bottom
                col_v[row+1..self.rows].iter().all(|it| **it < value);
        }
    }

    fn count_visible(&self) -> u32 {
        (0..self.data.len())
            .into_iter()
            .map(|pos| (pos / self.cols, pos % self.cols))
            .map(|(row, col)| self.is_visible(row, col))
            .map(|it| if it { 1 } else { 0 })
            .fold(0, |acc, it| acc + it)
    }

    fn scenic_score(&self, row: usize, col: usize) -> u32 {
        // print!("is_visible {} {}", row, col);
        let value = self.data[col + row * self.cols];
        // println!("  value {}", value);

        let col_v = self.data[col..]
            .iter()
            .step_by(self.cols)
            .collect::<Vec<_>>();
        let row_v = self.data[row * self.cols..(row + 1) * self.cols]
            .iter()
            .collect::<Vec<_>>();
            
        // println!("  col {:?} ({})  |  {}  |  {:?} ({})", col_v[0..row].iter().collect::<Vec<_>>(), col_v[0..row].iter().rev().fold((true, 0), |(counting, sum), it| if counting {(**it < value, sum+1)} else {(counting, sum)}).1, col_v[row], col_v[row+1..self.cols].iter().collect::<Vec<_>>(), col_v[row+1..self.rows].iter().fold((true, 0), |(counting, sum), it| if counting {(**it < value, sum+1)} else {(counting, sum)}).1);
        // println!("  row {:?} ({})  |  {}  |  {:?} ({})", row_v[0..col].iter().collect::<Vec<_>>(), row_v[0..col].iter().rev().fold((true, 0), |(counting, sum), it| if counting {(**it < value, sum+1)} else {(counting, sum)}).1, row_v[col], row_v[col+1..self.cols].iter().collect::<Vec<_>>(),row_v[col+1..self.cols].iter().fold((true, 0), |(counting, sum), it| if counting {(**it < value, sum+1)} else {(counting, sum)}).1);

        row_v[0..col].iter().rev().fold((true, 0), |(counting, sum), it| if counting {(**it < value, sum+1)} else {(counting, sum)}).1
        *
        row_v[col+1..self.cols].iter().fold((true, 0), |(counting, sum), it| if counting {(**it < value, sum+1)} else {(counting, sum)}).1
        *
        col_v[0..row].iter().rev().fold((true, 0), |(counting, sum), it| if counting {(**it < value, sum+1)} else {(counting, sum)}).1
        *
        col_v[row+1..self.rows].iter().fold((true, 0), |(counting, sum), it| if counting {(**it < value, sum+1)} else {(counting, sum)}).1
    }

    fn best_scenic_score(&self) -> u32 {
        (0..self.data.len())
            .into_iter()
            .map(|pos| (pos / self.cols, pos % self.cols))
            .map(|(row, col)| self.scenic_score(row, col))
            .fold(0, |acc, it| std::cmp::max(acc, it))
    }
}

impl fmt::Debug for Forest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Forest [");
        if self.cols > 0 {
            if f.alternate() {
                for (idx, it) in self.data.iter().enumerate() {
                    if idx % self.cols == 0 {
                        write!(f, "\n ");
                    }
                    write!(f, " {}", it);
                }
            } else {
                write!(f, "{:?}", self.data);
            }
        } else {
            write!(f, " Empty ");
        }
        write!(f, "\n]")
    }
}

fn main() {
    let lines = file("tests/myinput.txt");
    let forest = parse_forest(&lines);

    println!("Result: {}", forest.count_visible());
    println!("Result (part 2): {}", forest.best_scenic_score());
}

fn parse_forest(lines: &Vec<String>) -> Forest {
    let mut forest = Forest::new();

    for line in lines {
        forest.push_row(
            line.chars()
                .into_iter()
                .map(|it| String::from(it).parse::<u8>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    forest
}

#[cfg(test)]
mod tests {
    use crate::parse_forest;

    #[test]
    fn example() {
        let lines = crate::file("tests/example.txt");
        let forest = parse_forest(&lines);

        println!("{:#?}", forest);

        assert_eq!(true, forest.is_visible(0, 0));
        assert_eq!(true, forest.is_visible(0, 1));
        assert_eq!(true, forest.is_visible(0, 2));
        assert_eq!(true, forest.is_visible(0, 3));
        assert_eq!(true, forest.is_visible(0, 4));

        assert_eq!(true, forest.is_visible(1, 0));
        assert_eq!(true, forest.is_visible(1, 1));
        assert_eq!(true, forest.is_visible(1, 2));
        assert_eq!(false, forest.is_visible(1, 3));
        assert_eq!(true, forest.is_visible(1, 4));

        assert_eq!(true, forest.is_visible(2, 0));
        assert_eq!(true, forest.is_visible(2, 1));
        assert_eq!(false, forest.is_visible(2, 2));
        assert_eq!(true, forest.is_visible(2, 3));
        assert_eq!(true, forest.is_visible(2, 4));

        assert_eq!(true, forest.is_visible(3, 0));
        assert_eq!(false, forest.is_visible(3, 1));
        assert_eq!(true, forest.is_visible(3, 2));
        assert_eq!(false, forest.is_visible(3, 3));
        assert_eq!(true, forest.is_visible(3, 4));

        assert_eq!(true, forest.is_visible(4, 0));
        assert_eq!(true, forest.is_visible(4, 1));
        assert_eq!(true, forest.is_visible(4, 2));
        assert_eq!(true, forest.is_visible(4, 3));
        assert_eq!(true, forest.is_visible(4, 4));

        assert_eq!(21, forest.count_visible());

        assert_eq!(4, forest.scenic_score(1, 2));
        assert_eq!(8, forest.scenic_score(3, 2));

        assert_eq!(8, forest.best_scenic_score())
    }
}
