use std::{fmt::Debug, collections::HashSet};

fn file(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|it| String::from(it))
        .collect::<Vec<String>>()
}

struct Playground {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,

    head: (i32, i32),
    tail: Vec<(i32, i32)>,

    tail_history: HashSet<(i32, i32)>
}

impl Default for Playground {
    fn default() -> Self {
        Self { 
            min_x: Default::default(),
            max_x: Default::default(),
            min_y: Default::default(),
            max_y: Default::default(),
            head: Default::default(), 
            tail: Default::default(), 
            tail_history: Default::default() 
        }
    }
}

impl Playground {
    fn new(tracking_level: usize) -> Playground {
        if tracking_level == 0 {panic!("Must track at least 1 tail")}

        let mut instance = Playground {
            tail: (0..tracking_level).map(|_| (0,0)).collect::<Vec<_>>(),
            ..Default::default()
        };
        instance.record_tail_position();
        instance
    }

    fn update_min_max(&mut self) {
        self.min_x = std::cmp::min(self.min_x, self.head.0);
        self.max_x = std::cmp::max(self.max_x, self.head.0);
        self.min_y = std::cmp::min(self.min_y, self.head.1);
        self.max_y = std::cmp::max(self.max_y, self.head.1);
    }

    fn update_tail(&mut self) {
        for i in 0..self.tail.len() {
            self.update_tail_n(i)
        }
        
        println!("progress {:?} {:?}", self.head, self.tail);
    }

    fn update_tail_n(&mut self, index: usize) {
        if index >= self.tail.len() {panic!("Impossible to update unknown tail")}

        let cl = self.tail.clone();

        let head = if index == 0 {self.head} else {self.tail[index-1]};
        let mut tail = self.tail.get_mut(index).unwrap();

        let diff_x = head.0.abs_diff(tail.0);
        let diff_y = head.1.abs_diff(tail.1);

        let diag = if diff_x != 0 && diff_y != 0 {true} else {false};

        if diff_x == 2 && diff_y == 2 {
            tail.0 = (tail.0 + head.0) / 2;
            tail.1 = (tail.1 + head.1) / 2;
        } else if diff_x == 2 {
            // Move along x axis
            tail.0 = (tail.0 + head.0) / 2;
            if diag {
                tail.1 = head.1;
            }
        } else if diff_y == 2 {
            // Move along y axis
            tail.1 = (tail.1 + head.1) / 2;
            if diag {
                tail.0 = head.0;
            }
        } else if diff_x > 2 || diff_y > 2 {
            panic!("Not supposed to happen {:?} {:?} {} {} {}", self.head, self.tail, index, diff_x, diff_y);
        }
        // else do nothing
    }

    fn record_tail_position(&mut self) -> bool {
        let last_tail = self.tail.last().unwrap();
        self.tail_history.insert((last_tail.0, last_tail.1))
    }

    fn r#move(&mut self, instruction: &Instruction) {
        for _a in 0..instruction.get_count() {
            self.head.0 += instruction.get_unit().0;
            self.head.1 += instruction.get_unit().1;
            
            self.update_min_max();
            self.update_tail();
            self.record_tail_position();
        }
    }

    fn count_tail_visited(&self) -> usize {
        self.tail_history.len()
    }
}

impl Debug for Playground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Playground [");
        let cols = self.max_x - self.min_x + 1;
        let rows = self.max_y - self.min_y + 1;

        for (idx, letter) in (0..rows * cols).rev()
            // Number to row/cols
            .map(|it| (it/cols + self.min_y, cols - 1 - it%cols + self.min_x))
            // Draw that spot
            .map(|(row, col)| {
                if self.head == (col, row) {
                    "H".into()
                } else if self.tail.contains(&(col, row)) {
                    format!("{}", self.tail.iter().enumerate().filter(|(_,el)| **el == (col, row)).next().unwrap().0+1)
                } else if row == 0 && col == 0 {
                    "s".into()
                } else if self.tail_history.contains(&(col, row)) {
                    "#".into()
                } else {
                    ".".into()
                }
            })
            .enumerate() {
            
            if idx as i32%cols == 0 {
                if f.alternate() {
                    write!(f, "\n  ");
                } else if idx != 0 {
                    write!(f, " | ");
                }
            }
            write!(f, "{}", letter);
        }
        if f.alternate() {
            write!(f, "\n");
        }
        writeln!(f, "]")
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize)
}

impl Instruction {
    fn get_unit(&self) -> (i32, i32) {
        match self {
            Self::Up(_) => (0, 1),
            Self::Down(_) => (0, -1),
            Self::Left(_) => (-1, 0),
            Self::Right(_) => (1, 0)
        }
    }

    fn get_count(&self) -> usize {
        match self {
            Self::Up(ct) => *ct,
            Self::Down(ct) => *ct,
            Self::Left(ct) => *ct,
            Self::Right(ct) => *ct
        }
    }
}

impl From<&String> for Instruction {
    fn from(str: &String) -> Self {
        let elts = str.split_whitespace().collect::<Vec<_>>();
        let nb = elts[1].parse::<usize>().unwrap();

        match elts[0] {
            "U" => Self::Up(nb),
            "D" => Self::Down(nb),
            "L" => Self::Left(nb),
            "R" => Self::Right(nb),
            _ => Self::Up(0)
        }
    }
}

fn main() {
    let lines = file("tests/myinput.txt");
    let instructions = lines.iter().map(Instruction::from).collect::<Vec<_>>();

    let mut play1 = Playground::new(1);
    instructions.iter().for_each(|it| play1.r#move(it));
    println!("Result: {}", play1.count_tail_visited());

    let mut play2 = Playground::new(9);
    instructions.iter().for_each(|it| play2.r#move(it));
    println!("Result (part 2): {}", play2.count_tail_visited());
}

#[cfg(test)]
mod tests {
    use crate::{Playground, Instruction};

    #[test]
    fn example() {
        let lines = crate::file("tests/example.txt");
        let instructions = lines.iter().map(Instruction::from).collect::<Vec<_>>();
        let mut play = Playground::new(1);

        println!("{:#?}", play);

        play.r#move(&crate::Instruction::Up(1));

        println!("{:#?}", play);

        play.r#move(&crate::Instruction::Left(2));

        println!("{:#?}", play);

        play.r#move(&crate::Instruction::Down(3));

        println!("{:#?}", play);

        play.r#move(&crate::Instruction::Right(4));

        println!("{:#?}", play);

        assert_eq!(vec![
                Instruction::Right(4),
                Instruction::Up(4),
                Instruction::Left(3),
                Instruction::Down(1),
                Instruction::Right(4),
                Instruction::Down(1),
                Instruction::Left(5),
                Instruction::Right(2)
            ],
            instructions);

        let mut play2 = Playground::new(1);
        instructions.iter().for_each(|it| play2.r#move(it));
            
        assert_eq!(13, play2.count_tail_visited());
    }

    #[test]
    fn example2() {
        let lines = crate::file("tests/example2.txt");
        let instructions = lines.iter().map(Instruction::from).collect::<Vec<_>>();
        let mut play = Playground::new(9);
        instructions.iter().for_each(|it| play.r#move(it));

        println!("{:#?}", play);
            
        assert_eq!(36, play.count_tail_visited());
    }
}
