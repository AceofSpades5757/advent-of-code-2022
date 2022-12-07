use std::str::FromStr;
use std::path::PathBuf;
use std::collections::HashMap;

const FILESYSTEM_SIZE: usize = 70_000_000;
const MINIMUM_SYSTEM_SIZE: usize = 30_000_000;

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();

    // HashMap of directories and their sizes
    let mut dirs: HashMap<PathBuf, u32> = HashMap::new();
    let mut files: Vec<File> = Vec::new();

    let mut environ = Environment::new();
    for line in lines {
        let line = line.trim();
        // Command
        if line.starts_with("$ ") {
            let command = line[2..].parse::<Command>().unwrap();
            match command {
                Command::Cd(path) => {
                    if path == PathBuf::from("..") {
                        environ.cwd.pop();
                    } else {
                        environ.cwd.push(path);
                    }
                }
                Command::Ls => {},
            }
        // ls Ouput
        } else {
            // Directory: dir <path>
            if line.starts_with("dir ") {
                // ...
            // File: <size> <file>
            } else {
                let mut file = line.parse::<File>().unwrap();
                file.path = environ.cwd.join(&file.path);
                files.push(file);
            }
        }
    }

    for file in files {
        let dir = file.path.parent().unwrap();
        if let Some(size) = dirs.get(dir) {
            dirs.insert(dir.to_owned(), size + file.size);
        } else {
            dirs.insert(dir.to_owned(), file.size);
        }
    }

    // Add children sizes to parents
    for (dir, size) in dirs.clone().iter() {
        let mut parent = dir.parent();
        while let Some(parent_dir) = parent {
            if let Some(parent_size) = dirs.get(parent_dir) {
                dirs.insert(parent_dir.to_owned(), parent_size + size);
            } else {
                dirs.insert(parent_dir.to_owned(), *size);
            }
            parent = parent_dir.parent();
        }
    }

    let mut total = 0;
    for (_dir, size) in dirs.iter() {
        if *size <= 100000 {
            total += size;
        }
    }

    let current_disk_size = dirs.get(&PathBuf::from("/")).unwrap();
    let unused_disk_size = FILESYSTEM_SIZE - (*current_disk_size as usize);
    let removal_required = MINIMUM_SYSTEM_SIZE - unused_disk_size;

    let mut smallest = 0;
    for (_dir, size) in dirs.iter() {
        if *size >= removal_required as u32 {
            if smallest == 0 || *size < smallest {
                smallest = *size;
            }
        }
    }

    smallest as i32
}

#[derive(Debug)]
pub struct Environment {
    pub cwd: PathBuf,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            cwd: PathBuf::from("/"),
        }
    }
}

#[derive(Debug)]
struct File {
    size: u32,
    path: PathBuf,
}

impl FromStr for File {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitter = s.splitn(2, ' ').collect::<Vec<&str>>();
        let size: u32 = splitter[0].parse().unwrap();
        let path = splitter[1].parse::<PathBuf>().unwrap();
        Ok(Self { size, path })
    }
}

#[derive(Debug)]
pub enum Command {
    Cd(PathBuf),
    Ls,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let command = parts.next().ok_or("No command")?;
        let args = parts.collect::<Vec<_>>();
        match command {
            "cd" => {
                if args.len() != 1 {
                    return Err("cd takes one argument".to_owned());
                }
                Ok(Command::Cd(PathBuf::from(args[0])))
            }
            "ls" => {
                if !args.is_empty() {
                    return Err("ls takes no arguments".to_owned());
                }
                Ok(Command::Ls)
            }
            _ => Err(format!("Unknown command: {}", command)),
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let answer = solve(&input);
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_compute() {
        let tests = vec![
            ("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k", 24933642)
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
