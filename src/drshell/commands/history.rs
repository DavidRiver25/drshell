use lazy_static::lazy_static;
use rustyline::history;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::{
    io::{self, Write},
    sync::Mutex,
};

lazy_static! {
    static ref HISTORY_CMDS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

struct Status {
    pos_cursor: usize,
    in_history_mode: bool,
    last_direction: Direction,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
}

lazy_static! {
    static ref HISTORY_STATUS: Mutex<Status> = Mutex::new(Status {
        pos_cursor: 0,
        in_history_mode: false,
        last_direction: Direction::Up,
    });
}

pub fn history(limit: Option<usize>) {
    let history = HISTORY_CMDS.lock().unwrap();

    let len = history.len();
    let mut n = len;
    if let Some(limit) = limit {
        n = limit.min(n);
    }

    for (i, h) in history.iter().enumerate() {
        if i >= len - n {
            println!("{} {}", i + 1, h);
        }
    }
}

pub fn save_history(h: &str) {
    let mut history = HISTORY_CMDS.lock().unwrap();
    let mut status = HISTORY_STATUS.lock().unwrap();

    history.push(h.to_string());

    *status = Status {
        pos_cursor: { history.len() - 1 },
        in_history_mode: false,
        last_direction: Direction::Up,
    };
}

pub fn display_previous_cmd() {
    let mut history = HISTORY_CMDS.lock().unwrap();
    let mut status = HISTORY_STATUS.lock().unwrap();

    if history.is_empty() {
        return;
    }

    if (*status).in_history_mode == true && (*status).last_direction == Direction::Down {
        let mov = if (*status).pos_cursor == history.len() - 1 {
            1
        } else {
            2
        };
        (*status).pos_cursor = (*status).pos_cursor.saturating_sub(mov);
    }

    let c = history.get((*status).pos_cursor).unwrap();
    if (*status).in_history_mode == false {
        print!("{c}");
    } else {
        print!("\x1b[2K\r$ {c}");
    }
    io::stdout().flush();

    if (*status).in_history_mode == false {
        (*status).in_history_mode = true;
    }
    if (*status).pos_cursor > 0 {
        (*status).pos_cursor -= 1;
    }
    (*status).last_direction = Direction::Up;
}

pub fn display_next_cmd() {
    let mut history = HISTORY_CMDS.lock().unwrap();
    let mut status = HISTORY_STATUS.lock().unwrap();

    if history.is_empty() {
        return;
    }

    if (*status).in_history_mode == true && (*status).last_direction == Direction::Up {
        let r#move = if (*status).pos_cursor == 0 { 1 } else { 2 };
        (*status).pos_cursor = ((*status).pos_cursor + r#move).min(history.len() - 1);
    }

    let c = history.get((*status).pos_cursor).unwrap();
    if (*status).in_history_mode == false {
        print!("{c}");
    } else {
        print!("\x1b[2K\r$ {c}");
    }
    io::stdout().flush();

    if (*status).in_history_mode == false {
        (*status).in_history_mode = true;
    }
    if (*status).pos_cursor < history.len() - 1 {
        (*status).pos_cursor += 1;
    }
    (*status).last_direction = Direction::Down;
}

pub fn input_history() -> Option<String> {
    let history = HISTORY_CMDS.lock().unwrap();
    let mut status = HISTORY_STATUS.lock().unwrap();

    if (*status).in_history_mode {
        Some(history.get((*status).pos_cursor).unwrap().to_string())
    } else {
        None
    }
}

pub fn read_from_file(file: String) {
    let mut history = HISTORY_CMDS.lock().unwrap();
    let mut buffer = String::new();

    let path = Path::new(&file);

    if path.exists() {
        fs::OpenOptions::new()
            .read(true)
            .open(path)
            .expect("can't open the file!!!")
            .read_to_string(&mut buffer)
            .expect("can't read from the file!!!");
    }

    for line in buffer.lines() {
        history.push(line.to_string());
    }
}

pub fn write_to_file(file: String) {
    let history = HISTORY_CMDS.lock().unwrap();
    let mut buffer = String::new();

    for h in history.iter() {
        buffer += h;
        buffer += "\n";
    }

    let path = Path::new(&file);
    if !path.exists() {
        if let Err(err) = fs::write(path, "") {
            eprintln!("{}", err);
        }
    }

    fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("can't open the file!!!")
        .write_all(buffer.as_bytes())
        .expect("can't write to the file!!!");
}

pub fn append_to_file(file: String) {
    let history = HISTORY_CMDS.lock().unwrap();

    let mut cmds = vec![];
    for (i, h) in history.iter().rev().enumerate() {
        if i != 0 && h.starts_with("history -a ") && h.split_whitespace().into_iter().count() > 2 {
            break;
        }
        cmds.push(h.to_string());
    }

    let mut buffer = String::new();
    for c in cmds.iter().rev() {
        buffer += c;
        buffer += "\n";
    }

    let path = Path::new(&file);
    if !path.exists() {
        if let Err(err) = fs::write(path, "") {
            eprintln!("{}", err);
        }
    }

    fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("can't open the file!!!")
        .write_all(buffer.as_bytes())
        .expect("can't write to the file!!!");
}
