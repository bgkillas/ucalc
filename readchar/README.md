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

| Keystroke  | Action                                                                 |
|------------|------------------------------------------------------------------------|
| Left       | Move cursor one character left                                         |
| Right      | Move cursor one character right                                        |
| Ctrl-Left  | Move cursor one word left                                              |
| Ctrl-Right | Move cursor one word right                                             |
| Up         | Move up history by 1 entry                                             |
| Down       | Move down history by 1 entry                                           |
| Ctrl-Up    | Move cursor up one line                                                |
| Ctrl-Down  | Move cursor down one line                                              |
| Home       | Moves to start of line                                                 |
| End        | Moves to end of line                                                   |
| Backspace  | Removes the character left of the cursor, the moves left one character |
| Delete     | Removes the character on the cursor                                    |
| Ctrl-C     | Exits buffer                                                           |
| Enter      | Finishes line                                                          |