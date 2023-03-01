use std::{collections::HashMap, fs::read_to_string};

#[derive(PartialEq, Debug)]
enum Command {
    CdUp(),
    Cd(Node),
    Ls(Vec<Node>),
}

#[derive(PartialEq, Debug)]
struct Node {
    name: String,
    size: Option<u64>,
}

pub fn main() {
    let Ok(input) = read_to_string("src/days/day7_input.txt") else { return };

    let (_, directories) = input
        .split_terminator("$")
        .filter(|x| x.len() > 0)
        .map(Command::parse)
        .fold(
            (vec![], HashMap::<String, Vec<Node>>::new()),
            |(mut nav_list, mut nodes), curr| {
                match curr {
                    Command::CdUp() => {
                        nav_list.pop();
                    }
                    Command::Cd(x) => {
                        nav_list.push(x.name);
                    }
                    Command::Ls(children) => {
                        children.into_iter().for_each(|mut node| {
                            let child_path = [nav_list.join("/"), node.name].join("/");
                            let parent_path = nav_list.join("/");
                            node.name = child_path;
                            nodes.entry(parent_path).or_insert(vec![]).push(node);
                        });
                    }
                };
                (nav_list, nodes)
            },
        );

    let sum = directories.iter()
                    .map(|(k,_v)| {
                        size_of_node_in_tree(String::from(k), &directories)
                    }).filter(|x| *x<100000).sum::<u64>();

    println!("{:?}", sum);

    let used_disk = size_of_node_in_tree("/".to_string(), &directories);
    let disk_total =70000000 ;
    let unused_disk =  disk_total - used_disk;
    let needed_freed_disk_size = 30000000 - unused_disk;

    println!("{:?}", needed_freed_disk_size);

    let node_to_delete_size = directories.iter().fold(u64::MAX, |acc, (k, _)| {
        let dir_size = size_of_node_in_tree(k.to_string(), &directories);
        if dir_size < acc && dir_size > needed_freed_disk_size {
            return dir_size;
        }
        return acc;
    });

    println!("{}", node_to_delete_size);
}

fn size_of_node_in_tree(file_name: String, directory_tree: &HashMap<String, Vec<Node>>) -> u64 {
    let mut size = 0;
    if let Some(nodes) = directory_tree.get(&file_name) {
        for node in nodes {
            size += match node.size {
                Some(x) => x,
                None => size_of_node_in_tree(node.name.to_string(), directory_tree)
            }
        }
    }
    size
}

impl Command {
    fn parse(input: &str) -> Command {
        let command = input[1..3].to_owned();
        match command.as_str() {
            "cd" => {
                let command_value = input[4..].trim().to_owned();
                match command_value.as_str() {
                    ".." => Command::CdUp(),
                    x => Command::Cd(Node {
                        name: x.to_string(),
                        size: None,
                    })
                }
            },
            "ls" => Command::Ls({
                input.lines().skip(1).fold(vec![], |mut acc, line| {
                    let mut line = line.split_whitespace();
                    let mut n = 0;
                    let Some(next_line) = line.next() else { return acc; };

                    let size = match next_line.parse::<u64>() {
                        Ok(r) => Some(r),
                        Err(_) => None
                    };

                    let Some(name) = line.next() else { return acc; };

                    acc.push(Node {
                        name: name.to_string(),
                        size,
                    });

                    acc
                })
            }),
            _ => panic!("Unknown command '{}'", command.as_str()),
        }
    }
}
