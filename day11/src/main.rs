use std::ops::{Index, IndexMut};

fn file(path: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>()
        .chunks(7)
        .map(|chunks| chunks.to_vec())
        .collect::<Vec<Vec<_>>>()
}

#[derive(Debug)]
struct Game(Vec<Monkey>);

impl Game {
    fn get_round(&mut self, index: usize) -> Vec<Vec<u128>> {
        if self.0.iter().all(|it| it.items.len() > index) {
            // println!("# Round {}", index);
            // println!("Already calculated");
            self.0
                .iter()
                .map(|it| it.items.index(index).clone())
                .collect::<Vec<Vec<_>>>()
        } else {
            let mut round_idx = self
                .0
                .iter()
                .fold(usize::MAX, |acc, it| std::cmp::min(acc, it.items.len()))
                - 1;

            let mut next_round = self.get_round(round_idx);

            while round_idx < index {
                println!("Round {:.1} %", round_idx as f64 / index as f64 * 100f64);

                // println!("# Round {}", index);
                for (idx_monkey, monkey) in self.0.iter_mut().enumerate() {
                    // println!("Monkey {}:", idx_monkey);
                    for _idx_item in 0..next_round.index(idx_monkey).len() {
                        monkey.inspections += 1;
                        let item = next_round[idx_monkey].remove(0);
                        // println!("  Monkey inspects an item with a worry level of {}.", item);
                        let result = monkey.operation.exec(&item);
                        // println!("    Worry level {} to {}.", monkey.operation, result);

                        // part 2 no longer divides (uncomment for part 1)
                        let bored = result; // 3;

                        // println!(
                        //     "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                        //     bored
                        // );
                        let dest_monkey = monkey.test.monkey_for(bored.clone());
                        // println!(
                        //     "    Item with worry level {} is thrown to monkey {}.",
                        //     bored, dest_monkey
                        // );

                        next_round[dest_monkey].push(bored.clone());
                    }
                }

                for (idx, mon) in next_round.iter().enumerate() {
                    self.0.index_mut(idx).items.push(mon.to_vec());
                }

                round_idx += 1;
            }

            println!("Finished");

            next_round
        }
    }

    fn get_inspections(&mut self, index: usize) -> Vec<usize> {
        self.get_round(index);

        self.0
            .iter()
            .map(|it| it.inspections)
            .collect::<Vec<usize>>()
    }
}

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    test: Test,

    // Items per round
    items: Vec<Vec<u128>>,
    inspections: usize,
}

impl From<&Vec<String>> for Monkey {
    /**
    Monkey 0:
      Starting items: 79, 98
      Operation: new = old * 19
      Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3
        */
    fn from(inp: &Vec<String>) -> Self {
        if inp.len() < 6 {
            panic!("Not enough info to build a monkey {:?}", inp);
        }

        let starting = inp[1][18..]
            .to_string()
            .split(", ")
            .map(|it| it.trim().parse::<u128>().unwrap().into())
            .collect::<Vec<_>>();

        let operation: Operation = (&inp[2]).into();

        let test: Test = inp[3..=5].to_vec().into();

        Self {
            operation,
            test,
            items: vec![starting],
            inspections: 0,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(u128),
    Multiply(u128),
    Squared,
}

impl Operation {
    fn exec(&self, num: &u128) -> u128 {
        match self {
            Self::Add(i) => num + u128::from(*i),
            Self::Multiply(i) => num * u128::from(*i),
            Self::Squared => num * num,
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} by {}",
            match self {
                Self::Add(_) => "increases",
                Self::Multiply(_) => "is multiplied",
                Self::Squared => "is multiplied",
            },
            match self {
                Self::Add(i) => i.to_string(),
                Self::Multiply(i) => i.to_string(),
                Self::Squared => "itself".to_string(),
            }
        )
    }
}

impl From<&String> for Operation {
    fn from(inp: &String) -> Self {
        let mut it = inp.split_whitespace().skip(4);
        let op = it.next().unwrap_or("+");
        let num_str = it.next().unwrap_or("0");
        let num = num_str.parse::<u128>().unwrap_or(0);

        match op {
            "+" => {
                if num_str == "old" {
                    Self::Multiply(2)
                } else {
                    Self::Add(num)
                }
            }
            "*" => {
                if num_str == "old" {
                    Self::Squared
                } else {
                    Self::Multiply(num)
                }
            }
            _ => Self::Add(0),
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible: u128,
    monkey_ok: usize,
    monkey_ko: usize,
}

impl From<Vec<String>> for Test {
    fn from(inp: Vec<String>) -> Self {
        let divisible = inp[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u128>()
            .unwrap();

        let monkey_ok = inp[1]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let monkey_ko = inp[2]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Test {
            divisible,
            monkey_ok,
            monkey_ko,
        }
    }
}

impl Test {
    fn monkey_for(&self, num: u128) -> usize {
        if num % u128::from(self.divisible) == 0 {
            self.monkey_ok
        } else {
            self.monkey_ko
        }
    }
}

fn main() {
    let lines = file("tests/myinput.txt");
    let monkeys = lines.iter().map(Monkey::from).collect::<Vec<_>>();
    let mut game = Game(monkeys);

    println!("Result: {}", part1(&mut game));
    println!("Result (part 2): {}", part2(&mut game));
}

fn part1(game: &mut Game) -> usize {
    let mut inspections = game.get_inspections(20);
    inspections.sort();
    println!("Inspections : {:?}", inspections);
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

fn part2(game: &mut Game) -> usize {
    let mut inspections = game.get_inspections(10000);
    inspections.sort();
    println!("Inspections : {:?}", inspections);
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{part1, Game, Monkey};

    #[test]
    fn example() {
        let lines = crate::file("tests/example.txt");
        let monkeys = lines.iter().map(Monkey::from).collect::<Vec<_>>();
        let mut game = Game(monkeys);

        println!("{:#?}", game.0);

        assert_eq!(
            vec![
                vec![20, 23, 27, 26],
                vec![2080, 25, 167, 207, 401, 1046],
                vec![],
                vec![]
            ],
            game.get_round(1)
        );

        assert_eq!(
            vec![
                vec![695, 10, 71, 135, 350],
                vec![43, 49, 58, 55, 362],
                vec![],
                vec![]
            ],
            game.get_round(2)
        );
        assert_eq!(
            vec![
                vec![16, 18, 21, 20, 122],
                vec![1468, 22, 150, 286, 739],
                vec![],
                vec![]
            ],
            game.get_round(3)
        );
        assert_eq!(
            vec![
                vec![491, 9, 52, 97, 248, 34],
                vec![39, 45, 43, 258],
                vec![],
                vec![]
            ],
            game.get_round(4)
        );
        assert_eq!(
            vec![
                vec![15, 17, 16, 88, 1037],
                vec![20, 110, 205, 524, 72],
                vec![],
                vec![]
            ],
            game.get_round(5)
        );
        assert_eq!(
            vec![
                vec![8, 70, 176, 26, 34],
                vec![481, 32, 36, 186, 2190],
                vec![],
                vec![]
            ],
            game.get_round(6)
        );
        assert_eq!(
            vec![
                vec![162, 12, 14, 64, 732, 17],
                vec![148, 372, 55, 72],
                vec![],
                vec![]
            ],
            game.get_round(7)
        );
        assert_eq!(
            vec![
                vec![51, 126, 20, 26, 136],
                vec![343, 26, 30, 1546, 36],
                vec![],
                vec![]
            ],
            game.get_round(8)
        );
        assert_eq!(
            vec![
                vec![116, 10, 12, 517, 14],
                vec![108, 267, 43, 55, 288],
                vec![],
                vec![]
            ],
            game.get_round(9)
        );
        assert_eq!(
            vec![
                vec![91, 16, 20, 98],
                vec![481, 245, 22, 26, 1092, 30],
                vec![],
                vec![]
            ],
            game.get_round(10)
        );
        assert_eq!(
            vec![
                vec![83, 44, 8, 184, 9, 20, 26, 102],
                vec![110, 36],
                vec![],
                vec![]
            ],
            game.get_round(15)
        );
        assert_eq!(
            vec![
                vec![10, 12, 14, 26, 34],
                vec![245, 93, 53, 199, 115],
                vec![],
                vec![]
            ],
            game.get_round(20)
        );

        assert_eq!(10605, part1(&mut game));
        println!("{:?}", game.get_inspections(20));
    }
}
