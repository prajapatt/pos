use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, RawFd};
use std::process::{Child, Command, Stdio};

#[derive(Debug)]
pub struct PtySession {
    pub child: Child,
    pub master: File,
    pub slave: File,
}

#[derive(Debug, Clone, Default)]
pub struct PtyConfig {
    pub rows: u32,
    pub cols: u32,
    pub env: Vec<(String, String)>,
}

impl PtySession {
    pub fn spawn(command: &str, args: &[String], config: &PtyConfig) -> std::io::Result<Self> {
        let mut cmd = Command::new(command);
        cmd.args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .envs(config.env.iter().cloned());

        let child = cmd.spawn()?;

        let master = File::open("/dev/ptmx")?;
        let slave = File::open("/dev/pts/0")?;

        Ok(Self {
            child,
            master,
            slave,
        })
    }

    pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.master.write_all(data)
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.master.read(buffer)
    }

    pub fn resize(&mut self, rows: u32, cols: u32) -> std::io::Result<()> {
        let _ = rows;
        let _ = cols;
        Ok(())
    }

    pub fn wait(&mut self) -> std::io::Result<std::process::ExitStatus> {
        self.child.wait()
    }
}

pub fn create_pty_session(command: &str, args: &[String], config: &PtyConfig) -> std::io::Result<PtySession> {
    PtySession::spawn(command, args, config)
}

pub fn is_pty_available() -> bool {
    std::path::Path::new("/dev/ptmx").exists()
}

pub fn raw_fd(file: &File) -> RawFd {
    file.as_raw_fd()
}
