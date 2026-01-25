use super::operators::Opt;

pub struct CmdsAndOperators {
    pub cmds: Vec<Vec<String>>,
    pub operators: Vec<Opt>,
}

pub enum InputSplitFail {
    NoInput,
    NoRedirectArg,
    NoAppendArg,
    NoPipCmd,
    ShlexError,
}

struct CmdsNoSplitAndOperators {
    cmds: Vec<String>,
    operators: Vec<Opt>,
}

fn split_operators(input: String) -> Result<CmdsNoSplitAndOperators, InputSplitFail> {
    let input = input.trim_end();
    let input = shlex::split(input).ok_or(InputSplitFail::ShlexError)?;

    let last = input.last();
    match last {
        Some(last) => {
            let last = last.as_str();
            if last == ">" || last == "1>" || last == "2>" {
                return Err(InputSplitFail::NoRedirectArg);
            } else if last == ">>" || last == "1>>" || last == "2>>" {
                return Err(InputSplitFail::NoAppendArg);
            }
        }
        None => return Err(InputSplitFail::NoInput),
    }

    let mut input = input.iter();
    let mut next;
    let mut operators: Vec<Opt> = Vec::new();
    let mut cmds: Vec<String> = Vec::new();

    loop {
        next = input.next();
        match next {
            Some(string) => {
                let str = string.as_str();
                if str == ">" || str == "1>" {
                    operators.push(Opt::RedirectStdout(
                        input.next().expect("never").to_string(),
                    ));
                } else if str == "2>" {
                    operators.push(Opt::RedirectStderr(
                        input.next().expect("never").to_string(),
                    ));
                } else if str == ">>" || str == "1>>" {
                    operators.push(Opt::AppendStdout(input.next().expect("never").to_string()));
                } else if str == "2>>" {
                    operators.push(Opt::AppendStderr(input.next().expect("never").to_string()));
                } else {
                    cmds.push(string.to_string());
                }
            }
            None => break,
        }
    }
    Ok(CmdsNoSplitAndOperators { cmds, operators })
}

fn split_cmds(cmds: Vec<String>) -> Result<Vec<Vec<String>>, InputSplitFail> {
    if cmds.is_empty() {
        return Err(InputSplitFail::NoInput);
    }

    let mut index: Vec<usize> = cmds
        .iter()
        .enumerate()
        .filter(|(_index, s)| s.as_str() == "|")
        .map(|(index, _)| index)
        .collect();

    if index.is_empty() {
        return Ok(vec![cmds]);
    } else {
        let first = *index.first().expect("never");
        let last = *index.last().expect("never");
        if first == 0_usize || last == cmds.len() - 1 {
            return Err(InputSplitFail::NoPipCmd);
        }
        let mut previous = first;
        index.remove(0);
        for i in index {
            if i - previous == 1 {
                return Err(InputSplitFail::NoPipCmd);
            }
            previous = i;
        }
    }

    let mut cmds = cmds.iter();
    let mut next;
    let mut cmd: Vec<String> = Vec::new();
    let mut cmds_split: Vec<Vec<String>> = Vec::new();

    loop {
        next = cmds.next();
        match next {
            Some(string) => {
                let str = string.as_str();
                if str == "|" && !cmd.is_empty() {
                    cmds_split.push(cmd.clone());
                    cmd.clear();
                } else {
                    cmd.push(string.to_string());
                }
            }
            None => break,
        }
    }

    if !cmd.is_empty() {
        cmds_split.push(cmd.clone());
    }

    Ok(cmds_split)
}

pub fn split_input(input: String) -> Result<CmdsAndOperators, InputSplitFail> {
    let cmds_no_split_and_operators = split_operators(input)?;
    let cmds_split = split_cmds(cmds_no_split_and_operators.cmds)?;

    Ok(CmdsAndOperators {
        cmds: cmds_split,
        operators: cmds_no_split_and_operators.operators,
    })
}

pub fn split_fail_process(reason: InputSplitFail) {
    match reason {
        InputSplitFail::NoInput => {}
        InputSplitFail::NoRedirectArg => {
            eprintln!("no file to redirect!!!");
        }
        InputSplitFail::NoAppendArg => {
            eprintln!("no file to append!!!");
        }
        InputSplitFail::NoPipCmd => {
            eprintln!("no command for the pips!!!")
        }
        InputSplitFail::ShlexError => {
            eprintln!("shlex error!!!")
        }
    }
}
