mod cli;
use clap::Parser as _;
use cli::actions::{Action, Arguments};

use std::{
    env,
    io::{Error as IOError, ErrorKind::AlreadyExists as IOAlreadyExists, Read},
    path::PathBuf,
};

const DB_NAME: &str = ".todo.db.txt";
const DB_KEY_VAL_SEPERATOR: &str = "X4<'}/ghB^$M{@ugC=s~";
const DB_ENTRY_SEPERATOR: &str = "/!>(=]]4>gNdEhXm)he7";

struct ToDoEntry(String, bool);

struct ToDo {
    entries: Vec<ToDoEntry>,
    db_path: PathBuf,
}

impl ToDo {
    fn insert(&mut self, key: &String) {
        self.entries.push(ToDoEntry(key.to_string(), false));
    }

    fn save(&self) -> Result<(), IOError> {
        let mut content = String::new();
        for entry in &self.entries {
            let record = format!(
                "{}{}{}{}",
                entry.0, DB_KEY_VAL_SEPERATOR, entry.1, DB_ENTRY_SEPERATOR
            );
            content.push_str(&record);
        }
        return std::fs::write(&self.db_path, content);
    }

    fn new() -> Result<ToDo, IOError> {
        let current_dur = env::current_dir()?;
        let db_path = match ToDo::find_db(&current_dur) {
            Some(path) => path,
            None => PathBuf::from(DB_NAME),
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
            } else if !l.contains(DB_KEY_VAL_SEPERATOR) {
                println!("Skipping corrupted data: {}", l);
                continue;
            }

            let s = l.split(DB_KEY_VAL_SEPERATOR).collect::<Vec<&str>>();
            entries.push(ToDoEntry(s[0].to_string(), s[1] == "true"));
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
            if entry.1 {
                print!("{} [x]: ", i)
            } else {
                print!("{} [ ]: ", i)
            }
            println!("{}", entry.0)
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

    fn order(&mut self, reverse: bool) {
        self.entries
            .sort_by(|a: &ToDoEntry, b: &ToDoEntry| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

        if reverse {
            self.entries.reverse();
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

    fn destroy(self) -> std::io::Result<()> {
        return std::fs::remove_file(self.db_path);
    }
}

fn attempt_save(to_do: &ToDo, message: String) {
    match to_do.save() {
        Ok(_) => println!("{}", message),
        Err(err) => println!("An error occurred while attempting to save data: ({})", err),
    };
}

fn main() {
    let args = Arguments::parse();
    let mut to_do = ToDo::new().expect("Initialisation of db failed");

    let bar = String::from("-").repeat(40);
    println!("{} TODO {}", bar, bar);

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
            match to_do.delete(args.index - 1) {
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
            match to_do.toggle(args.index - 1) {
                Some(_) => attempt_save(
                    &to_do,
                    format!(
                        "Item #{} from {} had been toggled",
                        args.index,
                        to_do.db_path.display()
                    ),
                ),
                None => {
                    println!("No item found {}", args.index);
                }
            };
        }
        Action::Move(args) => {
            match to_do.movef(args.index - 1, args.to - 1) {
                Some(_) => attempt_save(
                    &to_do,
                    format!(
                        "Item #{} of {} has been moved to idx {}",
                        args.index,
                        to_do.db_path.display(),
                        args.to,
                    ),
                ),
                None => {
                    println!("No item found {}", args.index);
                }
            };
        }
        Action::Order(args) => {
            to_do.order(args.reverse);
            attempt_save(
                &to_do,
                format!("List at {} has been alphabetized", to_do.db_path.display()),
            );
        }
        Action::Init(_args) => match ToDo::init() {
            Ok(_) => print!("New todo list initalized, run add command to get started"),
            Err(err) => print!("Error initalizing db: ({})", err),
        },
        Action::Destroy(_args) => {
            let path = to_do.db_path.clone();
            match to_do.destroy() {
                Ok(_) => println!("Todo db at {} has been destroyed", path.display()),
                Err(err) => println!("Error destroying db: {}", err),
            };
        }
    };

    println!("{}------{}", bar, bar);
}
