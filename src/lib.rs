use std::process::Command;

#[derive(Debug, Clone)]
pub(crate) struct Socket(String);

impl Socket {
    fn new(str: &str) -> Self {
        Self(str.to_string())
    }
}

impl Default for Socket {
    fn default() -> Self {
        Self::new("/tmp/nvim/socket")
    }
}

impl Socket {
    fn path(&self) -> String {
        self.0.to_owned()
    }
}

enum SocketCommand {
    Open(Vec<String>),
    Send(String),
    Expr(String),
}

impl SocketCommand {
    pub fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        let mut other = match self {
            Self::Open(files) => {
                let mut c = vec!["--".to_owned()];
                c.append(&mut files.clone());
                c
            },
            Self::Send(keys) => {
               let mut c = vec!["--remote-send".to_owned()];
                c.push(keys.to_string());
                c
            },
            Self::Expr(expr) => {
               let mut c = vec!["--remote-expr".to_owned()];
                c.push(expr.to_string());
                c
            },
        };
        args.append(&mut other);
        args
    }
}

impl Socket {
    fn exec(&self, cmd: &SocketCommand) -> anyhow::Result<()> {
        let mut args = Vec::new();
        args.push("--server".to_owned());

        let socket_binding = self.path();
        args.push(socket_binding);

        let mut cmd_binding = cmd.to_args();
        args.append(&mut cmd_binding);

        let mut exec_cmd = Command::new("nvim");
        exec_cmd.args(args);

        let mut child = exec_cmd.spawn()?;
        child.wait()?;
        Ok(())
    }
}

pub(crate) fn open_files<P>(socket: &Socket, paths: Vec<String>) -> anyhow::Result<()>
{
    let cmd = SocketCommand::Open(paths);
    socket.exec(&cmd)?;
    Ok(())
}
