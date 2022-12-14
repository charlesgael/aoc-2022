use core::time;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    ops::{Index, IndexMut},
    process::Command,
    thread,
};

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
    fn get_round_inspections(&self, index: usize, releaf: bool) -> Vec<usize> {
        let mut round = self.0.iter().map(|it| it.items.clone()).collect::<Vec<_>>();
        let mut inspections = self.0.iter().map(|_| 0).collect::<Vec<_>>();
        let factor = self
            .0
            .iter()
            .map(|it| it.test.divisible)
            .fold(1, |acc, it| acc * it);

        let bar = ProgressBar::new(index as u64);
        bar.set_style(
            ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("=> "),
        );

        for round_idx in 0..index {
            // println!("Round {:.1} %", round_idx as f64 / index as f64 * 100f64);

            // println!("# Round {}", index);
            for (idx_monkey, monkey) in self.0.iter().enumerate() {
                // println!("Monkey {}:", idx_monkey);
                for _idx_item in 0..round.index(idx_monkey).len() {
                    *inspections.index_mut(idx_monkey) += 1;
                    let item = round[idx_monkey].remove(0);
                    // println!("  Monkey inspects an item with a worry level of {}.", item);
                    let result = monkey.operation.exec(&item) % factor;
                    // println!("    Worry level {} to {}.", monkey.operation, result);

                    // part 2 no longer divides (uncomment for part 1)
                    let bored = if releaf { result / 3 } else { result };

                    // println!(
                    //     "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                    //     bored
                    // );
                    let dest_monkey = monkey.test.monkey_for(&bored);
                    // println!(
                    //     "    Item with worry level {} is thrown to monkey {}.",
                    //     bored, dest_monkey
                    // );

                    round[dest_monkey].push(bored);
                }
            }

            bar.inc(1);
        }

        bar.finish();

        inspections
    }
}

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    test: Test,

    items: Vec<u128>,
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
            items: starting,
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
    fn monkey_for(&self, num: &u128) -> usize {
        if num % self.divisible == 0 {
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
    let mut inspections = game.get_round_inspections(20, true);
    inspections.sort();
    println!("Inspections : {:?}", inspections);
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

fn part2(game: &mut Game) -> usize {
    let mut inspections = game.get_round_inspections(10000, false);
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

        assert_eq!(10605, part1(&mut game));
        println!("{:?}", game.get_round_inspections(20, false));
    }
}
