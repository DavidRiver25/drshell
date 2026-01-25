use super::super::commands;
use super::super::env as drshell_env;
use super::Rustyline;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::Context;

fn sort_cmds() -> Vec<String> {
    let mut cmds: Vec<String> = vec![];

    for exe in drshell_env::find_exes() {
        cmds.push(exe.get_name());
    }

    cmds.sort();
    let mut unrepeat = vec![];
    for cmd in &cmds {
        if let Some(last) = unrepeat.last() {
            if cmd == last {
                continue;
            }
        }
        unrepeat.push(cmd.to_string());
    }

    for cmd in commands::BUILTIN_CMDS {
        unrepeat.insert(0, cmd.into());
    }

    unrepeat
}

fn complete_cmds(line: &str, pos: usize) -> Vec<Pair> {
    let mut result: Vec<Pair> = vec![];
    let prefix = &line[pos..];

    if !prefix.is_empty() {
        let cmds: Vec<String> = sort_cmds()
            .iter()
            .filter(|s| s.starts_with(prefix))
            .map(|s| s.to_string())
            .collect();
        for cmd in &cmds {
            result.push(Pair {
                display: cmd.to_string(),
                replacement: line[..pos].to_string() + cmd,
            });
        }
        if result.len() == 1 {
            result[0].replacement += " ";
        }
    }

    result
}

fn complete_files(line: &str, pos: usize) -> Vec<Pair> {
    let mut result: Vec<Pair> = vec![];
    let prefix = &line[pos..];

    let mut files = drshell_env::find_current_dir_files();

    if !prefix.is_empty() {
        files = files
            .iter()
            .filter(|s| s.starts_with(prefix))
            .map(|s| s.to_string())
            .collect();
    }
    for file in &files {
        result.push(Pair {
            display: file.to_string(),
            replacement: line[..pos].to_string() + file,
        });
    }
    if result.len() == 1 {
        result[0].replacement += " ";
    }

    result
}

impl Completer for Rustyline {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let mut result = vec![];
        let mut start = 0;

        /* trim white space at start */
        if let Some(i) = line.find(|c| c != ' ') {
            start = i;
        }

        if !line[start..pos].contains(' ') {
            result = complete_cmds(line, start);
        } else if let Some(pos) = line.rfind(" | ") {
            let content = line[(pos + 3)..].to_string();
            /* trim white space at start */
            if let Some(index) = content.find(|c| c != ' ') {
                if !line[pos + 3 + index..].contains(' ') {
                    result = complete_cmds(line, pos + 3 + index)
                } else if let Some(pos) = line.rfind(" ") {
                    result = complete_files(line, pos + 1);
                }
            }
        } else if let Some(pos) = line.rfind(" ") {
            result = complete_files(line, pos + 1);
        };

        Ok((0, result))
    }
}
