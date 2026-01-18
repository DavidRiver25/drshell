use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[allow(dead_code)]
const OPERATORS: [&str; 6] = [">", "1>", "2>", ">>", "1>>", "2>>"];

#[derive(Debug)]
pub enum Opt {
    RedirectStdout(String),
    RedirectStderr(String),
    AppendStdout(String),
    AppendStderr(String),
}

enum OptFind {
    Stdout,
    Stderr,
}

pub enum OptParseFail {
    NoOpt,
    NofileName,
    WrongOpt,
}

fn find_last_opt(opts: &Vec<Opt>, opt: OptFind) -> Option<&Opt> {
    match opt {
        OptFind::Stdout => opts
            .iter()
            .filter(|opt| match opt {
                Opt::RedirectStdout(_) | Opt::AppendStdout(_) => true,
                _ => false,
            })
            .last(),
        OptFind::Stderr => opts
            .iter()
            .filter(|opt| match opt {
                Opt::RedirectStderr(_) | Opt::AppendStderr(_) => true,
                _ => false,
            })
            .last(),
    }
}

/* first -> stdout, second -> stderr */
pub fn find_last_opts(opts: &Vec<Opt>) -> (Option<&Opt>, Option<&Opt>) {
    let mut ret = (None, None);

    if let Some(opt) = find_last_opt(opts, OptFind::Stdout) {
        ret.0 = Some(opt);
    }
    if let Some(opt) = find_last_opt(opts, OptFind::Stderr) {
        ret.1 = Some(opt);
    }

    ret
}

pub fn generate_opt(opt: &Opt) -> Command {
    let mut command_generate: Command;
    let command = "drshell".to_string();
    let mut args = vec![];

    args.push("-o".to_string());
    let file = match opt {
        Opt::RedirectStdout(file) => {
            args.push("redirect_stdout".to_string());
            file
        }
        Opt::RedirectStderr(file) => {
            args.push("redirect_stderr".to_string());
            file
        }
        Opt::AppendStdout(file) => {
            args.push("append_stdout".to_string());
            file
        }
        Opt::AppendStderr(file) => {
            args.push("append_stderr".to_string());
            file
        }
    };
    args.push(file.to_string());

    command_generate = std::process::Command::new(command);
    command_generate.args(args);

    command_generate
}

pub fn parse_opt(opt: Vec<String>) -> Result<Opt, OptParseFail> {
    if opt.is_empty() {
        return Err(OptParseFail::NoOpt);
    }
    let optname = opt.get(0).expect("never");
    let mut filename = String::new();
    if let Some(name) = opt.get(1) {
        filename = name.to_string();
    }
    let mut ret = match optname.as_str() {
        "redirect_stdout" => Ok(Opt::RedirectStdout(filename.clone())),
        "redirect_stderr" => Ok(Opt::RedirectStdout(filename.clone())),
        "append_stdout" => Ok(Opt::AppendStdout(filename.clone())),
        "append_stderr" => Ok(Opt::AppendStderr(filename.clone())),
        &_ => return Err(OptParseFail::WrongOpt),
    };
    if filename.is_empty() {
        ret = Err(OptParseFail::NofileName);
    }

    ret
}

pub fn operator_action(opt: Opt) {
    let flag_redirect;
    let file = match opt {
        Opt::RedirectStdout(file) => {
            flag_redirect = true;
            file
        }
        Opt::RedirectStderr(file) => {
            flag_redirect = true;
            file
        }
        Opt::AppendStdout(file) => {
            flag_redirect = false;
            file
        }
        Opt::AppendStderr(file) => {
            flag_redirect = false;
            file
        }
    };
    let path = Path::new(&file);
    let mut line = String::new();
    let stdin = io::stdin();

    if flag_redirect || !path.exists() {
        if let Err(err) = fs::write(path, "") {
            eprintln!("{}", err);
        }
    }
    loop {
        match stdin.read_line(&mut line) {
            Ok(num) => {
                if num == 0 {
                    return;
                }
            }
            Err(_) => {
                eprintln!("read stdin error!!!");
                return;
            }
        }

        fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .expect("can't open the file!!!")
            .write_all(line.as_bytes())
            .expect("can't write to the file!!!");
        line.clear();
    }
}

pub fn parse_opt_fail_process(reason: OptParseFail) {
    match reason {
        OptParseFail::NoOpt => {
            eprintln!("no opt!!!");
        }
        OptParseFail::NofileName => {
            eprintln!("no filename for opt!!!");
        }
        OptParseFail::WrongOpt => {
            eprintln!("wrong opt!!!");
        }
    }
}
