use std::io;

pub fn echo(mut str: Vec<String>) {
    if str.is_empty() {
        let mut line = String::new();
        let stdin = io::stdin();
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
            print!("{line}");
            line.clear();
        }
    } else {
        let last = str.pop();
        for s in str {
            print!("{s} ");
        }
        if let Some(last) = last {
            println!("{last}");
        }
    }
}
