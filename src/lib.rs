use std::process::{Command, Output};

pub struct PowerShell {
    buf: Vec<String>,
}

impl PowerShell {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn no_confirm(mut self) -> Self {
        self.buf.push(
            "$ConfirmPreference=[System.Management.Automation.ConfirmImpact]::None".to_string(),
        );
        self
    }

    pub fn add_variable(mut self, name: &str, value: &str) -> Self {
        self.buf.push(format!("${}='{}'", name, value));
        self
    }

    pub fn invoke(mut self, command: &str) -> Output {
        self.buf.push(command.to_string());
        use base64::prelude::*;
        let encoded = BASE64_STANDARD.encode(bytemuck::cast_slice::<u16, u8>(
            &self.buf.join("\n").encode_utf16().collect::<Vec<_>>(),
        ));
        Command::new("PowerShell")
            .arg("-NoLogo")
            .arg("-NoProfile")
            .arg("-NonInteractive")
            .arg("-EncodedCommand")
            .arg(encoded)
            .output()
            .unwrap()
    }
}
