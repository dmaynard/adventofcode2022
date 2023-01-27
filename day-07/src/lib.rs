use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::{
        complete::{alpha1, anychar, newline, one_of},
        is_alphabetic,
    },
    multi::{many1, separated_list1},
    sequence::separated_pair,
    *,
};
use std::collections::HashMap;
#[derive(Debug)]
struct FileEntry {
    path: String,
    name: String,
    size: u32,
}
#[derive(Debug)]
struct DirEntry {
    path: String,
    size: u32,
}
#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug, Clone)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}
#[derive(Debug)]
enum Files<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm."),
    )(input)?;
    Ok((input, Files::File { size, name }))
}
fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}
fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;
    Ok((input, Operation::Ls(files)))
}
fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        name => Operation::Cd(Cd::Down(name)),
    };
    Ok((input, op))
}

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, cmd))
}
fn path(context: &Vec<&str>) -> String {
    let mut p = String::new();
    p.push_str("/");
    for dir in context {
        p.push_str(dir);
        p.push_str("/");
    }
    p
}

pub fn process_part1(input: &str) -> String {
    let cmds = commands(input).unwrap().1;
    // dbg!(cmds);
    let mut context: Vec<&str> = vec![];
    let mut file_entries: Vec<FileEntry> = vec![];
    for command in cmds.iter() {
        match command {
            Operation::Cd(Cd::Root) => {
                context.clear();
            }
            Operation::Cd(Cd::Up) => {
                context.pop();
                // println!("path: {}", path(&context));
            }
            Operation::Cd(Cd::Down(name)) => {
                context.push(name);
                // println!("path: {}", path(&context));
            }
            Operation::Ls(files) => {
                for file in files.iter() {
                    match file {
                        Files::File { size, name } => {
                            file_entries.push(FileEntry {
                                path: path(&context),
                                name: name.to_string(),
                                size: *size,
                            });
                        }
                        Files::Dir(name) => {
                            let mut dir_string = String::new();
                            dir_string = path(&context);
                            dir_string.push_str(&name.to_string());
                            dir_string.push_str("/");
                            file_entries.push(FileEntry {
                                path: dir_string,
                                name: name.to_string(),
                                size: 0,
                            });
                        }
                    }
                }
            }
        }
    }

    // dbg!(file_entries);
    let total: u32 = file_entries.iter().map(|fe| fe.size).sum();

    println!(" total {}", total);
    let mut unique_dirs: HashMap<String, u32> = HashMap::new();
    for item in file_entries {
        *unique_dirs.entry(item.path).or_insert(0) += item.size
    }
    let total2: u32 = unique_dirs.iter().map(|(key, value)| value).sum();
    // dbg!(unique_dirs);
    let mut accum_dir_totals: Vec<DirEntry> = vec![];
    for (okey, oval) in &unique_dirs {
        let asize: u32 = unique_dirs
            .iter()
            .filter(|(key, _val)| (&key).starts_with(okey))
            .map(|(_key, value)| value)
            .sum();
        accum_dir_totals.push(DirEntry {
            path: okey.to_string(),
            size: asize,
        });
    }
    // dbg!(accum_dir_totals);

    let answer: u32 = accum_dir_totals
        .iter()
        .filter(|de| de.size < 100000)
        .map(|de| de.size)
        .sum();

    // dbg!(accum_dir_totals);
    accum_dir_totals.sort_by(|a, b| a.size.cmp(&b.size));

    answer.to_string()

    // "95347".to_string()
    // part 1 answer 1581595
}

pub fn process_part2(input: &str) -> String {
    let cmds = commands(input).unwrap().1;
    // dbg!(cmds);
    let mut context: Vec<&str> = vec![];
    let mut file_entries: Vec<FileEntry> = vec![];
    for command in cmds.iter() {
        match command {
            Operation::Cd(Cd::Root) => {
                context.clear();
            }
            Operation::Cd(Cd::Up) => {
                context.pop();
                // println!("path: {}", path(&context));
            }
            Operation::Cd(Cd::Down(name)) => {
                context.push(name);
                // println!("path: {}", path(&context));
            }
            Operation::Ls(files) => {
                for file in files.iter() {
                    match file {
                        Files::File { size, name } => {
                            file_entries.push(FileEntry {
                                path: path(&context),
                                name: name.to_string(),
                                size: *size,
                            });
                        }
                        Files::Dir(name) => {
                            let mut dir_string = String::new();
                            dir_string = path(&context);
                            dir_string.push_str(&name.to_string());
                            dir_string.push_str("/");
                            file_entries.push(FileEntry {
                                path: dir_string,
                                name: name.to_string(),
                                size: 0,
                            });
                        }
                    }
                }
            }
        }
    }

    // dbg!(file_entries);
    let total: u32 = file_entries.iter().map(|fe| fe.size).sum();

    println!(" total {}", total);
    let mut unique_dirs: HashMap<String, u32> = HashMap::new();
    for item in file_entries {
        *unique_dirs.entry(item.path).or_insert(0) += item.size
    }
    let total2: u32 = unique_dirs.iter().map(|(key, value)| value).sum();
    // dbg!(unique_dirs);
    let mut accum_dir_totals: Vec<DirEntry> = vec![];
    for (okey, oval) in &unique_dirs {
        let asize: u32 = unique_dirs
            .iter()
            .filter(|(key, _val)| (&key).starts_with(okey))
            .map(|(_key, value)| value)
            .sum();
        accum_dir_totals.push(DirEntry {
            path: okey.to_string(),
            size: asize,
        });
    }
    // dbg!(accum_dir_totals);

    let answer: u32 = accum_dir_totals
        .iter()
        .filter(|de| de.size < 100000)
        .map(|de| de.size)
        .sum();

    // dbg!(accum_dir_totals);
    accum_dir_totals.sort_by(|a, b| a.size.cmp(&b.size));

    let needed = 30000000 - (70000000 - total);
    let mut this_dir: u32 = 0;
    for de in accum_dir_totals {
        if de.size > needed {
            this_dir = de.size;
            break;
        }
    }

    this_dir.to_string()

    // "95347".to_string()
    // part 1 answer 1581595
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
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
7214296 k";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}
