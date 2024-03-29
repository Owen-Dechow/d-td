# d-td (Dechow TODO)
An exhaustive todo list cli tool made with rust.

## Installation
```
cargo install d-td
```
* crates.io page: https://crates.io/crates/d-td/versions
* github repo: https://github.com/Owen-Dechow/d-td

## CLI Help Menu
```
Usage: d-td <COMMAND>

Commands:
  add          Add a new item
  delete       Delete an item
  clear        Clear all items from list
  toggle       Toggle the state of an item
  list         List all items
  move         Move item from one position to another
  shift        Shift an item from one position to another
  order-alpha  Order list alphabetically
  order-date   Order list by date
  declare      Get the path of targeted db
  init         Initialize new list at current position
  destroy      Destroy db
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## DB Information
*d-td* is use a flat txt file (`.todo.db.txt`) to save the its entries. Before initalizing a new db *d-td* will attempt to find a suitable db in a parent directory, for example: if your current working directory is `Users/bob/desktop/pig/icecream/` and there is a *d-td* db in `Users/bob/desktop/` *d-td* will not create a new db it will instead use the db in `Users/bob/desktop/`. Each db entry will have the following information: `text`, `status` and `timecreated`, separated by the following pattern `X4<'}/ghB^$M{@ugC=s~` this pattern is calld DB_KEY_VAL_SEPERATOR. Each entry is separated by the pattern DB_ENTRY_SEPERATOR `/!>(=]]4>gNdEhXm)he7`. Neither DB_KEY_VAL_SEPERATOR DB_ENTRY_SEPERATOR can be included within an entry.
