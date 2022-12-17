use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn read_size(line_str: &String) -> u32 {
    let (first, _) = line_str.split_once(" ")
        .expect("Malformed listing entry");
    u32::from_str_radix(first, 10)
        .expect("Impossible to parse")
}

#[test]
fn test_read_size() {
    assert_eq!(14848514, read_size(&"14848514 b.txt".to_string()));
}

enum ElveDeviceCommand {
    ToSubDir(String),
    ToParentDir,
    ToRootDir,
    ListDir
}

fn parse_command(cmd: &str) -> ElveDeviceCommand {
    if cmd.starts_with("cd ") {
        let path = &cmd[3..];
        if path == DELIMITER {
            ElveDeviceCommand::ToRootDir
        }
        else if path == ".." {
            ElveDeviceCommand::ToParentDir
        }
        else {
            ElveDeviceCommand::ToSubDir(path.to_string())
        }
    }
    else if cmd == "ls" {
        ElveDeviceCommand::ListDir
    }
    else {
        panic!("Unsupported command {}", cmd);
    }
}

// Root folder
static DELIMITER : &str = "/";

fn navigate(pwd: String, command: ElveDeviceCommand) -> Option<String> {
    match command {
        ElveDeviceCommand::ToRootDir => {
            Some(DELIMITER.to_string())
        },
        ElveDeviceCommand::ToParentDir => {
            Some(to_parent_dir(&pwd))
        },
        ElveDeviceCommand::ToSubDir(dst) => {
            Some(to_sub_dir(&pwd, &dst))
        },
        ElveDeviceCommand::ListDir => None
    }
}

fn to_parent_dir(pwd: &String) -> String {
    let folders = pwd.split(DELIMITER).collect::<Vec<&str>>();
    let new_folder_count = folders.len() - 1;
    if new_folder_count > 1 {
        folders[0..new_folder_count].join(DELIMITER)
    }
    else {
        DELIMITER.to_string()
    }
}

#[test]
fn test_parentdir() {
    assert_eq!("/", to_parent_dir(&"/foo".to_string()));
    assert_eq!("/foo", to_parent_dir(&"/foo/bar".to_string()));
}

fn to_sub_dir(pwd: &String, dst: &String) -> String {
    let mut new_pwd = pwd.clone();
    if pwd != DELIMITER {
        new_pwd.push_str(DELIMITER);
    }
    new_pwd.push_str(&dst);
    new_pwd
}

#[test]
fn test_subdir() {
    assert_eq!("/foo", to_sub_dir(&"/".to_string(), &"foo".to_string()));
    assert_eq!("/foo/bar", to_sub_dir(&"/foo".to_string(), &"bar".to_string()));
}

fn ascending(folder: &String) -> Vec<String> {    
    let tokens : Vec<&str> = folder.split(DELIMITER).collect();
    if tokens.len() < 2 {
        panic!("what?")
    }
    let mut folders : Vec<String> = Vec::with_capacity(tokens.len());
    folders.push(DELIMITER.to_string());
    let mut advance : String = String::new(); 
    for i in 1..tokens.len() {
        if !tokens[i].is_empty() {
            advance.push_str(&DELIMITER.to_string());
            advance.push_str(tokens[i]);
            folders.push(advance.clone())
        }
    }
    folders
}

#[test]
fn test_ascending_3() {
    let folders = ascending(&"/foo/bar/baz".to_string());
    assert!(folders.contains(&"/".to_string()));
    assert!(folders.contains(&"/foo".to_string()));
    assert!(folders.contains(&"/foo/bar".to_string()));
    assert!(folders.contains(&"/foo/bar/baz".to_string()));
    assert_eq!(4, folders.len());
}

#[test]
fn test_ascending_0() {
    let folders = ascending(&"/".to_string());
    assert!(folders.contains(&"/".to_string()));
    assert_eq!(1, folders.len());
}

#[test]
fn test_ascending_1() {
    let folders = ascending(&"/foo".to_string());
    assert!(folders.contains(&"/".to_string()));
    assert!(folders.contains(&"/foo".to_string()));
    assert_eq!(2, folders.len());
}

fn flush_sizes(by_folder: &mut HashMap<String, u32>, size: u32, folder: &String) -> () {
    for f in ascending(folder) {
        let val = by_folder.get(&f);
        let new_size : u32 = match val {
            None => size,
            Some(old_size) => size + old_size
        };
        //println!("Insert {} in {}", new_size, &f);
        by_folder.insert(f, new_size);
    }
}

fn process(file_name: &String, folder_max_size: u32) -> u32 {

    // IO variables
    let f = File::open(file_name).expect("Unable to open file");
    let mut reader = BufReader::new(f);
    let mut line_str = String::new();

    // Folders and corresponding size
    let mut folders : HashMap<String, u32> = HashMap::new();
    // Current folder
    let mut pwd     : Box<String> = Box::new(DELIMITER.to_string());
    // Output mode
    let mut listing : bool = false;
    // Total size
    let mut total_size: u32 = 0;

    loop {
        let len = reader.read_line(&mut line_str)
            .expect("Unable to read line");
        let line_length = line_str.len();
        if len == 0 {
            break; // EOF
        }
        // Processing command
        else if line_str.starts_with("$") {
            if listing {
                flush_sizes(&mut folders, total_size, &pwd);
                total_size = 0;
                listing = false;
            }
            let cmd = parse_command(&line_str[2..(line_length-1)]);
            match cmd {
                ElveDeviceCommand::ListDir => {
                    listing = true
                },
                _ => {
                    let new_path = navigate(*pwd, cmd).expect("No new path");
                    //println!("New path {}", new_path);
                    pwd = Box::new(new_path);
                }
            };
        }
        // Reading output
        else if listing {
            if ! line_str.starts_with("dir") {
                total_size += read_size(&line_str);
            }
        }
        else {
            panic!("Invalid state");
        }

        line_str.clear();
    }
    if listing {
        flush_sizes(&mut folders, total_size, &pwd);
    }

    // Computation
    //println!("Result is {:?}", folders);
    folders.values()
        .map(|e| *e)
        .filter(|size| size <= &folder_max_size)
        .reduce(|acc, e| acc + e)
        .expect("No value?")
}

#[test]
fn test_1() {
    let rs = process(&"test.log".to_string(), 100000);
    assert_eq!(95437, rs);
}

fn main() {
    // Arguments parsing
    let arguments = env::args().collect::<Vec<String>>();
    let mut arg_iter = arguments.iter();
    arg_iter.next().expect("Command name");
    let file_name = arg_iter.next().expect("No file name given");
    let folder_max_size : u32 = arg_iter.next()
        .map_or(100000, |arg| arg.parse().expect("Invalid size"));

    let result = process(file_name, folder_max_size);
    println!("Result is {}", result);
}
