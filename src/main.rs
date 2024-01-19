mod cli;
use clap::Parser as _;
use cli::actions::{Action, Arguments};

use std::{
    env,
    io::{Error as IOError, ErrorKind::AlreadyExists as IOAlreadyExists, Read},
    path::PathBuf,
    str::FromStr,
    usize,
};

use colored::Colorize;

use datetime::{Instant, LocalDateTime};

const DB_NAME: &str = ".todo.db.txt";
const DB_KEY_VAL_SEPERATOR: &str = "X4<'}/ghB^$M{@ugC=s~";
const DB_ENTRY_SEPERATOR: &str = "/!>(=]]4>gNdEhXm)he7";

struct ToDoEntry(String, bool, LocalDateTime);

struct ToDo {
    entries: Vec<ToDoEntry>,
    db_path: PathBuf,
}

impl ToDo {
    fn insert(&mut self, key: &String) {
        self.entries
            .push(ToDoEntry(key.to_string(), false, LocalDateTime::now()));
    }

    fn save(&self) -> Result<(), IOError> {
        let mut content = String::new();

        for entry in &self.entries {
            let time_instant = entry.2.to_instant();

            let record = format!(
                "{}{}{}{}{}{}",
                entry.0,
                DB_KEY_VAL_SEPERATOR,
                entry.1,
                DB_KEY_VAL_SEPERATOR,
                format!("{}:{}", time_instant.seconds(), time_instant.milliseconds()),
                DB_ENTRY_SEPERATOR
            );
            content.push_str(&record);
        }
        return std::fs::write(&self.db_path, content);
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

        let mut entries = Vec::<ToDoEntry>::new();
        for l in content.split(DB_ENTRY_SEPERATOR) {
            if l == "" {
                continue;
            }

            let s = l.split(DB_KEY_VAL_SEPERATOR).collect::<Vec<&str>>();

            if s.len() != 3 {
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

            let date = {
                let s = s[2].split(":").collect::<Vec<&str>>();
                if s.len() != 2 {
                    println!("Skipping corrupted data line: {:?}", s);
                    continue;
                }

                let seconds = match i64::from_str(s[0]) {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Skipping corrupted data line: {}", s[0]);
                        continue;
                    }
                };

                let milliseconds = match i16::from_str(s[1]) {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Skipping corrupted data line: {}", s[1]);
                        continue;
                    }
                };

                let instant = Instant::at_ms(seconds, milliseconds);

                // return local date time
                LocalDateTime::from_instant(instant)
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
                    println!("{}", format!("{} [X]: {}", i, entry.0).green());
                }
                false => {
                    println!("{}", format!("{} [ ]: {}", i, entry.0).red());
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
        if reverse {
            self.entries
                .sort_by(|b: &ToDoEntry, a: &ToDoEntry| a.2.cmp(&b.2));
        } else {
            self.entries
                .sort_by(|a: &ToDoEntry, b: &ToDoEntry| a.2.cmp(&b.2));
        }
    }

    fn init() -> Result<std::fs::File, IOError> {
        if PathBuf::from(DB_NAME).is_file() {
            return Err(IOError::new(
                IOAlreadyExists,
                "Todo db for current directory already exists",
            ));
        } else {
            return std::fs::File::create(DB_NAME);
        }
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
        Ok(_) => println!("{}", message.green()),
        Err(err) => println!(
            "{}",
            format!("An error occurred while attempting to save data: ({})", err).red()
        ),
    };
}

fn no_item_found(index: &i8) {
    println!("{}", format!("No item found {}", index).red());
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
                Ok(_) => println!(
                    "{}",
                    format!("New todo list initalized, run add command to get started").green()
                ),
                Err(err) => println!("{}", format!("Error initalizing db: ({})", err).red()),
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
            println!("{}", format!("{}", to_do.db_path.display()).green());
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
    };
}

fn main() {
    let bar = String::from("-").repeat(40);
    let header = "| TODO |";
    println!("{}", format!("{}{}{}", bar, header, bar).blue());

    let args = Arguments::parse();
    let mut to_do = ToDo::new().expect("Initialisation of db failed");

    run_action(&mut to_do, args);
    println!(
        "{}",
        format!("{}{}{}", bar, String::from("-").repeat(header.len()), bar).blue()
    );
}
