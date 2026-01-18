pub fn builtinhelp(cmd: &str) {
    match cmd {
        "echo" => {
            println!(
                "Description:
echo the words
Usage:
echo <STRING>"
            );
        }
        "exit" => {
            println!(
                "Description:
terminate the shell progress
Usage:
exit <NUMBER>
Details:
the number can be 0 or 1"
            );
        }
        "type" => {
            println!(
                "Description:
show the command if builtin or other
Usage:
type <COMMAND>"
            );
        }
        "pwd" => {
            println!(
                "Description:
print the current working directory
Usage:
pwd"
            );
        }
        "cd" => {
            println!(
                "Description:
change the current working directory
Usage:
cd <DIRECTORY>
Details:
1. - the last work directory
2. ~ the home directory"
            );
        }
        "lsbuiltin" => {
            println!(
                "Description:
list all the builtin commands
Usage:
lsbuiltin"
            );
        }
        "history" => {
            println!(
                "Description:
1. list the previously executed commands
2. write the history to a file
3. read history from a file
4. append the history to a file
Usage:
history <LIMIT> | <-w FILE> | <-r FILE> | <-a FILE>"
            );
        }
        &_ => {}
    }
}
