# readchar
[![docs.rs](https://img.shields.io/docsrs/readchar)](https://docs.rs/readchar/latest/readchar/)

a concurrently outputting readline implementation

## Example

see `./examples/generic.rs` for example usage

`cargo run --example generic.rs -- executable args`

executes executable with args, and then the input line from readchar is the final argument, so

`cargo run --example generic.rs -- calc`

will run `calc` on each input line while you type

## Features

- local/file history
- outputting while you type

## Actions

| Keystroke             | Action                                                                      |
| --------------------- | --------------------------------------------------------------------------- |
| Left                  | Move cursor one character left                                              |
| Right                 | Move cursor one character right                                             |
| Ctrl-Up               | Move cursor up one line                                                     |
| Ctrl-Down             | Move cursor down one line                                                   |
| Up                    | Move up history by 1 entry                                                  |
| Down                  | Move down history by 1 entry                                                |
| Backspace             | Removes the character left of the cursor, the moves left one character      |
| Delete                | Removes the character on the cursor                                         |
| Enter                 | goes to next line and resets input                                          |

## Special keywords

runs these on enter

I might remove these or make these optional

- `clear` clears the terminal
- `exit` exits the program
