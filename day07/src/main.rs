use std::vec::IntoIter;

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
    let commands = parse_commands(&lines);
    let children = build_tree(&commands);

    println!(
        "Result: {}",
        sum_directories_max_size(&mut children.iter(), 100000)
    );
    println!(
        "Result (part 2): {}",
        pick_directory_to_delete(&children, 70000000, 30000000)
    );
}

#[derive(Debug, Clone)]
enum Node {
    Dir { name: String, children: Vec<Node> },
    File { name: String, size: i32 },
}

impl Node {
    fn get_name(self: &Self) -> String {
        match self {
            Self::File { name, .. } => name.clone(),
            Self::Dir { name, .. } => name.clone(),
        }
    }

    fn get_size(self: &Self) -> i32 {
        match self {
            Self::File { size, .. } => *size,
            Self::Dir { children, .. } => children.iter().fold(0, |acc, el| acc + el.get_size()),
        }
    }

    fn iter(self: &Self) -> IntoIter<Node> {
        let mut res: Vec<Node> = vec![self.clone()];

        if let Node::Dir { children, .. } = self {
            res.extend(children.iter().flat_map(|it| Self::into_iter(it.clone())))
        }

        res.into_iter()
    }
}

impl IntoIterator for Node {
    type Item = Node;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let rself = &self;
        let mut res: Vec<Node> = vec![rself.clone()];

        if let Node::Dir { children, .. } = rself {
            res.extend(children.iter().flat_map(|it| Self::into_iter(it.clone())))
        }

        res.into_iter()
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::Dir {
            name: Default::default(),
            children: Default::default(),
        }
    }
}

impl TryFrom<&String> for Node {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.starts_with("dir") {
            Err("Directory".into())
        } else {
            let ls_entry = value.split_whitespace().collect::<Vec<_>>();
            Ok(Node::File {
                name: String::from(ls_entry.get(1).unwrap().trim()),
                size: ls_entry.get(0).unwrap().parse::<i32>().unwrap(),
            })
        }
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    CdParent,
    Ls(Vec<Node>),
    Unknown,
}

fn parse_commands(lines: &Vec<String>) -> Vec<Command> {
    let mut reader = lines.iter().peekable();
    let mut commands = Vec::<Command>::new();

    while let Some(elt) = reader.next() {
        let args = elt.split_whitespace().collect::<Vec<_>>();
        commands.push(match args.get(1).unwrap() {
            &"cd" => match args.get(2).unwrap() {
                &".." => Command::CdParent,
                &folder => Command::Cd(String::from(folder)),
            },
            &"ls" => {
                let mut result = Vec::<Node>::new();

                while reader.peek().is_some() && !reader.peek().unwrap().starts_with("$") {
                    if let Ok(node) = Node::try_from(reader.next().unwrap()) {
                        result.push(node);
                    }
                }

                Command::Ls(result)
            }
            _ => Command::Unknown,
        })
    }

    commands
}

fn build_tree(commands: &Vec<Command>) -> Node {
    build_children(&mut commands.iter()).get(0).unwrap().clone()
}

fn build_children<'a, T>(reader: &mut T) -> Vec<Node>
where
    T: Iterator<Item = &'a Command>,
{
    // let mut reader = commands.iter().skip(start_at);
    let mut children = Vec::<Node>::new();

    while let Some(command) = reader.next() {
        match command {
            Command::Cd(folder_name) => children.push(Node::Dir {
                name: String::from(folder_name),
                children: build_children(reader),
            }),
            Command::Ls(result) => children.extend(result.to_vec()),
            _ => return children,
        }
    }

    children
}

fn sum_directories_max_size<T>(it: &mut T, max: i32) -> i32
where
    T: Iterator<Item = Node>,
{
    it
        // Only keep dirs
        .filter(|it| match it {
            Node::Dir { .. } => true,
            _ => false,
        })
        // Get their size
        .map(|it| it.get_size())
        // Keep only those of max size
        .filter(|it| it <= &max)
        // Sum it
        .fold(0, |acc, it| acc + it)
}

fn pick_directory_to_delete(root: &Node, fs_size: i32, space_needed: i32) -> i32 {
    let total_space = root.get_size();
    let free_space = fs_size - total_space;
    let min_delete = space_needed - free_space;

    root.iter()
        // Only keep dirs
        .filter(|it| match it {
            Node::Dir { .. } => true,
            _ => false,
        })
        // Get their size
        .map(|it| it.get_size())
        // Keep only those freeing enough space
        .filter(|it| it >= &min_delete)
        // Get smallest
        .fold(total_space, |acc, it| std::cmp::min(acc, it))
}

#[cfg(test)]
mod tests {
    use crate::{
        build_tree, parse_commands, pick_directory_to_delete, sum_directories_max_size, Node,
    };

    #[test]
    fn example() {
        let lines = crate::file("tests/example.txt");
        let commands = parse_commands(&lines);
        let children = build_tree(&commands);

        println!("{:#?}", children);

        println!("{:#?}", children.iter());

        assert_eq!(
            4,
            children
                .iter()
                .filter(|it| match it {
                    Node::Dir { .. } => true,
                    _ => false,
                })
                .count()
        );

        assert_eq!(
            95437,
            sum_directories_max_size(&mut children.iter(), 100000)
        );

        assert_eq!(
            24933642,
            pick_directory_to_delete(&children, 70000000, 30000000)
        );
    }
}
