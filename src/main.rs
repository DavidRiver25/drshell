mod drshell;

use crate::drshell::args::{self, ArgsParseSuccess};
use crate::drshell::commands::{self, Api};
use crate::drshell::input::{self, CmdsAndOperators};
use crate::drshell::operators;
use crate::drshell::pipline::{self, Pipeline};
use crate::drshell::rustyline::Rustyline;
use rustyline::error::ReadlineError;
use rustyline::{Config, Editor, Event, EventHandler, KeyCode, KeyEvent, Modifiers};
use std::env;

fn main() -> rustyline::Result<()> {
    /* read history */
    if let Ok(home) = env::var("HOME") {
        commands::api(Api::ReadHistoryFromFile((home + "/.drhistory").as_str()));
    }

    /* args */
    let args = env::args();
    if let Ok(ArgsParseSuccess::SubCmdExeOver) = args::parse_args(args) {
        return Ok(());
    }

    println!(">::< welcome to drshell");

    /* rustyline */
    let config = Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .completion_show_all_if_ambiguous(false)
        .auto_add_history(false)
        .build();
    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(Rustyline::new()));
    rl.bind_sequence(
        Event::KeySeq(vec![KeyEvent(KeyCode::Up, Modifiers::NONE)]),
        EventHandler::Conditional(Box::new(Rustyline)),
    );
    rl.bind_sequence(
        Event::KeySeq(vec![KeyEvent(KeyCode::Down, Modifiers::NONE)]),
        EventHandler::Conditional(Box::new(Rustyline)),
    );

    'repl: loop {
        match rl.readline("$ ") {
            Ok(line) => {
                let line = handle_history_with_line(line);
                match input::split_input(line) {
                    Ok(cmds_and_operators) => {
                        let Some((mut cmds, opts)) = parse_cmds_and_opts(cmds_and_operators) else {
                            continue 'repl;
                        };
                        execute_cmds_and_opts(cmds, opts);
                    }
                    Err(reason) => {
                        input::split_fail_process(reason);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    /* write history */
    if let Ok(home) = env::var("HOME") {
        commands::api(Api::WriteHistoryToFile((home + "/.drhistory").as_str()));
    }

    Ok(())
}

fn handle_history_with_line(mut line: String) -> String {
    if let Some(input) = commands::api(commands::Api::InputHistory) {
        line = input;
    }
    if !line.is_empty() {
        let _ = commands::api(commands::Api::SaveHistory(&line));
    }
    line
}

fn parse_cmds_and_opts(
    cmds_and_operators: CmdsAndOperators,
) -> Option<(Vec<commands::Cmd>, Vec<operators::Opt>)> {
    let mut cmds = vec![];
    for cmd in &cmds_and_operators.cmds {
        match commands::parse_cmd(cmd.clone()) {
            Ok(cmd) => cmds.push(cmd),
            Err(reason) => {
                commands::parse_cmd_fail_process(reason);
                return None;
            }
        }
    }
    let opts: Vec<operators::Opt> = cmds_and_operators.operators.into_iter().collect();

    Some((cmds, opts))
}

fn execute_cmds_and_opts(mut cmds: Vec<commands::Cmd>, opts: Vec<operators::Opt>) {
    if cmds.len() == 1 && opts.is_empty() {
        commands::eval(cmds.pop().expect("never"));
        return;
    }

    let mut cmds_generate = pipline::Cmds::new();
    for cmd in cmds {
        let cmd = commands::generate_cmd(cmd);
        cmds_generate.add_cmd(cmd);
    }
    let (redirect_stdout, redirect_stderr) = operators::find_last_opts(&opts);
    if let Some(opt) = redirect_stdout {
        cmds_generate.add_redirect_stdout(operators::generate_opt(opt));
    }
    if let Some(opt) = redirect_stderr {
        cmds_generate.add_redirect_stderr(operators::generate_opt(opt));
    }

    let mut pipeline = Pipeline::new();
    if pipeline.pipe(cmds_generate).is_ok() {
        pipeline.wait();
    } else if let Err(err) = pipeline.kill() {
        eprintln!("{}", err);
    }
}
