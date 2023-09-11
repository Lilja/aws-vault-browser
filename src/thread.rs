use std::io;
use std::process::{Command, ExitStatus, Stdio};

pub fn run_and_capture(command: &mut Command) -> io::Result<(ExitStatus, Vec<u8>, Vec<u8>)> {
    let cmd = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let x = cmd.wait_with_output().unwrap();

    Ok((x.status, x.stdout, x.stderr))
}
