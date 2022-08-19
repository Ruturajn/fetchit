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
    println!("{}       : {}", format!("OS").red().bold().italic(), os_name);

    let kernel = fetchit::get_kernel_version();
    println!(
        "{}   : {}",
        format!("KERNEL").magenta().bold().italic(),
        kernel
    );

    let shell_name = fetchit::get_shell_name();
    println!(
        "{}    : {}",
        format!("SHELL").yellow().bold().italic(),
        shell_name
    );

    let session = fetchit::get_session_name();
    println!(
        "{}  : {}",
        format!("SESSION").blue().bold().italic(),
        session
    );

    let uptime = fetchit::get_sys_uptime();
    println!("{}   : {}", format!("UPTIME").cyan().bold().italic(), uptime);
    
    let total_packages = packages::get_num_packages();
    println!(
        "{} : {}",
        format!("PACKAGES").green().bold().italic(),
        total_packages
    );
}
