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
        Err(_) => "Unknown".to_string(), // If the command fails assign
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
    // First check `DESKTOP_SESSION`.
    // Read the value of the Environment Variable, `DESKTOP_SESSION`
    // to obtain the name of the DE(Desktop Environment) or WM(Window Manager).
    let session_name = "DESKTOP_SESSION";

    // If that fails, assign `wm_name` to `Unknown`
    let wm_name = env::var(session_name).unwrap_or_else(|_| "Unknown".to_string());

    // If there was an error, in reading `DESKTOP_SESSION`, or if it is empty,
    // try reading `XDG_SESSION_DESKTOP`.
    if wm_name.is_empty() || wm_name == *"Unknown" {
        let session_name = "XDG_SESSION_DESKTOP";
        let wm_name = env::var(session_name).unwrap_or_else(|_| "Unknown".to_string());

        // If there was an error, in reading `XDG_SESSION_DESKTOP`, or if it is
        // empty, try reading `XDG_CURRENT_DESKTOP`.
        if wm_name.is_empty() || wm_name == *"Unknown" {
            let session_name = "XDG_CURRENT_DESKTOP";
            let wm_name = env::var(session_name).unwrap_or_else(|_| "Unknown".to_string());

            // If there was an error, in reading `XDG_CURRENT_DESKTOP`, fall back
            // to reading `_NET_WM_NAME` using `xprop`.
            if wm_name.is_empty() || wm_name == *"Unknown" {
                // Now, we try looking at `_NET_WM_NAME`, by using `xprop`.
                let xprop_id = Command::new("xprop")
                    .args(["-root", "-notype", "_NET_SUPPORTING_WM_CHECK"])
                    .output();

                // If the above command ran successfully, assign its output to `xprop_id`.
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
            } else {
                wm_name
            }
        } else {
            wm_name
        }
    } else {
        wm_name
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
        Err(_) => "Unknown".to_string(), // If the command fails, assign
                                         // up_time to "Unknown".
    };

    // Remove any newline character.

    up_time.replace('\n', "")
}

pub fn get_hostname() -> String {
    // Get the hostname using the 'hostname' command
    let hostname = Command::new("hostname").output();
    let hostname = match hostname {
        Ok(x) => String::from_utf8(x.stdout).unwrap(),
        Err(_) => {
            let hostname = Command::new("uname").arg("-n").output();
            match hostname {
                Ok(x) => String::from_utf8(x.stdout).unwrap(),
                Err(_) => "Unknown".to_string(),
            }
        }
    };
    // Remove any new line character
    hostname.replace('\n', "")
}

// Add some tests, for testing the `get_session_name()` function.
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // #[test]
    // fn fallback_session() {
    //     // Remove all the checked Environment variables.
    //     let env_var_1 = "DESKTOP_SESSION";
    //     env::remove_var(env_var_1);

    //     let env_var_2 = "XDG_CURRENT_DESKTOP";
    //     env::remove_var(env_var_2);

    //     let env_var_3 = "XDG_SESSION_DESKTOP";
    //     env::remove_var(env_var_3);

    //     let wm_name = get_session_name();
    //     assert_eq!(wm_name, "LG3D");
    // }

    #[test]
    fn xdg_current() {
        // Remove all the checked Environment variables.
        let env_var_1 = "DESKTOP_SESSION";
        env::remove_var(env_var_1);

        let env_var_2 = "XDG_CURRENT_DESKTOP";
        env::remove_var(env_var_2);

        let env_var_3 = "XDG_SESSION_DESKTOP";
        env::remove_var(env_var_3);

        // Set `XDG_SESSION_DESKTOP`
        let env_var = "XDG_CURRENT_DESKTOP";
        env::set_var(env_var, "Qtile");

        let wm_name = get_session_name();
        assert_eq!(wm_name, "Qtile");
    }

    #[test]
    fn xdg_session() {
        // Remove all the checked Environment variables.
        let env_var_1 = "DESKTOP_SESSION";
        env::remove_var(env_var_1);

        let env_var_2 = "XDG_CURRENT_DESKTOP";
        env::remove_var(env_var_2);

        let env_var_3 = "XDG_SESSION_DESKTOP";
        env::remove_var(env_var_3);

        // Set `XDG_SESSION_DESKTOP`
        let env_var = "XDG_SESSION_DESKTOP";
        env::set_var(env_var, "Testing");

        let wm_name = get_session_name();
        assert_eq!(wm_name, "Testing");
    }
}
