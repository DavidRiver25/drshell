use super::commands;
use super::operators;

pub enum ArgsParseSuccess {
    SubCmdExeOver,
    NoArgs,
}

pub enum ArgsParseFail {}

pub fn parse_args(args: impl Iterator<Item = String>) -> Result<ArgsParseSuccess, ArgsParseFail> {
    let args: Vec<String> = args.into_iter().collect();
    if args.len() <= 1 {
        return Ok(ArgsParseSuccess::NoArgs);
    }
    match args.get(1).unwrap().as_str() {
        "-c" => {
            let mut cmd = vec![];
            for (i, arg) in args.iter().enumerate() {
                if i > 1 {
                    cmd.push(arg.to_string());
                }
            }
            match commands::parse_cmd(cmd) {
                Ok(cmd) => commands::eval(cmd),
                Err(reason) => commands::parse_cmd_fail_process(reason),
            }
        }
        "-o" => {
            let mut opt = vec![];
            for (i, arg) in args.iter().enumerate() {
                if i > 1 {
                    opt.push(arg.to_string());
                }
            }
            match operators::parse_opt(opt) {
                Ok(opt) => operators::operator_action(opt),
                Err(reason) => operators::parse_opt_fail_process(reason),
            }
        }
        &_ => {}
    }
    Ok(ArgsParseSuccess::SubCmdExeOver)
}
