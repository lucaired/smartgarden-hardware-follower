use std::process::{Command, Output};

// TODO: env
// hardcoded value for the usb hub on the raspberry pi
const FAN_PORT = 2;

#[derive(Debug, Fail)]
pub enum UsbControlError {
    #[fail(display = "Could not execute : {}", command)]
    CommandError { command: String },
}

/// Parse the argument provided:
/// `(on || off)` as switch argument
pub fn fan_control(switch: &str) -> Result<Output, UsbControlError> {
    let command = format!("sudo uhubctl -a {} -p {}", switch, FAN_PORT);
    execute(command)
}

fn execute(command: String) -> Result<Output, UsbControlError> {
    match Command::new("sh").arg("-c").arg(command.clone()).output() {
        Ok(output) => Ok(output),
        Err(_err) => Err(UsbControlError::CommandError { command }),
    }
}
