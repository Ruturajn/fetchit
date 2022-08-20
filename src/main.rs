// @Author: Ruturajn <nanotiruturaj@gmail.com>
// @Date  : 18th August, 2022
// @Brief : This is a system fetch tool written in Rust.

use colored::Colorize;

// Bring the functions from `lib.rs`, and 
// `packages.rs` into scope.
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
    let final_length = 11 + max_val + 5;

    // The number `14` defines the total characters, upto the output
    // for each system info value. For example,
    // ` OS          ` this will be printed before the name of the OS is
    // printed, and this text contains 14 characters. The `11` and `5` above
    // are chosen based on that formatting.

    println!("");
    println!("    {}        {}{}{}", format!("_").repeat(14).red(), format!("╭").blue(), format!("─").repeat(final_length).blue(), format!("╮").blue());
    println!("   {}       {} {}        {}  {}{}{}", format!("|  __________  |").red(), format!("│").blue(), format!("OS").red().bold().italic(), format!("").red(), os_name, format!(" ").repeat(final_length - 14 - string_length_vector[0]), format!("│").blue());
    println!("   {}       {} {}    {}  {}{}{}", format!("| :          : |").red(), format!("│").blue(), format!("KERNEL").magenta().bold().italic(), format!("").magenta(), kernel, format!(" ").repeat(final_length - 14 - string_length_vector[1]), format!("│").blue());
    println!("   {}       {} {}     {}  {}{}{}", format!("| :   Rust   : |").red(), format!("│").blue(), format!("SHELL").yellow().bold().italic(), format!("").yellow(), shell_name, format!(" ").repeat(final_length - 14 - string_length_vector[2]), format!("│").blue());
    println!("   {}       {} {}   {}  {}{}{}", format!("| :__________: |").red(), format!("│").blue(), format!("SESSION").blue().bold().italic(), format!("").blue(), session, format!(" ").repeat(final_length - 14 - string_length_vector[3]), format!("│").blue());
    println!("   {}       {} {}    {} {}{}{}", format!("|______________|").red(), format!("│").blue(), format!("UPTIME").cyan().bold().italic(), format!("祥").cyan(), uptime, format!(" ").repeat(final_length - 14 - string_length_vector[4]), format!("│").blue());
    println!("   {}      {} {}  {}  {}{}{}", format!("\\   =========   \\").blue(), format!("│").blue(), format!("PACKAGES").green().bold().italic(), format!("").green(), total_packages, format!(" ").repeat(final_length - 14 - string_length_vector[5]), format!("│").blue());
    println!("    {}     {}{}{}", format!("\\ ==== ____ ==  \\").blue(), format!("╰").blue(), format!("─").repeat(final_length).blue(), format!("╯").blue());
    println!("     {}   ", format!("\\_____\\___\\_____\\").blue());
    println!("");
}
