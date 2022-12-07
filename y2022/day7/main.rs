fn main() {
    println!("{:?}", solve(INPUT));
}

const INPUT: &str = "\
$ cd /
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

use std::collections::HashMap;

type Size = u64;
type DirContents<'a> = HashMap<&'a str, Metadata<'a>>;
type Dir<'a> = (&'a str, DirContents<'a>);

#[derive(Debug)]
enum Metadata<'a> {
    Dir(DirContents<'a>),
    File(Size),
}

struct Entry<'a> {
    name: &'a str,
    metadata: Metadata<'a>,
}

impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = ();

    fn try_from(s: &'a str) -> Result<Self, ()> {
        if let Some(name) = s.strip_prefix("dir ") {
            return Ok(Entry {
                name,
                metadata: Metadata::Dir(DirContents::new()),
            });
        }
        if let Some((size, name)) = s.split_once(' ') {
            let size = size.parse().map_err(drop)?;
            return Ok(Entry {
                name,
                metadata: Metadata::File(size),
            });
        }
        Err(())
    }
}

fn go_up<'a>(path: &mut Vec<Dir<'a>>, last: Dir<'a>) -> Result<Dir<'a>, Dir<'a>> {
    if let Some(mut parent) = path.pop() {
        let (name, contents) = last;
        parent.1.insert(name, Metadata::Dir(contents));
        Ok(parent)
    } else {
        Err(last)
    }
}

fn collapse<'a>(path: &mut Vec<Dir<'a>>, mut last: Dir<'a>) -> Dir<'a> {
    loop {
        match go_up(path, last) {
            Ok(parent) => last = parent,
            Err(complete) => break complete,
        }
    }
}

fn read(s: &str) -> DirContents<'_> {
    let mut curr_path = Vec::new();
    let mut last = ("", HashMap::new());

    for line in s.lines() {
        if let Some(cmd) = line.strip_prefix("$ ") {
            if cmd == "ls" {
                continue;
            }

            let (cd, dest) = cmd.split_once(' ').unwrap();
            assert_eq!(cd, "cd");

            match dest {
                "/" => last = collapse(&mut curr_path, last),
                ".." => last = go_up(&mut curr_path, last).unwrap(),
                _ => {
                    curr_path.push(last);
                    last = (dest, HashMap::new());
                }
            }

            continue;
        }

        let Entry { name, metadata } = Entry::try_from(line).unwrap();
        last.1.insert(name, metadata);
    }

    collapse(&mut curr_path, last).1
}

enum Traverse<'a> {
    Enter(&'a str),
    File(&'a str, Size),
    Leave(&'a str),
}

fn join<'a>(it: impl Iterator<Item = &'a str>, sep: char) -> String {
    let mut ret = String::new();
    it.for_each(|x| { ret.push(sep); ret += x });
    ret
}

fn traverse_files<'a>(tree: &'a DirContents<'_>, op: &mut impl FnMut(Traverse<'a>)) {
    for (&name, metadata) in tree {
        match metadata {
            Metadata::Dir(contents) => {
                op(Traverse::Enter(name));
                traverse_files(contents, op);
                op(Traverse::Leave(name));
            }
            &Metadata::File(size) => op(Traverse::File(name, size)),
        }
    }
}

const THRESHOLD: Size = 100_000;
const TOTAL_SPACE: Size = 70_000_000;
const REQUIRED: Size = 30_000_000;
const LIMIT: Size = TOTAL_SPACE - REQUIRED;

fn solve(input: &str) -> (Size, Size) {
    let tree = read(input);

    let mut sizes = HashMap::new();
    let mut curr_size = 0;
    let mut path = Vec::new();
    let mut op = |tr| match tr {
        Traverse::Enter(name) => {
            path.push((name, curr_size));
            curr_size = 0;
        }
        Traverse::File(_name, size) => curr_size += size,
        Traverse::Leave(_) => {
            sizes.insert(join(path.iter().map(|&(name, _sz)| name), '/'), curr_size);
            let (_, parent_size) = path.pop().unwrap();
            curr_size += parent_size;
        }
    };

    traverse_files(&tree, &mut op);
    sizes.insert(String::new(), curr_size);

    let answer1 = sizes.values().filter(|&&sz| sz <= THRESHOLD).sum();

    let total = curr_size;
    assert!(total > LIMIT);
    let to_delete = total - LIMIT;
    let answer2 = sizes
        .values()
        .copied()
        .filter(move |&size| size >= to_delete)
        .min()
        .unwrap();

    (answer1, answer2)
}
