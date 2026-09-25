use std::collections::VecDeque;
use std::io::{self, Read, Write};
use std::process::ExitStatus;

#[derive(Debug, Clone, Default)]
pub struct TerminalBuffer {
    pub lines: VecDeque<String>,
    pub scrollback: usize,
}

#[derive(Debug, Clone, Default)]
pub struct TerminalState {
    pub rows: u32,
    pub cols: u32,
    pub cursor_x: u32,
    pub cursor_y: u32,
    pub active: bool,
}

#[derive(Debug)]
pub struct Terminal {
    pub buffer: TerminalBuffer,
    pub state: TerminalState,
    pub history: Vec<String>,
    pub input: String,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            buffer: TerminalBuffer {
                lines: VecDeque::new(),
                scrollback: 500,
            },
            state: TerminalState {
                rows: 24,
                cols: 80,
                cursor_x: 0,
                cursor_y: 0,
                active: true,
            },
            history: Vec::new(),
            input: String::new(),
        }
    }
}

impl Terminal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_size(&mut self, rows: u32, cols: u32) {
        self.state.rows = rows;
        self.state.cols = cols;
    }

    pub fn push_line(&mut self, line: impl Into<String>) {
        let line = line.into();
        self.buffer.lines.push_back(line.clone());
        if self.buffer.lines.len() > self.buffer.scrollback {
            self.buffer.lines.pop_front();
        }
        self.history.push(line);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        let text = std::str::from_utf8(bytes).unwrap_or("<invalid utf8>");
        self.write_text(text)
    }

    pub fn write_text(&mut self, text: &str) -> io::Result<()> {
        for ch in text.chars() {
            match ch {
                '\n' => {
                    self.push_line(self.input.clone());
                    self.input.clear();
                    self.state.cursor_y += 1;
                    self.state.cursor_x = 0;
                }
                '\r' => {
                    self.state.cursor_x = 0;
                }
                '\t' => {
                    self.input.push(' ');
                    self.input.push(' ');
                    self.state.cursor_x += 2;
                }
                _ => {
                    self.input.push(ch);
                    self.state.cursor_x += 1;
                }
            }
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.buffer.lines.clear();
        self.history.clear();
        self.input.clear();
        self.state.cursor_x = 0;
        self.state.cursor_y = 0;
    }

    pub fn input(&mut self, text: &str) {
        self.input.push_str(text);
    }

    pub fn read_line(&mut self) -> String {
        let line = self.input.clone();
        self.input.clear();
        line
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        for line in &self.buffer.lines {
            output.push_str(line);
            output.push('\n');
        }
        output
    }
}

pub fn process_terminal_output(term: &mut Terminal, bytes: &[u8]) -> io::Result<()> {
    term.write_bytes(bytes)
}

pub fn terminal_exit_status() -> ExitStatus {
    std::process::Command::new("/bin/true").status().unwrap_or_else(|_| {
        std::process::Command::new("cmd").status().unwrap_or_else(|_| std::process::ExitStatus::from_raw(0))
    })
}
