use core::slice::Iter;

fn file(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|it| String::from(it))
        .collect::<Vec<String>>()
}

#[derive(Debug, PartialEq)]
struct Instruction {
    qty: usize,
    from: usize,
    to: usize,
}

impl<T> From<T> for Instruction
where
    T: Into<String>,
{
    fn from(string: T) -> Self {
        let elts = string
            .into()
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        Self {
            qty: elts.get(1).unwrap().parse::<usize>().unwrap(),
            from: elts.get(3).unwrap().parse::<usize>().unwrap() - 1,
            to: elts.get(5).unwrap().parse::<usize>().unwrap() - 1,
        }
    }
}

impl Instruction {
    fn execute(self: &Self, stacks: &mut Vec<Vec<String>>) {
        let size = stacks.len();
        if self.from < size && self.to < size && self.from != self.to {
            let mut from_stack = stacks.get_mut(self.from).unwrap();
            let mut moved: Vec<String> = vec![];
            for _ in 0..self.qty {
                if let Some(el) = from_stack.pop() {
                    moved.push(el);
                }
            }
            let mut to_stack = stacks.get_mut(self.to).unwrap();
            for el in moved {
                to_stack.push(el);
            }
        } else {
            println!("not possible")
        }
    }

    fn execute_p2(self: &Self, stacks: &mut Vec<Vec<String>>) {
        let size = stacks.len();
        if self.from < size && self.to < size && self.from != self.to {
            let mut from_stack = stacks.get_mut(self.from).unwrap();
            let mut moved: Vec<String> = vec![];
            for _ in 0..self.qty {
                if let Some(el) = from_stack.pop() {
                    moved.push(el);
                }
            }
            moved.reverse();
            let mut to_stack = stacks.get_mut(self.to).unwrap();
            for el in moved {
                to_stack.push(el);
            }
        } else {
            println!("not possible")
        }
    }
}

fn main() {
    let lines = file("tests/myinput.txt");

    let mut it = lines.iter();
    let mut stacks = parse_stacks(&mut it);
    let mut stacks_p2 = stacks.to_vec();
    let instructions = parse_instructions(&mut it);

    instructions
        .iter()
        .for_each(|instru| instru.execute(&mut stacks));

    println!("Result: {}", top_items(&stacks).join(""));

    instructions
        .iter()
        .for_each(|instru| instru.execute_p2(&mut stacks_p2));

    println!("Result (part 2): {}", top_items(&stacks_p2).join(""));
}

fn top_items(stacks: &Vec<Vec<String>>) -> Vec<String> {
    stacks
        .iter()
        .map(|el| el.last().unwrap_or(&"".into()).clone())
        .collect::<Vec<String>>()
}

/**
 * In charge of reading lines which contains crates
 * and stopping when getting a blank line
 * Iterator is updated for instructions reading
 */
fn parse_stacks(iterator: &mut Iter<String>) -> Vec<Vec<String>> {
    let mut stack_str: Vec<String> = Vec::new();
    loop {
        if let Some(line) = iterator.next() {
            if (line.len() == 0) {
                break;
            } else {
                stack_str.push(line.clone());
            }
        } else {
            break;
        }
    }

    // Remove the line of numbers
    stack_str.pop();

    // For each line build the items in place
    let mut res = stack_str
        .iter()
        .map(|it| {
            it.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|it| it.to_vec().get(1).unwrap().to_string())
                .map(|it| it.trim().into())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut stacks = res
        .get(0)
        .unwrap()
        .iter()
        .map(|_| vec![])
        .collect::<Vec<Vec<String>>>();

    loop {
        if let Some(line) = res.pop() {
            for (i, el) in line.iter().enumerate() {
                if el.len() > 0 {
                    stacks.get_mut(i).unwrap().push(el.clone());
                }
            }
        } else {
            break;
        }
    }

    return stacks;
}

fn parse_instructions(iterator: &mut Iter<String>) -> Vec<Instruction> {
    iterator.map(Instruction::from).collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_instructions, parse_stacks, Instruction};

    #[test]
    fn example() {
        let lines = crate::file("tests/example.txt");

        let mut it = lines.iter();
        let mut stacks = parse_stacks(&mut it);
        println!("{:?}", stacks);

        assert_eq!(vec!["[Z]", "[N]"], *stacks.get(0).unwrap());
        assert_eq!(vec!["[M]", "[C]", "[D]"], *stacks.get(1).unwrap());
        assert_eq!(vec!["[P]"], *stacks.get(2).unwrap());

        (Instruction {
            qty: 1,
            from: 1,
            to: 0,
        })
        .execute(&mut stacks);
        println!("{:?}", stacks);

        assert_eq!(vec!["[Z]", "[N]", "[D]"], *stacks.get(0).unwrap());
        assert_eq!(vec!["[M]", "[C]"], *stacks.get(1).unwrap());
        assert_eq!(vec!["[P]"], *stacks.get(2).unwrap());

        (Instruction {
            qty: 3,
            from: 0,
            to: 2,
        })
        .execute(&mut stacks);
        println!("{:?}", stacks);

        assert_eq!(Vec::<String>::new(), *stacks.get(0).unwrap());
        assert_eq!(vec!["[M]", "[C]"], *stacks.get(1).unwrap());
        assert_eq!(vec!["[P]", "[D]", "[N]", "[Z]"], *stacks.get(2).unwrap());

        (Instruction {
            qty: 2,
            from: 1,
            to: 0,
        })
        .execute(&mut stacks);
        println!("{:?}", stacks);

        assert_eq!(vec!["[C]", "[M]"], *stacks.get(0).unwrap());
        assert_eq!(Vec::<String>::new(), *stacks.get(1).unwrap());
        assert_eq!(vec!["[P]", "[D]", "[N]", "[Z]"], *stacks.get(2).unwrap());

        (Instruction {
            qty: 1,
            from: 0,
            to: 1,
        })
        .execute(&mut stacks);
        println!("{:?}", stacks);

        assert_eq!(vec!["[C]"], *stacks.get(0).unwrap());
        assert_eq!(vec!["[M]"], *stacks.get(1).unwrap());
        assert_eq!(vec!["[P]", "[D]", "[N]", "[Z]"], *stacks.get(2).unwrap());

        let instructions = parse_instructions(&mut it);
        println!("{:?}", instructions);

        assert_eq!(
            vec![
                Instruction {
                    qty: 1,
                    from: 1,
                    to: 0
                },
                Instruction {
                    qty: 3,
                    from: 0,
                    to: 2
                },
                Instruction {
                    qty: 2,
                    from: 1,
                    to: 0
                },
                Instruction {
                    qty: 1,
                    from: 0,
                    to: 1
                }
            ],
            instructions
        )
    }

    #[test]
    fn full_example() {
        let lines = crate::file("tests/example.txt");
        let mut it = lines.iter();
        let mut stacks = parse_stacks(&mut it);
        let instructions = parse_instructions(&mut it);

        assert_eq!(vec!["[Z]", "[N]"], *stacks.get(0).unwrap());
        assert_eq!(vec!["[M]", "[C]", "[D]"], *stacks.get(1).unwrap());
        assert_eq!(vec!["[P]"], *stacks.get(2).unwrap());

        instructions.get(0).unwrap().execute(&mut stacks);

        assert_eq!(vec!["[Z]", "[N]", "[D]"], *stacks.get(0).unwrap());
        assert_eq!(vec!["[M]", "[C]"], *stacks.get(1).unwrap());
        assert_eq!(vec!["[P]"], *stacks.get(2).unwrap());

        instructions.get(1).unwrap().execute(&mut stacks);

        assert_eq!(Vec::<String>::new(), *stacks.get(0).unwrap());
        assert_eq!(vec!["[M]", "[C]"], *stacks.get(1).unwrap());
        assert_eq!(vec!["[P]", "[D]", "[N]", "[Z]"], *stacks.get(2).unwrap());

        instructions.get(2).unwrap().execute(&mut stacks);

        assert_eq!(vec!["[C]", "[M]"], *stacks.get(0).unwrap());
        assert_eq!(Vec::<String>::new(), *stacks.get(1).unwrap());
        assert_eq!(vec!["[P]", "[D]", "[N]", "[Z]"], *stacks.get(2).unwrap());

        instructions.get(3).unwrap().execute(&mut stacks);

        assert_eq!(vec!["[C]"], *stacks.get(0).unwrap());
        assert_eq!(vec!["[M]"], *stacks.get(1).unwrap());
        assert_eq!(vec!["[P]", "[D]", "[N]", "[Z]"], *stacks.get(2).unwrap());
    }
}
