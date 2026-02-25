use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
#[derive(Default)]
pub enum History {
    Local(LocalHistory),
    File(LocalHistory, File),
    #[default]
    None,
}
impl History {
    pub fn new(path: Option<&Path>) -> io::Result<Self> {
        if let Some(path) = path {
            let mut file = OpenOptions::new().read(true).append(true).open(path)?;
            let mut history = String::with_capacity(file.metadata()?.len() as usize);
            file.read_to_string(&mut history)?;
            let char_index = history.len();
            let lines = history.lines().count();
            Ok(Self::File(
                LocalHistory {
                    history,
                    history_modified: HashMap::with_capacity(8),
                    char_index,
                    index: lines,
                    lines,
                },
                file,
            ))
        } else {
            Ok(Self::Local(LocalHistory {
                history: String::with_capacity(256),
                history_modified: HashMap::with_capacity(8),
                char_index: 0,
                index: 0,
                lines: 0,
            }))
        }
    }
    pub fn mv(&mut self, cur: &str, up: bool) -> &str {
        match self {
            History::Local(v) | History::File(v, _) => v.mv(cur, up),
            History::None => unreachable!(),
        }
    }
    pub fn at_start(&self) -> bool {
        match self {
            History::Local(h) | History::File(h, _) => h.index == 0,
            History::None => true,
        }
    }
    pub fn at_end(&self) -> bool {
        match self {
            History::Local(h) | History::File(h, _) => h.index == h.lines,
            History::None => true,
        }
    }
    pub fn push(&mut self, s: &str) -> io::Result<()> {
        match self {
            History::Local(h) => h.push(s),
            History::File(h, f) => {
                h.push(s);
                writeln!(f, "{s}")?;
            }
            History::None => {}
        }
        Ok(())
    }
}
pub struct LocalHistory {
    history: String,
    history_modified: HashMap<usize, String>,
    char_index: usize,
    index: usize,
    lines: usize,
}
impl LocalHistory {
    pub fn history_modified(&self, cur: &str) -> bool {
        self.history_modified
            .get(&self.index)
            .map(|s| s != cur)
            .unwrap_or(
                self.index == self.lines
                    || self
                        .history
                        .get(self.char_index + 1..self.char_index + 1 + cur.len())
                        .map(|s| s != cur)
                        .unwrap_or(true)
                    || self
                        .history
                        .as_bytes()
                        .get(self.char_index + 1 + cur.len())
                        .map(|c| *c != b'\n')
                        .unwrap_or(false),
            )
    }
    pub fn mv(&mut self, cur: &str, up: bool) -> &str {
        if up {
            if self.history_modified(cur) {
                self.history_modified.insert(self.index, cur.to_string());
            }
            self.index -= 1;
            let last = self.history[..self.char_index]
                .rfind('\n')
                .map(|i| i + 1)
                .unwrap_or(0);
            let s = &self.history[last..self.char_index];
            self.char_index = last.saturating_sub(1);
            self.history_modified
                .get(&self.index)
                .map(|s| s.as_str())
                .unwrap_or(s)
        } else if self.index + 1 == self.lines {
            self.index += 1;
            self.char_index = self.history.len() - 1;
            &self.history_modified[&self.index]
        } else {
            if self.history_modified(cur) {
                self.history_modified.insert(self.index, cur.to_string());
            }
            self.index += 1;
            let next = self.history[self.char_index + 1..]
                .find('\n')
                .map(|i| self.char_index + 1 + i)
                .unwrap();
            let f = self.history[next + 1..]
                .find('\n')
                .map(|i| next + 1 + i)
                .unwrap_or(self.history.len());
            let s = &self.history[next + 1..f];
            self.char_index = next;
            self.history_modified
                .get(&self.index)
                .map(|s| s.as_str())
                .unwrap_or(s)
        }
    }
    pub fn push(&mut self, s: &str) {
        use std::fmt::Write;
        writeln!(&mut self.history, "{s}").unwrap();
        self.history_modified.clear();
        self.char_index = self.history.len() - 1;
        self.lines += 1;
        self.index = self.lines;
    }
}
