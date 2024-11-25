use std::process::{Command, Output};

use base64::prelude::*;

pub fn execute(command: impl AsRef<str>) -> Output {
    Command::new("powershell")
        .arg("-NoLogo")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-EncodedCommand")
        .arg(
            BASE64_STANDARD.encode(bytemuck::cast_slice::<u16, u8>(
                command
                    .as_ref()
                    .encode_utf16()
                    .collect::<Vec<u16>>()
                    .as_slice(),
            )),
        )
        .output()
        .unwrap()
}
