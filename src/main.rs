// @Author: Ruturajn <nanotiruturaj@gmail.com>
// @Date: 18th August, 2022
// @Brief: This is a system fetch tool written in Rust.

use std::env; // For getting commandline arguments and reading Environment
              // Variables
use std::error::Error;
use std::fs; // For reading files
use std::process::Command; // For exit with a code.
use colored::Colorize;

fn main() {
    let os_name = match get_os_name() {
        Ok(x) => x,
        Err(_) => String::from("Unknown"),
    };
    println!("{}      : {}", format!("OS").red().bold().italic(), os_name);

    let kernel = get_kernel_version();
    println!("{}  : {}", format!("KERNEL").magenta().bold().italic(), kernel);

    let shell_name = get_shell_name();
    println!("{}   : {}", format!("SHELL").yellow().bold().italic(),shell_name);

    let session = get_session_name();
    println!("{} : {}", format!("SESSION").blue().bold().italic(),session);

    let uptime = get_sys_uptime();
    println!("{}  : {}", format!("UPTIME").cyan().bold().italic(), uptime);
}

fn get_sys_uptime() -> String {
    let up_time = Command::new("uptime").arg("-p").output();

    let up_time = match up_time {
        Ok(x) => {
            let time = String::from_utf8(x.stdout)
                .unwrap()
                .replace("hours", "h")
                .replace("minutes", "m")
                .replace("up ", "");
            time
        }
        Err(_) => "Unknown".to_string(),
    };

    up_time
}

fn get_kernel_version() -> String {
    let kernel_ver = Command::new("uname").arg("-r").output();

    let kernel_ver = match kernel_ver {
        Ok(x) => {
            let rev_kernel_ver: String =
                String::from_utf8(x.stdout).unwrap().chars().rev().collect();

            let rev_kernel_ver = rev_kernel_ver
                .split("-")
                .last()
                .unwrap()
                .chars()
                .rev()
                .collect();

            rev_kernel_ver
        }
        Err(_) => "Unknown".to_string(),
    };

    kernel_ver
}

fn get_shell_name() -> String {
    let shell_var = "SHELL";
    match env::var(shell_var) {
        Ok(mut val) => {
            val = val.replace("/", " ");
            val.split(" ").last().unwrap().to_string()
        }
        Err(_) => "Unknown".to_string(),
    }
}

fn get_session_name() -> String {
    let session_name = "DESKTOP_SESSION";
    match env::var(session_name) {
        Ok(val) => val,
        Err(_) => {
            let session_name = "XDG_SESSION_DESKTOP";
            match env::var(session_name) {
                Ok(val) => val,
                Err(_) => {
                    let session_name = "XDG_CURRENT_DESKTOP";
                    match env::var(session_name) {
                        Ok(val) => val,
                        Err(_) => "Unknown".to_string(),
                    }
                }
            }
        }
    }
}

fn get_os_name() -> Result<String, Box<dyn Error>> {
    // println!("You are running : {}", os_name);
    let os_name = Command::new("lsb_release").arg("-sd").output();

    let os_name = match os_name {
        Ok(x) => String::from_utf8(x.stdout).unwrap(),
        Err(_) => {
            // Read the `/etc/os-release` file if `lsb_release` does not exist.
            let file_name = String::from("/home/ruturajn/test/os-release");

            // Read the file
            let file_contents = fs::read_to_string(file_name)?;

            // Search for `PRETTY_NAME`
            let search_string = "PRETTY_NAME";

            for line in file_contents.lines() {
                if line.contains(search_string) {
                    // Once found, remove everything after the `=` sign.
                    let vec_new = &line[12..];
                    // Remove the '"' , i.e. double quotes from the output.
                    let vec_new = vec_new.replace('"', "");
                    return Ok(vec_new.to_string());
                }
            }
            String::from("Unknown")
        }
    };
    // Remove the '"' , i.e. double quotes from the output.
    let os_name = os_name.replace('"', "");

    Ok(os_name)
}
