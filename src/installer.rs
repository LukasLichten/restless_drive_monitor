use std::{io, fs, path::PathBuf, process::{Command, Output}};

use log::{error, debug, info};

pub fn install() -> io::Result<()> {
    if cfg!(target_os = "windows") {

        error!("Installing not supported under windows");
        return Ok(());
    }

    info!("Stopping Service...");

    // We stop any running service
    let res = Command::new("systemctl")
            .arg("stop")
            .arg("restless-drive-monitor.service")
            .output();
    if let Ok(output) = res {
        print_output(output);
    }
    info!("DONE");
    

    // Install
    info!("Copying executable into /usr/bin...");
    let exec_path = fs::canonicalize(std::env::current_exe()?)?;
    let target_exec = PathBuf::from("/usr/bin/restless_drive_monitor");
    if exec_path != target_exec {
        // We could check if the user is root, but we can also just execute and let it fail
        let res = fs::copy(exec_path, target_exec);
        if let Err(e) = res {
            error!("Failed to copy executable... Are you root?");
            return Err(e);
        }
    }
    info!("DONE");

    info!("Creating /etc/restless_drive_monitor...");
    fs::create_dir_all("/etc/restless_drive_monitor")?; // We let the programm deal with creating the config on launch
    info!("DONE");

    info!("Adding the service file...");
    let service_file_bytes = include_bytes!("./assets/restless-drive-monitor.service");
    fs::write("/etc/systemd/system/restless-drive-monitor.service", service_file_bytes)?;

    info!("Reloading the Daemon...");
    print_output(Command::new("systemctl")
        .arg("daemon-reload")
        .output()?);
    
    info!("Starting the 'restless-drive-monitor.service'...");
    print_output(Command::new("systemctl")
        .arg("start")
        .arg("restless-drive-monitor.service")
        .output()?);

    info!("Enabling the service on boot...");
    print_output(Command::new("systemctl")
        .arg("enable")
        .arg("restless-drive-monitor.service")
        .output()?);

    info!("All done!");

    Ok(())
}

fn print_output(output: Output) {
    debug!("{}{}", String::from_utf8(output.stdout).unwrap_or("".to_string()), String::from_utf8(output.stderr).unwrap_or("".to_string()));
}