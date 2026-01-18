use std::process::{Command, Stdio};

pub struct Cmds {
    cmds: Vec<Command>,
    redirect_stdout: Option<Command>,
    redirect_stderr: Option<Command>,
}

impl Cmds {
    pub fn new() -> Self {
        Self {
            cmds: Vec::new(),
            redirect_stdout: None,
            redirect_stderr: None,
        }
    }

    pub fn add_cmd(&mut self, cmd: Command) {
        self.cmds.push(cmd);
    }

    pub fn add_redirect_stdout(&mut self, cmd: Command) {
        self.redirect_stdout = Some(cmd);
    }

    pub fn add_redirect_stderr(&mut self, cmd: Command) {
        self.redirect_stderr = Some(cmd);
    }

    fn take_redirect_stdout(&mut self) -> Option<Command> {
        self.redirect_stdout.take()
    }

    fn take_redirect_stderr(&mut self) -> Option<Command> {
        self.redirect_stderr.take()
    }

    pub fn is_empty(&self) -> bool {
        self.cmds.is_empty()
    }
}

pub struct Pipeline {
    childs: Vec<std::process::Child>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { childs: Vec::new() }
    }

    pub fn pipe(&mut self, mut cmds: Cmds) -> std::io::Result<()> {
        if cmds.is_empty() {
            return Ok(());
        }

        /* check the stdout and stderr opts */
        let opt_stdout = cmds.take_redirect_stdout();
        let opt_stderr = cmds.take_redirect_stderr();

        /* create the cmds' threads except the last one */
        let mut previous_stdout = None;
        let mut last_cmd = cmds.cmds.pop().expect("cmds is not empty");

        for mut cmd in cmds.cmds {
            if let Some(stdout) = previous_stdout.take() {
                cmd.stdin(stdout);
            }
            cmd.stdout(Stdio::piped());
            let mut child = cmd.spawn()?;
            let stdout = child.stdout.take().expect("never");
            previous_stdout = Some(Stdio::from(stdout));
            self.childs.push(child);
        }

        /* check whether to redirect the last cmd's stdio */
        if let Some(stdout) = previous_stdout.take() {
            last_cmd.stdin(stdout);
        }
        if opt_stdout.is_some() {
            last_cmd.stdout(Stdio::piped());
        }
        if opt_stderr.is_some() {
            last_cmd.stderr(Stdio::piped());
        }

        /* create the last thread from the last cmd */
        let mut child = last_cmd.spawn()?;
        let child_stdout = child.stdout.take();
        let child_stderr = child.stderr.take();
        self.childs.push(child);

        /* check whether to create the stdout thread */
        if let Some(mut cmd) = opt_stdout {
            cmd.stdin(Stdio::from(child_stdout.unwrap()));
            let child = cmd.spawn()?;
            self.childs.push(child);
        }

        /* check whether to create the stderr thread */
        if let Some(mut cmd) = opt_stderr {
            cmd.stdin(Stdio::from(child_stderr.unwrap()));
            let child = cmd.spawn()?;
            self.childs.push(child);
        }

        Ok(())
    }

    pub fn wait(&mut self) {
        for child in &mut self.childs {
            let _ = child.wait();
        }
    }

    pub fn kill(&mut self) -> std::io::Result<()> {
        for child in &mut self.childs {
            child.kill()?;
        }
        Ok(())
    }
}
