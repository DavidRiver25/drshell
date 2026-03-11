use super::super::commands;
use super::super::env as drshell_env;
use super::Rustyline;
use rustyline::Context;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use std::env;
use std::path::Path;

fn sort_cmds() -> Vec<String> {
    let mut cmds: Vec<String> = vec![];

    for exe in drshell_env::find_exes() {
        cmds.push(exe.get_name());
    }

    cmds.sort();
    let mut unrepeat = vec![];
    for cmd in &cmds {
        if let Some(last) = unrepeat.last()
            && cmd == last
        {
            continue;
        }
        unrepeat.push(cmd.to_string());
    }

    let mut index = vec![];
    for (i, u) in unrepeat.iter().enumerate() {
        for c in commands::BUILTIN_CMDS {
            if c == u {
                index.push(i);
            }
        }
    }
    for i in index.iter().rev() {
        unrepeat.remove(*i);
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

fn complete_files(line: &str, mut pos: usize) -> Vec<Pair> {
    let mut result: Vec<Pair> = vec![];
    let mut prefix = &line[pos..];

    let mut path = "";
    if prefix.contains("/") {
        let pos_dir = line.rfind("/").expect("never");
        path = &line[pos..=pos_dir];
        prefix = &line[pos_dir + 1..];
        pos = pos_dir + 1;
    }

    let current_dir;
    let mut count = 0;
    if path.is_empty() {
        current_dir = loop {
            if let Ok(dir) = env::current_dir()
                && let Ok(dir) = dir.into_os_string().into_string()
            {
                break dir;
            }
            count += 1;
            if count > 10 {
                return result;
            }
        };
        path = &current_dir;
    }

    let mut files = drshell_env::find_files_from_dir(path);
    files = files
        .iter()
        .filter(|s| s.starts_with(prefix))
        .map(|s| s.to_string())
        .collect();
    files.sort();

    for file in &files {
        result.push(Pair {
            display: file.to_string(),
            replacement: line[..pos].to_string() + file,
        });
    }

    if result.len() == 1 {
        let p = path.to_string() + "/" + &result[0].display;
        let p = Path::new(&p);
        if p.is_file() {
            result[0].replacement += " ";
        } else if let Some(p) = p.to_str()
            && drshell_env::find_files_from_dir(p).is_empty()
        {
            if result[0].replacement.ends_with("/") {
                result[0].replacement.pop();
            }
            result[0].replacement += " ";
        }
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
            /* if there are some pipelines, complete the last pipeline's input */
        } else if let Some(pos) = line.rfind(" | ") {
            let content = line[(pos + 3)..].to_string();
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
