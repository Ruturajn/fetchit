// @Author: Ruturajn <nanotiruturaj@gmail.com>
// @Date  : 19th August, 2022
// @Brief : This file, contains the logic to get the
//          number of packages installed on the system.

use std::process::{Command, Stdio}; // For executing shell commands.

pub fn get_num_packages() -> u32 {
    // For Arch Based Distributions.
    let num_packages = match packages_generic("pacman", vec!["-Q"]) {
        Ok(x) => x,
        Err(_) => {
            // For Debian Based Distributions.
            match packages_debian_based() {
                Ok(x) => x,
                Err(_) => {
                    // For Fedora based Distributions.
                    match packges_fedora_based() {
                        Ok(x) => x,
                        Err(_) => {
                            // For BSD Based Distributions.
                            match packages_generic("pkg", vec!["info"]) {
                                Ok(x) => x,
                                Err(_) => {
                                    // For Gentoo Based Distributions.
                                    match packages_generic("ls", vec!["-d", "var/db/pkg/*/*"]) {
                                        Ok(x) => x,
                                        Err(_) => {
                                            // For Venon Linux Based Distributions.
                                            match packages_generic(
                                                "ls",
                                                vec!["-d", "/var/lib/scratchpkg/db/*"],
                                            ) {
                                                Ok(x) => x,
                                                Err(_) => {
                                                    // For Solus Based Distributions.
                                                    match packages_generic(
                                                        "ls",
                                                        vec!["/var/lib/eopkg/package/"],
                                                    ) {
                                                        Ok(x) => x,
                                                        Err(_) => {
                                                            // For Void Linux Based Distributions.
                                                            match packages_generic(
                                                                "xbps-query",
                                                                vec!["-l"],
                                                            ) {
                                                                Ok(x) => x,
                                                                Err(_) => {
                                                                    // For OpenSUSE Based Distributions.
                                                                    match packages_generic(
                                                                        "rpm",
                                                                        vec!["-qa"],
                                                                    ) {
                                                                        Ok(x) => x,
                                                                        Err(_) => {
                                                                            // For NixOS Based
                                                                            // Distributions.
                                                                            match packages_nixos_based() {
                                                                                Ok(x) => x,
                                                                                Err(_) => String::from("Unknown")
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    // Count the total number of packages
    let mut total_count: u32 = 0;
    for _ in num_packages.lines() {
        total_count += 1;
    }

    total_count
}

pub fn packages_generic(cmd: &str, options: Vec<&str>) -> Result<String, String> {
    // Use `pkg info` to list the installed packages.
    let packages = Command::new(cmd).args(options).output();

    // Check if the above command executed, successfully,
    // if so, unwrap the output from stdout, and return it.
    match packages {
        Ok(x) => Ok(String::from_utf8(x.stdout).unwrap()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn packages_debian_based() -> Result<String, String> {
    // Getting the list of packages on a debian based system,
    // requires the use of pipes, i.e. we will need to pipe the
    // output from `dpkg-query -l` into `grep`, which turns out
    // to be `dpkg-query -l | grep "^ii"`. So, we first use `Stdio::piped()`
    // to write the output from `dpkg-query -l`.
    let packages = Command::new("dpkg-query")
        .arg("-l")
        .stdout(Stdio::piped())
        .spawn();

    // Then, we check whether the command above ran successfully.
    let packages_output = match packages {
        Ok(mut x) => {
            // If it did, then, take the output from `Stdio::piped()`.
            if let Some(out) = x.stdout.take() {
                // If the output was successfully taken from `Stdio::piped()`,
                // then we pipe it into `grep`.
                let grep_cmd = match Command::new("grep")
                    .arg("^ii")
                    .stdin(out)
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    // Once the piping is successfully done, we use `wait_with_output`, to collect the
                    // output and kill the process.
                    Ok(y) => match y.wait_with_output() {
                        Ok(z) => Ok(String::from_utf8(z.stdout).unwrap()),
                        Err(e) => return Err(e.to_string()),
                    },
                    Err(e) => return Err(e.to_string()),
                };
                // If there are no errors, during this piping process,
                // we return the output from `grep`, which will then
                // be assigned to `packages_output`.
                grep_cmd
            } else {
                return Err("Unknown".to_string());
            }
        }
        Err(e) => return Err(e.to_string()),
    };

    // We return `packages_output`, i.e. list of packages installed.
    packages_output
}

pub fn packges_fedora_based() -> Result<String, String> {
    // Use `yum list installed` to get the list of packages installed.
    let packages = Command::new("yum").args(["list", "installed"]).output();

    // Check if the above command executed, successfully,
    // if so, unwrap the output from stdout, and return it.
    match packages {
        Ok(x) => Ok(String::from_utf8(x.stdout).unwrap()),
        Err(_) => {
            // If `yum list installed` fails, try running
            // `rpm -qa` to get the list of installed packages.
            let packages = Command::new("dnf").args(["list", "installed"]).output();
            match packages {
                Ok(x) => Ok(String::from_utf8(x.stdout).unwrap()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}

pub fn packages_nixos_based() -> Result<String, String> {
    // Use `nix-store -qR /run/current-system/sw/ 2>/dev/null && nix-store -qR ~/.nix-profile/`
    // to get the list of installed packages. So, we will first get the output for the
    // first command in the shell chain.
    let packages = Command::new("nix-store")
        .args(["-qR", "/run/current-system/sw/"])
        .output();

    match packages {
        Ok(x) => {
            // Once the first chain command succeeds, we will add it's output,
            // to the second chain command.
            let packages_output = String::from_utf8(x.stdout).unwrap();
            match Command::new("nix-store")
                .args(["-qR", "~/.nix-profile/"])
                .output()
            {
                Ok(y) => {
                    let prev_output = String::from_utf8(y.stdout).unwrap();
                    Ok(format!("{}{}", packages_output, prev_output))
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
