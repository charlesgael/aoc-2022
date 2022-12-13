use std::ops::Index;

fn file(path: &str) -> Vec<Command> {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|it| String::from(it))
        .map(Command::from)
        .collect::<Vec<_>>()
}

struct Program(Vec<Command>);

impl Program {
    fn time_at(&self, index: usize) -> usize {
        self.0[0..index]
            .iter()
            .map(Command::get_duration)
            .fold(0, |acc, it| acc + it)
    }

    fn index_at(&self, time: usize) -> usize {
        let mut cycles = 0usize;
        let mut idx = 0;
        for cmd in self.0.iter() {
            cycles += cmd.get_duration();
            if cycles >= time {
                break;
            }
            idx += 1;
        }

        idx
    }

    fn commands_at(&self, time: usize) -> Vec<Command> {
        Vec::from(&self.0[0..self.index_at(time)])
    }

    fn register_at(&self, time: usize) -> i32 {
        1 + self
            .commands_at(time)
            .iter()
            .map(|it| match it {
                Command::Addx(el) => *el,
                _ => 0,
            })
            .fold(0, |acc, el| acc + el)
    }

    fn signal_strength(&self, index: usize) -> i32 {
        self.register_at(index) * index as i32
    }

    fn raw_draw(&self, length: usize, width: usize) -> String {
        let mut line = "".to_string();
        let mut reg_x = 1i32;

        for cmd in self.0.iter() {
            println!("new command: {:?}", cmd);
            for _ in 0..cmd.get_duration() {
                let pixel = (line.len() % width) as i32;
                line += if pixel >= reg_x - 1 && pixel <= reg_x + 1 {
                    "#"
                } else {
                    " "
                };
                println!(
                    "pixel: {} x: {} adding: {}",
                    pixel,
                    reg_x,
                    line.chars().last().unwrap()
                );
                if line.len() == length {
                    return line;
                }
            }
            reg_x += match cmd {
                Command::Addx(el) => *el,
                _ => 0,
            };
            println!("new x: {}", reg_x);
        }

        line
    }

    fn draw(&self, length: usize, width: usize) -> String {
        self.raw_draw(length, width)
            .chars()
            .collect::<Vec<_>>()
            .chunks(width)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Clone)]
enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn get_duration(&self) -> usize {
        match self {
            Self::Addx(_) => 2,
            _ => 1,
        }
    }
}

impl<T> From<T> for Command
where
    T: Into<String>,
{
    fn from(input: T) -> Self {
        let str = input.into();
        let mut it = str.split_whitespace();
        if let Some(cmd) = it.next() {
            if cmd == "addx" {
                if let Some(num_str) = it.next() {
                    if let Ok(num) = num_str.parse::<i32>() {
                        return Self::Addx(num);
                    }
                }
            }
        }

        Self::Noop
    }
}

fn main() {
    let lines = file("tests/myinput.txt");
    let prog = Program(lines);

    println!("Result: {}", part1(&prog));

    println!("Result (part 2) [\n{}\n]", prog.draw(240, 40));
}

fn part1(prog: &Program) -> i32 {
    prog.signal_strength(20)
        + prog.signal_strength(60)
        + prog.signal_strength(100)
        + prog.signal_strength(140)
        + prog.signal_strength(180)
        + prog.signal_strength(220)
}

#[cfg(test)]
mod tests {
    use crate::Program;

    #[test]
    fn example() {
        let lines = crate::file("tests/example.txt");
        let prog = Program(lines);

        println!("{:?}", prog.0);

        println!("{:?}", prog.commands_at(20));

        assert_eq!(21, prog.register_at(20));
        assert_eq!(19, prog.register_at(60));
        assert_eq!(18, prog.register_at(100));
        assert_eq!(21, prog.register_at(140));
        assert_eq!(16, prog.register_at(180));
        assert_eq!(18, prog.register_at(220));

        assert_eq!(420, prog.signal_strength(20));
        assert_eq!(1140, prog.signal_strength(60));
        assert_eq!(1800, prog.signal_strength(100));
        assert_eq!(2940, prog.signal_strength(140));
        assert_eq!(2880, prog.signal_strength(180));
        assert_eq!(3960, prog.signal_strength(220));

        println!("{}", prog.draw(240, 40));
        assert_eq!(
            vec![
                "##..##..##..##..##..##..##..##..##..##..".to_string(),
                "###...###...###...###...###...###...###.".to_string(),
                "####....####....####....####....####....".to_string(),
                "#####.....#####.....#####.....#####.....".to_string(),
                "######......######......######......####".to_string(),
                "#######.......#######.......#######.....".to_string()
            ]
            .join("\n"),
            prog.draw(240, 40)
        )
    }
}
