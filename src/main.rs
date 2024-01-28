mod cli;

use cli::colors::*;

use clap::Parser as _;
use cli::actions::{Action, Arguments};

use std::{
    cmp::Ordering,
    env,
    io::{Error as IOError, Read},
    path::PathBuf,
    str::FromStr,
    usize,
};

use dtt::DateTime;

const DB_NAME: &str = ".todo.db.txt";
const DB_KEY_VAL_SEPERATOR: &str = "X4<'}/ghB^$M{@ugC=s~";
const DB_ENTRY_SEPERATOR: &str = "/!>(=]]4>gNdEhXm)he7";
const DB_STARTMAKER: &str = "ohIw*s-^ZP;4SYF/Wl#:{*zpKWpshX&r*VZ`-UvVJr$A3)+n{b?`(bnY;b1{u";
const INFORMATION: &str = {
    r#"
+---------------------------------------------------+
|  d-td (Dechow Todo)                               |
|  An exhaustive todo list cli tool made with rust. |
|  Crate: https://crates.io/crates/d-td/versions    |
|  Github: https://github.com/Owen-Dechow/d-td      |
+---------------------------------------------------+
"#
};
const DBINFORMATION: &str = {
    r#"d-td (Dechow Todo) Database
To get started with d-td install using: `cargo install d-td`
Crate: https://crates.io/crates/d-td/versions
Github: https://github.com/Owen-Dechow/d-td

"#
};

struct ToDoEntry(String, bool, DateTime);

struct ToDo {
    entries: Vec<ToDoEntry>,
    db_path: PathBuf,
}

impl ToDo {
    fn insert(&mut self, key: &String) {
        self.entries
            .push(ToDoEntry(key.to_string(), false, DateTime::new()));
    }

    fn save(&self) -> Result<(), IOError> {
        let mut content = String::new();

        for entry in &self.entries {
            let record = format!(
                "{}{}{}{}{}{}{}{}",
                entry.0,
                DB_KEY_VAL_SEPERATOR,
                entry.1,
                DB_KEY_VAL_SEPERATOR,
                {
                    let d = &entry.2;
                    let date = d.iso_8601.split(" ").nth(0).unwrap();
                    let hour = d.hour;
                    let min = d.minute;
                    let sec = d.second;
                    format!("{date}T{hour:0>2}:{min:0>2}:{sec:0>2}+00:00",)
                },
                DB_KEY_VAL_SEPERATOR,
                entry.2.microsecond,
                DB_ENTRY_SEPERATOR,
            );
            content.push_str(&record);
        }

        let db = format!("{}{}{}", DBINFORMATION, DB_STARTMAKER, content);
        return std::fs::write(&self.db_path, db);
    }

    fn new() -> Result<ToDo, IOError> {
        let current_dur = env::current_dir()?;
        let db_path = match ToDo::find_db(&current_dur) {
            Some(path) => path,
            None => {
                println!("New database initialized");
                PathBuf::from(DB_NAME)
            }
        };

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(&db_path)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;
        content = match content.split(DB_STARTMAKER).nth(1) {
            Some(content) => content.to_string(),
            None => content,
        };

        let mut entries = Vec::<ToDoEntry>::new();
        for l in content.split(DB_ENTRY_SEPERATOR) {
            if l == "" {
                continue;
            }

            let s = l.split(DB_KEY_VAL_SEPERATOR).collect::<Vec<&str>>();

            if s.len() != 4 {
                println!("Skipping corrupted data line: {:?}", s);
                continue;
            }

            let key = String::from(s[0]);

            let val = match bool::from_str(s[1]) {
                Ok(val) => val,
                Err(_) => {
                    println!("Skipping corrupted data: {}", s[1]);
                    continue;
                }
            };

            let mut date = match DateTime::parse(s[2]) {
                Ok(date) => date,
                Err(_) => {
                    println!("Skipping corrupted data: {}", s[2]);
                    continue;
                }
            };

            date.microsecond = match s[3].parse::<u32>() {
                Ok(micro) => micro,
                Err(_) => {
                    println!("Skipping corrupted data: {}", s[3]);
                    continue;
                }
            };

            entries.push(ToDoEntry(key, val, date));
        }

        return Ok(ToDo {
            entries: entries,
            db_path: db_path,
        });
    }

    fn toggle(&mut self, key: usize) -> Option<()> {
        match self.entries.get_mut(key) {
            Some(v) => Some(v.1 = !v.1),
            None => None,
        }
    }

    fn list(&self) {
        let mut i = 0;
        for entry in &self.entries {
            i += 1;

            match entry.1 {
                true => {
                    println!("{}", format!("{GREEN}{} [X]: {}{RESET}", i, entry.0));
                }
                false => {
                    println!("{}", format!("{RED}{} [ ]: {}{RESET}", i, entry.0));
                }
            };
        }
    }

    fn delete(&mut self, item: usize) -> Option<()> {
        if item < self.entries.len() {
            self.entries.remove(item);
            return Some(());
        } else {
            return None;
        }
    }

    fn find_db(path: &PathBuf) -> Option<PathBuf> {
        let file_path = path.join(DB_NAME);

        if file_path.is_file() {
            return Some(file_path);
        }

        if let Some(parent_path) = path.parent() {
            return ToDo::find_db(&PathBuf::from(parent_path));
        }

        None
    }

    fn movef(&mut self, index: usize, to: usize) -> Option<()> {
        let item: ToDoEntry;
        if index < self.entries.len() {
            item = self.entries.remove(index);
        } else {
            return None;
        }

        let mut new_index = to;
        if to > self.entries.len() {
            new_index = self.entries.len();
        }

        self.entries.insert(new_index, item);
        return Some(());
    }

    fn order_alpha(&mut self, reverse: bool, strict: bool) {
        let converter = match strict {
            true => |s: &String| String::from(s),
            false => |s: &String| s.to_lowercase(),
        };

        if reverse {
            self.entries
                .sort_by(|b: &ToDoEntry, a: &ToDoEntry| converter(&a.0).cmp(&converter(&b.0)));
        } else {
            self.entries
                .sort_by(|a: &ToDoEntry, b: &ToDoEntry| converter(&a.0).cmp(&converter(&b.0)));
        }
    }

    fn order_date(&mut self, reverse: bool) {
        let cmp = |a: &DateTime, b: &DateTime| -> Ordering {
            if a.year > b.year {
                return Ordering::Greater;
            } else if a.month > b.month {
                return Ordering::Greater;
            } else if a.day > b.day {
                return Ordering::Greater;
            } else if a.hour > b.hour {
                return Ordering::Greater;
            } else if a.minute > b.minute {
                return Ordering::Greater;
            } else if a.second > b.second {
                return Ordering::Greater;
            } else if a.microsecond > b.microsecond {
                return Ordering::Greater;
            } else if a.microsecond == b.microsecond {
                return Ordering::Equal;
            } else {
                return Ordering::Less;
            }
        };

        if reverse {
            self.entries
                .sort_by(|b: &ToDoEntry, a: &ToDoEntry| cmp(&a.2, &b.2));
        } else {
            self.entries
                .sort_by(|a: &ToDoEntry, b: &ToDoEntry| cmp(&a.2, &b.2));
        }
    }

    fn init() -> Result<std::fs::File, IOError> {
        return std::fs::File::create(DB_NAME);
    }

    fn destroy(&self) -> std::io::Result<()> {
        return std::fs::remove_file(&self.db_path);
    }

    fn clamp_index(&self, index: i8) -> usize {
        if self.entries.len() == 0 {
            return 0;
        }

        let max: i8 = (self.entries.len() as i8) - 1;
        let min: i8 = 0;
        if index < min {
            return (min as i8).try_into().unwrap();
        } else if index > max {
            return (max as i8).try_into().unwrap();
        } else {
            return (index as i8).try_into().unwrap();
        }
    }

    fn loop_index(&self, index: i8) -> usize {
        return ((index % self.entries.len() as i8) as usize)
            .try_into()
            .unwrap();
    }

    fn validate_index(&self, index: i8) -> Option<usize> {
        if index < 0 {
            return None;
        } else if index >= self.entries.len() as i8 {
            return None;
        } else {
            return Some(index as usize);
        }
    }
}

fn attempt_save(to_do: &ToDo, message: String) {
    match to_do.save() {
        Ok(_) => println!("{GREEN}{}{RESET}", message),
        Err(err) => println!(
            "{}",
            format!(
                "{RED}An error occurred while attempting to save data: ({}){RESET}",
                err
            )
        ),
    };
}

fn no_item_found(index: &i8) {
    println!("{}", format!("{RED} No item found {}{RESET}", index));
}

fn run_action(to_do: &mut ToDo, args: Arguments) {
    match args.action {
        Action::Add(args) => {
            to_do.insert(&args.item);
            attempt_save(
                &to_do,
                format!(
                    "Item '{}' was added to {}",
                    args.item,
                    to_do.db_path.display()
                ),
            )
        }

        Action::Delete(args) => {
            let idx;
            match to_do.validate_index(args.index - 1) {
                Some(val) => idx = val,
                None => {
                    println!("Index #{} invalid", args.index);
                    return;
                }
            }

            match to_do.delete(idx) {
                Some(_) => attempt_save(
                    &to_do,
                    format!(
                        "Item #{} was removed from {}",
                        args.index,
                        to_do.db_path.display()
                    ),
                ),
                None => {
                    println!("No item found {}", args.index);
                }
            };
        }

        Action::List(_args) => {
            to_do.list();
        }

        Action::Toggle(args) => {
            let idx = to_do.clamp_index(args.index - 1);

            match to_do.toggle(idx) {
                Some(_) => attempt_save(
                    &to_do,
                    format!(
                        "Item #{} from {} had been toggled",
                        args.index,
                        to_do.db_path.display()
                    ),
                ),
                None => no_item_found(&args.index),
            };
        }

        Action::Move(args) => {
            let idx = to_do.clamp_index(args.index - 1);
            let to = to_do.clamp_index(args.to - 1);

            match to_do.movef(idx, to) {
                Some(_) => attempt_save(
                    &to_do,
                    format!(
                        "Item #{} of {} has been moved to idx {}",
                        args.index,
                        to_do.db_path.display(),
                        args.to,
                    ),
                ),
                None => no_item_found(&args.index),
            };
        }

        Action::OrderAlpha(args) => {
            to_do.order_alpha(args.reverse, args.strict);
            attempt_save(
                &to_do,
                format!("List at {} has been alphabetized", to_do.db_path.display()),
            );
        }

        Action::OrderDate(args) => {
            to_do.order_date(args.reverse);
            attempt_save(
                &to_do,
                format!(
                    "List at {} has been ordered by date",
                    to_do.db_path.display()
                ),
            );
        }

        Action::Init(_args) => {
            match ToDo::init() {
                Ok(_file) => println!(
                    "{GREEN}New todo list initalized, run add command to get started{RESET}"
                ),
                Err(err) => println!("{}", format!("{RED}Error initalizing db: ({}){RESET}", err)),
            };
        }

        Action::Destroy(_args) => {
            let path = to_do.db_path.clone();
            match to_do.destroy() {
                Ok(_) => println!("Todo db at {} has been destroyed", path.display()),
                Err(err) => println!("Error destroying db: {}", err),
            };
        }

        Action::Declare(_args) => {
            println!("{}", format!("{GREEN}{}{RESET}", to_do.db_path.display()));
        }

        Action::Shift(args) => {
            let idx = to_do.clamp_index(args.index as i8 - 1);
            let to = to_do.loop_index(idx as i8 + args.shift);

            match to_do.movef(idx, to) {
                Some(_) => attempt_save(
                    &to_do,
                    format!(
                        "Item #{} of {} has been shifted by {} place(s)",
                        args.index,
                        to_do.db_path.display(),
                        args.shift,
                    ),
                ),
                None => no_item_found(&args.index),
            };
        }

        Action::Clear(_) => {
            to_do.entries.clear();
            attempt_save(
                &to_do,
                format!("Todo db at {} has been cleard", to_do.db_path.display()),
            );
        }

        Action::Info(_args) => {
            println!("{}", INFORMATION)
        }
    };
}

fn main() {
    let bar = String::from("-").repeat(40);
    let header = "| TODO |";
    println!("{}", format!("{BLUE}{}{}{}{RESET}", bar, header, bar));

    match Arguments::try_parse() {
        Ok(args) => match ToDo::new() {
            Ok(mut to_do) => {
                run_action(&mut to_do, args);
            }
            Err(err) => println!(
                "{RED}Could not esablish connection to database: {}{RESET}",
                err
            ),
        },
        Err(err) => println!("{}", err),
    };

    println!(
        "{BLUE}{}{}{}{RESET}",
        bar,
        String::from("-").repeat(header.len()),
        bar
    );
}
