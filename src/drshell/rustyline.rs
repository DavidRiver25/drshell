mod cmp;
mod event;

use rustyline::{highlight::Highlighter, hint::Hinter, validate::Validator, Editor, Helper};

#[derive(Debug, Default)]
pub struct Rustyline;

impl Rustyline {
    pub fn new() -> Self {
        Rustyline
    }
}

impl Helper for Rustyline {}
impl Hinter for Rustyline {
    type Hint = String;
}
impl Highlighter for Rustyline {}
impl Validator for Rustyline {}
