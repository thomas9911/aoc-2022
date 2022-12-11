use std::collections::HashMap;

const MAX_SMALL_FOLDER_SIZE: usize = 100000;

const TOTAL_DISK_SPACE: usize = 70000000;
const REQUIRED_DISK_SPACE: usize = 30000000;

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Ls,
    Cd(&'a str),
    CdMoveUp,
}

#[derive(Debug, PartialEq)]
pub enum Listing<'a> {
    Directory(&'a str),
    File { size: usize, name: &'a str },
}

#[derive(Debug, PartialEq)]
pub enum Line<'a> {
    Command(Command<'a>),
    Output(Listing<'a>),
}

impl<'a> Line<'a> {
    pub fn text_as_line(text: &'a str) -> Option<Self> {
        // command
        if let Some((_, command_text)) = text.split_once("$ ") {
            if command_text.starts_with("ls") {
                return Some(Line::Command(Command::Ls));
            }

            if let Some((_, path)) = command_text.split_once("cd ") {
                if path == ".." {
                    return Some(Line::Command(Command::CdMoveUp));
                }
                return Some(Line::Command(Command::Cd(path)));
            }

            // invalid command
            return None;
        };

        // output
        if let Some((left, right)) = text.split_once(' ') {
            if left == "dir" {
                return Some(Line::Output(Listing::Directory(right)));
            } else {
                let size = left.parse().ok()?;
                return Some(Line::Output(Listing::File {
                    size,
                    name: right,
                }));
            }
        };

        None
    }

    pub fn as_command(&self) -> Option<&Command> {
        match self {
            Line::Command(command) => Some(command),
            Line::Output(_) => None,
        }
    }

    pub fn as_output(&self) -> Option<&Listing> {
        match self {
            Line::Output(listing) => Some(listing),
            Line::Command(_) => None,
        }
    }
}

struct LineEmittor<'a> {
    lines: std::str::Lines<'a>,
}

impl<'a> Iterator for LineEmittor<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line_text = self.lines.next()?;
        Line::text_as_line(line_text)
    }
}

impl<'a> LineEmittor<'a> {
    pub fn new(text: &'a str) -> LineEmittor<'a> {
        LineEmittor {
            lines: text.lines(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Directory {
    name: String,
    nested: Option<Box<FileSystem>>,
}

#[derive(Debug, PartialEq)]
pub struct File {
    name: String,
    size: usize,
}

#[derive(Debug, PartialEq)]
pub enum FileSystemItem {
    Dir(Directory),
    File(File),
}

#[derive(Debug, PartialEq, Default)]
pub struct FileSystem {
    data: HashMap<String, FileSystemItem>,
    size: usize,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem::default()
    }

    pub fn load(&mut self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut iter = LineEmittor::new(input);
        assert_eq!(Some(Line::Command(Command::Cd("/"))), iter.next());
        self.inner_load(&mut iter)
    }

    fn inner_load(&mut self, iter: &mut LineEmittor) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(line) = iter.next() {
            if let Some(command) = line.as_command() {
                match command {
                    Command::Ls => (),
                    Command::Cd(path) => {
                        if self.data.contains_key(*path) {
                            let mut sub_file_system = FileSystem::new();
                            sub_file_system.inner_load(iter)?;

                            match self.data.get_mut(*path).ok_or("invalid path")? {
                                FileSystemItem::File(_) => {
                                    return Err("file specified as dir".into())
                                }
                                FileSystemItem::Dir(dir) => {
                                    dir.nested = Some(Box::new(sub_file_system));
                                }
                            }
                        } else {
                            return Err("invalid folder".into());
                        }
                    }
                    Command::CdMoveUp => return Ok(()),
                }
            }

            if let Some(output) = line.as_output() {
                match output {
                    Listing::File { size, name } => {
                        self.data.insert(
                            name.to_string(),
                            FileSystemItem::File(File {
                                name: name.to_string(),
                                size: *size,
                            }),
                        );
                    }
                    Listing::Directory(name) => {
                        self.data.insert(
                            name.to_string(),
                            FileSystemItem::Dir(Directory {
                                name: name.to_string(),
                                nested: None,
                            }),
                        );
                    }
                }
            }
        }

        Ok(())
    }

    pub fn calculate_folder_sizes(&mut self) {
        let mut total = 0;
        for item in self.data.values_mut() {
            match item {
                FileSystemItem::File(file) => {
                    total += file.size;
                }
                FileSystemItem::Dir(dir) if dir.nested.is_some() => {
                    dir.nested.as_mut().unwrap().calculate_folder_sizes();
                    total += dir.nested.as_ref().unwrap().size;
                }
                FileSystemItem::Dir(_) => (),
            }
        }

        self.size = total;
    }

    pub fn sum_of_small_folders(&self) -> usize {
        let mut total = 0;
        for item in self.data.values() {
            match item {
                FileSystemItem::File(_) => (),
                FileSystemItem::Dir(dir) => {
                    if let Some(nested_file_system) = &dir.nested {
                        if nested_file_system.size <= MAX_SMALL_FOLDER_SIZE {
                            total += nested_file_system.size;
                        }
                        total += nested_file_system.sum_of_small_folders();
                    }
                }
            }
        }

        total
    }

    pub fn minimal_deleted_folder_size(&self, total: usize) -> usize {
        let mut minimum = usize::MAX;
        for item in self.data.values() {
            match item {
                FileSystemItem::File(_) => (),
                FileSystemItem::Dir(dir) => {
                    if let Some(nested_file_system) = &dir.nested {
                        if (TOTAL_DISK_SPACE - total + nested_file_system.size) >= REQUIRED_DISK_SPACE && nested_file_system.size < minimum {
                            minimum = nested_file_system.size
                        }
                        let inner_score = nested_file_system.minimal_deleted_folder_size(total);
                        if inner_score < minimum {
                            minimum = inner_score;
                        }
                    }
                }
            }
        }

        minimum
    }
}

pub fn day07a() -> usize {
    let input = crate::load_to_string("data/day07/day07a.txt").unwrap();

    let mut root_file_system = FileSystem::new();

    root_file_system.load(&input).unwrap();
    root_file_system.calculate_folder_sizes();

    root_file_system.sum_of_small_folders()
}

pub fn day07b() -> usize {
    let input = crate::load_to_string("data/day07/day07b.txt").unwrap();

    let mut root_file_system = FileSystem::new();

    root_file_system.load(&input).unwrap();
    root_file_system.calculate_folder_sizes();

    root_file_system.minimal_deleted_folder_size(root_file_system.size)
}
