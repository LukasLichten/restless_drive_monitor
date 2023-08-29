use std::{io, fs, path::PathBuf, process::{Command, Output}};

pub fn install() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        println!("Installing not supported under windows");
        return Ok(());
    }

    // We stop any running service
    let res = Command::new("systemctl")
            .arg("stop")
            .arg("restless-drive-monitor.service")
            .output();
    if let Ok(output) = res {
        print_output(output);
    }
    

    // Install
    let exec_path = fs::canonicalize(std::env::current_exe()?)?;
    let target_exec = PathBuf::from("/usr/bin/restless_drive_monitor");
    if exec_path != target_exec {
        fs::copy(exec_path, target_exec)?; // We could check if the user is root, but we can also just execute and let it fail
    }

    fs::create_dir_all("/etc/restless_drive_monitor")?; // We let the programm deal with creating the config on launch

    let service_file_bytes = include_bytes!("./assets/restless-drive-monitor.service");
    fs::write("/etc/systemd/system/restless-drive-monitor.service", service_file_bytes)?;

    print_output(Command::new("systemctl")
        .arg("daemon-reload")
        .output()?);
    

    print_output(Command::new("systemctl")
        .arg("start")
        .arg("restless-drive-monitor.service")
        .output()?);

    print_output(Command::new("systemctl")
        .arg("enable")
        .arg("restless-drive-monitor.service")
        .output()?);

    Ok(())
}

fn print_output(output: Output) {
    println!("{}{}", String::from_utf8(output.stdout).unwrap_or("".to_string()), String::from_utf8(output.stderr).unwrap_or("".to_string()));
}