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

fn find_last_opt(opts: &[Opt], opt: OptFind) -> Option<&Opt> {
    match opt {
        OptFind::Stdout => opts
            .iter()
            .filter(|opt| matches!(opt, Opt::RedirectStdout(_) | Opt::AppendStdout(_)))
            .next_back(),
        OptFind::Stderr => opts
            .iter()
            .filter(|opt| matches!(opt, Opt::RedirectStderr(_) | Opt::AppendStderr(_)))
            .next_back(),
    }
}

/* first -> stdout, second -> stderr */
pub fn find_last_opts(opts: &[Opt]) -> (Option<&Opt>, Option<&Opt>) {
    (
        find_last_opt(opts, OptFind::Stdout),
        find_last_opt(opts, OptFind::Stderr),
    )
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
    let optname = opt.first().ok_or(OptParseFail::NoOpt)?.as_str();
    let filename = opt.get(1).ok_or(OptParseFail::NofileName)?.to_string();

    match optname {
        "redirect_stdout" => Ok(Opt::RedirectStdout(filename)),
        "redirect_stderr" => Ok(Opt::RedirectStdout(filename)),
        "append_stdout" => Ok(Opt::AppendStdout(filename)),
        "append_stderr" => Ok(Opt::AppendStderr(filename)),
        &_ => Err(OptParseFail::WrongOpt),
    }
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
