use std::{
    io::{Read, Write},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::Receiver,
    thread::JoinHandle,
};

use nom::{
    bytes::complete::{tag, take_until},
    sequence::preceded,
    IResult,
};
use uuid::Uuid;

pub struct PowerShell {
    child: Child,
    stdin: ChildStdin,
    receiver: Receiver<String>,
    outbuf: String,
    daemon: Option<JoinHandle<()>>,
}

impl PowerShell {
    pub fn new() -> Self {
        let mut child = Command::new("PowerShell")
            .args(["-NoLogo", "-NoProfile", "-NonInteractive", "-Command", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdin = child.stdin.take().unwrap();
        let mut stdout = child.stdout.take().unwrap();
        let (sender, receiver) = std::sync::mpsc::channel();
        let h = std::thread::spawn(move || {
            let mut buf = vec![0; 1024 * 1024];
            loop {
                let n = stdout.read(&mut buf).unwrap();
                if n == 0 {
                    break;
                }
                let s = String::from_utf8(buf[..n].to_vec()).unwrap();
                sender.send(s).unwrap()
            }
        });
        stdin.write_all(b"$ConfirmPreference='None'\n").unwrap();
        stdin
            .write_all(b"$ProgressPreference='SilentlyContinue'\n")
            .unwrap();
        Self {
            child,
            stdin,
            receiver,
            outbuf: String::new(),
            daemon: Some(h),
        }
    }

    pub fn exec(&mut self, cmd: &str) -> String {
        let d1 = d();
        let d2 = d();
        self.stdin
            .write_fmt(format_args!(
                "Write-Host '{}' -NoNewline;{};Write-Host '{}' -NoNewline\n",
                d1, cmd, d2
            ))
            .unwrap();
        loop {
            if self.outbuf.len() >= d1.len() + d2.len() {
                if let Ok((remaining, output)) = parse(&self.outbuf, &d1, &d2) {
                    let output = output.trim().to_string();
                    self.outbuf = remaining.to_string();
                    break output;
                }
            }
            let s = self.receiver.recv().unwrap();
            self.outbuf.push_str(&s);
        }
    }
}

impl Drop for PowerShell {
    fn drop(&mut self) {
        self.stdin.write_all(b"exit\n").unwrap();
        self.child.wait().unwrap();
        self.daemon.take().unwrap().join().unwrap();
    }
}

fn d() -> String {
    format!("_{}_", Uuid::new_v4())
}

fn parse<'a>(input: &'a str, d1: &str, d2: &str) -> IResult<&'a str, &'a str> {
    match preceded(tag(d1), take_until(d2))(input) {
        Ok((remaining, output)) => Ok((&remaining[d2.len()..], output)),
        e => e,
    }
}
