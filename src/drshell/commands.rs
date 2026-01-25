mod cd;
mod echo;
mod exit;
mod help;
mod history;
mod lsbuiltin;
mod pwd;
mod r#type;

use super::env as drshell_env;
use std::process::Command;

pub const BUILTIN_CMDS: [&str; 7] = ["echo", "exit", "type", "pwd", "cd", "lsbuiltin", "history"];

#[allow(dead_code)]
const BUILTIN_ARGS: [&str; 1] = ["-h"];

#[derive(Debug)]
pub enum Cmd {
    Echo(Vec<String>),
    Exit(i32),
    Type(String),
    Pwd,
    Cd(String),
    BuiltinLs,
    BuiltinHelp(&'static str),
    History(HistoryArgs),
    NotBuiltin(Vec<String>),
}

#[derive(Debug)]
pub enum HistoryArgs {
    Show(Option<usize>),
    ReadFromFile(String),
    WriteToFile(String),
}

pub enum CmdParseFail {
    NoCommand,
    NotCommand(String),
    ExitArgsError,
    TypeArgsError,
    CdArgsError,
    HistoryArgsError,
    Never,
}

pub enum Api<'a> {
    SaveHistory(&'a str),
    DisplayPreviousCmd,
    DisplayNextCmd,
    InputHistory,
    ReadHistoryFromFile(&'a str),
    WriteHistoryToFile(&'a str),
}

pub fn parse_cmd(mut cmd: Vec<String>) -> Result<Cmd, CmdParseFail> {
    if cmd.is_empty() {
        return Err(CmdParseFail::NoCommand);
    } else if cmd.len() >= 2 {
        for builtin in BUILTIN_CMDS.iter() {
            if &cmd[0] == builtin && cmd[1] == "-h" {
                return Ok(Cmd::BuiltinHelp(builtin));
            }
        }
    }
    match cmd[0].as_str() {
        "echo" => {
            cmd.remove(0);
            Ok(Cmd::Echo(cmd))
        }
        "exit" => {
            if cmd.len() == 1 {
                Ok(Cmd::Exit(0))
            } else {
                match cmd[1].parse::<i32>() {
                    Ok(num) => Ok(Cmd::Exit(num)),
                    Err(_) => Err(CmdParseFail::ExitArgsError),
                }
            }
        }
        "type" => {
            if cmd.len() == 1 {
                Err(CmdParseFail::TypeArgsError)
            } else {
                Ok(Cmd::Type(cmd[1].clone()))
            }
        }
        "pwd" => Ok(Cmd::Pwd),
        "cd" => {
            if cmd.len() == 1 {
                Err(CmdParseFail::CdArgsError)
            } else {
                Ok(Cmd::Cd(cmd[1].clone()))
            }
        }
        "lsbuiltin" => Ok(Cmd::BuiltinLs),
        "history" => {
            let len = cmd.len();
            if len > 1 {
                let arg_1 = cmd[1].as_str();
                if arg_1 == "-r" || arg_1 == "-w" {
                    if len < 3 {
                        Err(CmdParseFail::HistoryArgsError)
                    } else {
                        match arg_1 {
                            "-r" => Ok(Cmd::History(HistoryArgs::ReadFromFile(cmd[2].to_string()))),
                            "-w" => Ok(Cmd::History(HistoryArgs::WriteToFile(cmd[2].to_string()))),
                            &_ => Err(CmdParseFail::Never),
                        }
                    }
                } else {
                    match cmd[1].parse::<usize>() {
                        Ok(num) => Ok(Cmd::History(HistoryArgs::Show(Some(num)))),
                        Err(_) => Err(CmdParseFail::HistoryArgsError),
                    }
                }
            } else {
                Ok(Cmd::History(HistoryArgs::Show(None)))
            }
        }
        _ => {
            if let Some(_path) = drshell_env::if_executable(&cmd[0]) {
                Ok(Cmd::NotBuiltin(cmd))
            } else {
                Err(CmdParseFail::NotCommand(cmd[0].clone()))
            }
        }
    }
}

pub fn parse_cmd_fail_process(reason: CmdParseFail) {
    match reason {
        CmdParseFail::NoCommand => {}
        CmdParseFail::ExitArgsError => {
            eprintln!("no number of exiting!!!");
        }
        CmdParseFail::TypeArgsError => {
            eprintln!("no name of a command!!!");
        }
        CmdParseFail::CdArgsError => {
            eprintln!("no name of a directory!!!");
        }
        CmdParseFail::NotCommand(cmd) => {
            eprintln!("{}", cmd + ": command not found");
        }
        CmdParseFail::HistoryArgsError => {
            eprintln!("wrong args for history!!!");
        }
        CmdParseFail::Never => {}
    }
}

pub fn eval(cmd: Cmd) {
    match cmd {
        Cmd::Echo(str) => echo::echo(str),
        Cmd::Exit(num) => exit::exit(num),
        Cmd::Type(name) => r#type::r#type(name),
        Cmd::Pwd => pwd::pwd(),
        Cmd::Cd(dir) => cd::cd(dir),
        Cmd::BuiltinLs => lsbuiltin::lsbuiltin(),
        Cmd::BuiltinHelp(cmd) => help::builtinhelp(cmd),
        Cmd::History(args) => match args {
            HistoryArgs::Show(limit) => history::history(limit),
            HistoryArgs::ReadFromFile(path) => history::read_from_file(path),
            HistoryArgs::WriteToFile(path) => history::write_to_file(path),
        },
        Cmd::NotBuiltin(cmd) => {
            let mut args = cmd.clone();
            args.remove(0);
            match Command::new(&cmd[0]).args(args).spawn() {
                Ok(mut child) => {
                    if let Err(err) = child.wait() {
                        eprintln!("{}", err);
                    }
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
    }
}

pub fn generate_cmd(cmd: Cmd) -> Command {
    let mut command_generate: Command;
    let command: String;
    let mut args = vec![];

    if let Cmd::NotBuiltin(mut cmd) = cmd {
        command = cmd.remove(0);
        for c in cmd {
            args.push(c);
        }
    } else {
        command = "drshell".to_string();
        args.push("-c".to_string());

        match cmd {
            Cmd::Echo(str) => {
                args.push("echo".to_string());
                for s in str {
                    args.push(s);
                }
            }
            Cmd::Exit(num) => {
                args.push("exit".to_string());
                args.push(num.to_string());
            }
            Cmd::Type(name) => {
                args.push("type".to_string());
                args.push(name);
            }
            Cmd::Pwd => {
                args.push("pwd".to_string());
            }
            Cmd::Cd(dir) => {
                args.push("cd".to_string());
                args.push(dir);
            }
            Cmd::BuiltinLs => {
                args.push("lsbuiltin".to_string());
            }
            Cmd::BuiltinHelp(cmd) => {
                args.push(cmd.to_string());
                args.push("-h".to_string());
            }
            Cmd::History(args_history) => {
                args.push("history".to_string());
                match args_history {
                    HistoryArgs::Show(limit) => {
                        if let Some(l) = limit {
                            args.push(l.to_string());
                        }
                    }
                    HistoryArgs::ReadFromFile(path) => {
                        args.push("-r".to_string());
                        args.push(path);
                    }
                    HistoryArgs::WriteToFile(path) => {
                        args.push("-w".to_string());
                        args.push(path);
                    }
                }
            }
            _ => {}
        }
    }
    command_generate = std::process::Command::new(command);
    command_generate.args(args);

    command_generate
}

pub fn api(api: Api) -> Option<String> {
    match api {
        Api::SaveHistory(s) => history::save_history(s),
        Api::DisplayPreviousCmd => history::display_previous_cmd(),
        Api::DisplayNextCmd => history::display_next_cmd(),
        Api::InputHistory => {
            return history::input_history();
        }
        Api::ReadHistoryFromFile(file) => history::read_from_file(file.to_string()),
        Api::WriteHistoryToFile(file) => history::write_to_file(file.to_string()),
    }
    None
}
