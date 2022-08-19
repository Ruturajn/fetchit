// @Author: Ruturajn <nanotiruturaj@gmail.com>
// @Date  : 18th August, 2022
// @Brief : This is a system fetch tool written in Rust.

use colored::Colorize;

// Import the functions from `lib.rs`
use fetchit;
pub mod packages;

fn main() {
    let os_name = match fetchit::get_os_name() {
        Ok(x) => x,
        Err(_) => String::from("Unknown"),
    };

    let kernel = fetchit::get_kernel_version();

    let shell_name = fetchit::get_shell_name();

    let session = fetchit::get_session_name();

    let uptime = fetchit::get_sys_uptime();

    let total_packages = packages::get_num_packages().to_string();

    // Create a vector to store the lengths of all the strings
    let string_length_vector = vec![os_name.len(),
                                    kernel.len(),
                                    shell_name.len(),
                                    session.len(),
                                    uptime.len(),
                                    total_packages.len()];

    // Initialize the maximum string length to `0.`
    let mut max_val = 0;

    // Iterate over the vector, to find the maximum string length.
    for val in &string_length_vector {
        if val > &max_val {
            max_val = *val;
        }
    }
    
    // Define the length for which the horizontal characters `─`
    // should be repeated.
    let final_length = 9 + max_val + 5;

    // The number `12` defines the total characters, upto the output
    // for each system info value. For example,
    // ` OS       : ` this will be printed before the name of the OS is
    // printed, and this text contains 12 characters. The `9` and `5` above
    // are chosen based on formatting.

    println!("    ______________        ╭{}╮", format!("─").repeat(final_length));
    println!("   |  __________  |       │ {}       : {}{}{}", format!("OS").red().bold().italic(), os_name, format!(" ").repeat(final_length - 12 - string_length_vector[0]), format!("│"));
    println!("   | :          : |       │ {}   : {}{}{}", format!("KERNEL").magenta().bold().italic(), kernel, format!(" ").repeat(final_length - 12 - string_length_vector[1]), format!("│"));
    println!("   | :   Rust   : |       │ {}    : {}{}{}", format!("SHELL").yellow().bold().italic(), shell_name, format!(" ").repeat(final_length - 12 - string_length_vector[2]), format!("│"));
    println!("   | :__________: |       │ {}  : {}{}{}", format!("SESSION").blue().bold().italic(), session, format!(" ").repeat(final_length - 12 - string_length_vector[3]), format!("│"));
    println!("   |______________|       │ {}   : {}{}{}", format!("UPTIME").cyan().bold().italic(), uptime, format!(" ").repeat(final_length - 12 - string_length_vector[4]), format!("│"));
    println!("   \\   =========  \\       │ {} : {}{}{}", format!("PACKAGES").green().bold().italic(), total_packages, format!(" ").repeat(final_length - 12 - string_length_vector[5]), format!("│"));
    println!("    \\ ==== ____ == \\      ╰{}╯", format!("─").repeat(final_length));
    println!("     \\_____\\___\\____\\   ");
}
