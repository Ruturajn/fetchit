// @Author: Ruturajn <nanotiruturaj@gmail.com>
// @Date  : 18th August, 2022
// @Brief : This file, contains the logic to get the
//          number of packages installed on the system.

use std::process::{Command, Stdio}; // For executing shell commands.

pub fn get_num_packages() -> u32 {
    // For Arch Based distributions.
    let num_packages = match packages_arch_linux_based() {
        Ok(x) => x,
        Err(_) => {
            // For Debian Based Distributions
            let _num_packages = match packages_debian_based() {
                Ok(x) => x,
                Err(_) => return 1,
            };
            return 1;
        }
    };

    // Count the total number of packages
    let mut total_count: u32 = 0;
    for _ in num_packages.lines() {
        total_count += 1;
    }

    total_count
}

pub fn packages_debian_based() -> Result<String, String> {
    let packages = Command::new("dpkg-query").arg("-l").stdout(Stdio::piped()).spawn();

    let packages_output = match packages {
        Ok(mut x) => {
            if let Some(out) = x.stdout.take() {
            let grep_cmd = match Command::new("grep").arg("^ii").stdin(out).stdout(Stdio::piped()).spawn() {
                Ok(y) => match y.wait_with_output() {
                    Ok(z) => Ok(String::from_utf8(z.stdout).unwrap()),
                    Err(e) => return Err(e.to_string()),
                },
                Err(e) => return Err(e.to_string()),
            };
                grep_cmd
            } else {
                return Err("Unknown".to_string());
            }
        },
        Err(e) => return Err(e.to_string()),
    };

    packages_output
}

pub fn packages_arch_linux_based() -> Result<String, String> {
    let packages = Command::new("pacman").arg("-Q").output();
    
    match packages {
        Ok(x) => Ok(String::from_utf8(x.stdout).unwrap()),
        Err(e) => return Err(e.to_string()),
    }
}
