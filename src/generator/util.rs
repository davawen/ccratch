use std::io::{self, Write};
use nanoid::nanoid;

use crate::parser::{self, VarMap};

pub struct IndentWriter<W: Write> {
    pub writer: W,
    indent_level: u32,
    last_write_ended_with_newline: bool
}

impl<W: Write> Write for IndentWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for line in buf.split_inclusive(|&c| c == 0xa) { // split on newlines
            if self.last_write_ended_with_newline {
                for _ in 0..self.indent_level {
                    write!(self.writer, "    ")?;
                }
                self.last_write_ended_with_newline = false;
            }
            self.writer.write_all(line)?;

            if line.last() == Some(&0xa) {
                self.last_write_ended_with_newline = true;
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> IndentWriter<W> {
    pub fn new(writer: W) -> Self {
        IndentWriter { writer, indent_level: 0, last_write_ended_with_newline: true }
    }

    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn deindent(&mut self) {
        self.indent_level -= 1;
    }
}

pub type IW<W> = IndentWriter<W>;

pub fn generate_var_name() -> String {
    pub const CIDENT: [char; 53] = [
        '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
        'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    nanoid!(12, &CIDENT)
}

pub struct GeneratorArgs<'a> {
    pub target: &'a parser::Target,
    pub globals: &'a VarMap,
    pub state: &'a mut u32,
    pub new_locals: &'a mut Vec<String>,
}

/// get a variable in a function
pub fn get_var<'a>(args: &GeneratorArgs, id: &'_ str) -> String {
    if let Some(var) = args.target.vars.get(id) {
        format!("a->var_{var}")
    } else if let Some(var) = args.globals.get(id) {
        format!("g->var_{var}")
    } else {
        unreachable!(
            "variable with ID `{id}` does not exists:\n{:#?}\n{:#?}",
            args.target.vars, args.globals
        );
    }
}

pub fn start_case<W: Write>(f: &mut IW<W>, state: &mut u32) -> io::Result<()> {
    writeln!(f, "case {}: {{", *state)?;
    f.indent();
    Ok(())
}

pub fn end_case<W: Write>(f: &mut IW<W>, state: &mut u32) -> io::Result<()> {
    f.deindent();
    writeln!(f, "}}")?;
    writeln!(f, "break;")?;
    *state += 1;
    Ok(())
}

pub enum Return {
    /// The block returns nothing and ends normally
    Empty,
    /// The block returns a value (an identifier to a C variable)
    Value(String),
    /// The block handles switching the state by itself. 
    /// `"s->state = {*state + 1}"` will not be added automatically.
    Hold,
    /// The block handles ending the case by itself by calling [`end_case`].
    /// `"} break;"` will not be added automatically.
    Ended
}
