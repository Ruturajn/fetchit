// @Author: Ruturajn <nanotiruturaj@gmail.com>
// @Date  : 19th August, 2022
// @Brief : This is the lib.rs file for `fetchit`

use std::env; // For getting commandline arguments and reading Environment
              // Variables.
use std::error::Error;
use std::fs; // For reading files.
use std::process::Command; // For exit with a code.

pub fn get_os_name() -> Result<String, Box<dyn Error>> {
    // Get the name of the Distribution, using the `lsb_release` command.
    let os_name = Command::new("lsb_release").arg("-sd").output();

    let os_name = match os_name {
        Ok(x) => String::from_utf8(x.stdout).unwrap(),
        Err(_) => {
            // Read the `/etc/os-release` file if `lsb_release` does not exist.
            let file_name = String::from("/etc/os-release");

            // Read the file
            let file_contents = fs::read_to_string(file_name)?;

            // Search for `PRETTY_NAME`
            let search_string = "PRETTY_NAME";

            for line in file_contents.lines() {
                if line.contains(search_string) {
                    // Get the value for the key, `PRETTY_NAME`
                    let vec_new = line.split('=').last().unwrap();
                    // Remove the '"' , i.e. double quotes from the output.
                    let vec_new = vec_new.replace('"', "");
                    return Ok(vec_new);
                }
            }
            String::from("Unknown")
        }
    };
    // Remove the '"' , i.e. double quotes from the output.
    let os_name = os_name.replace('"', "");
    let os_name = os_name.replace('\n', ""); // Remove any newline character

    Ok(os_name)
}

pub fn get_kernel_version() -> String {
    // Get the kernel version with `uname -r`.
    let kernel_ver = Command::new("uname").arg("-r").output();

    let kernel_ver = match kernel_ver {
        Ok(x) => {
            // Reverse the string obtained from the output.
            let rev_kernel_ver: String =
                String::from_utf8(x.stdout).unwrap().chars().rev().collect();

            // Split the string based on `-`, and then reverse it again,
            // to obtain only the kernel version, and not any other info.
            let rev_kernel_ver = rev_kernel_ver
                .split('-')
                .last()
                .unwrap()
                .chars()
                .rev()
                .collect();

            rev_kernel_ver
        }
        Err(_) => "Unknown".to_string(), // If the commnd fails assingn
                                         // kernel_ver to "Unknown".
    };

    kernel_ver
}

pub fn get_shell_name() -> String {
    // Read the value of the Environment Variable, `SHELL`
    // to obtain the current shell name.
    let shell_var = "SHELL";
    match env::var(shell_var) {
        Ok(mut val) => {
            val = val.replace('/', " "); // Replace all the forward slashes
                                         // with a space.
            val.split(' ').last().unwrap().to_string() // Split the string
                                                       // based on the spaces
                                                       // and get the last word.
        }
        Err(_) => "Unknown".to_string(), // If the Environment variable is
                                         // not read, return "Unknown".
    }
}

pub fn get_session_name() -> String {
    // Read the value of the Environment Variable, `DESKTOP_SESSION`
    // to obtain the name of the DE(Desktop Environment) or WM(Window Manager).
    let session_name = "DESKTOP_SESSION";
    match env::var(session_name) {
        Ok(val) => val,
        Err(_) => {
            let session_name = "XDG_SESSION_DESKTOP"; // If reading `DESKTOP_SESSION`
                                                      // fails try reading `XDG_SESSION_DESKTOP`.
            match env::var(session_name) {
                Ok(val) => val,
                Err(_) => {
                    let session_name = "XDG_CURRENT_DESKTOP"; // If reading `XDG_SESSION_DESKTOP`
                                                              // fails try reading `XDG_CURRENT_DESKTOP`.
                    match env::var(session_name) {
                        Ok(val) => val,
                        Err(_) => {
                            // Now, we try looking at `_NET_WM_NAME`, by using `xprop`.
                            let xprop_id = Command::new("xprop")
                                .args(["-root", "-notype", "_NET_SUPPORTING_WM_CHECK"])
                                .output();

                            // If the above commnd ran successfully, assign its output to `xprop_id`.
                            let xprop_id = match xprop_id {
                                Ok(x) => String::from_utf8(x.stdout).unwrap(),
                                Err(_) => "Unknown".to_string(),
                            };

                            // Extract the ID
                            let xprop_id = xprop_id.split(' ').last().unwrap();

                            // Call `xprop` again, but now by passing in the ID, we just found.
                            let mut wm_name = match Command::new("xprop")
                                .args(["-id", xprop_id, "-notype"])
                                .output()
                            {
                                Ok(x) => String::from_utf8(x.stdout).unwrap(),
                                Err(_) => "Unknown".to_string(),
                            };

                            // Now, from the output, of the above call, we look for `_NET_WM_NAME`.
                            for line in wm_name.lines() {
                                if line.contains("_NET_WM_NAME") {
                                    wm_name = line
                                        .split('=')
                                        .last()
                                        .unwrap()
                                        .to_string()
                                        .replace('"', "") // Remove double-quotes.
                                        .replace(' ', ""); // Remove space literal, which is
                                                           // present between the `_NET_WM_NAME`
                                                           // and it's value, after the `=` sign.
                                    return wm_name;
                                }
                            }
                            // If all else fails, return "Unknown".
                            String::from("Unknown")
                        }
                    }
                }
            }
        }
    }
}

pub fn get_sys_uptime() -> String {
    // Get the uptime using the `uptime -p` command.
    let up_time = Command::new("uptime").arg("-p").output();

    let up_time = match up_time {
        Ok(x) => {
            // Remove the word up.
            String::from_utf8(x.stdout)
                .unwrap()
                .replace("hours", "h") // Replace words with letters.
                .replace("hour", "h")
                .replace("minutes", "m")
                .replace("minute", "m")
                .replace("days", "d")
                .replace("day", "d")
                .replace("up ", "")
        }
        Err(_) => "Unknown".to_string(), // If the commnd fails, assingn
                                         // up_time to "Unknown".
    };

    // Remove any newline character.

    up_time.replace('\n', "")
}
