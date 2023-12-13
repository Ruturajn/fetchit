// @Author: Ruturajn <nanotiruturaj@gmail.com>
// @Date  : 18th August, 2022
// @Brief : This is a system fetch tool for Linux written in Rust.

use clap::Parser;
use colored::Color;
use colored::Colorize;
use std::fs;
use std::thread;
use std::sync::mpsc;

// Bring the functions from `lib.rs`, and
// `packages.rs` into scope.

pub mod packages;

fn main() {
    let args = FetchitArgs::parse();

    let (tx0, rx0) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();
    let (tx4, rx4) = mpsc::channel();
    let (tx5, rx5) = mpsc::channel();
    let (tx6, rx6) = mpsc::channel();

    let os_name_th = thread::spawn(move || {
        let os_name = match fetchit::get_os_name() {
            Ok(x) => x,
            Err(_) => String::from("Unknown"),
        };
        tx0.send(os_name).unwrap();
    });

    let kernel_th = thread::spawn(move || {
        let kernel = fetchit::get_kernel_version();
        tx1.send(kernel).unwrap();
    });

    let shell_name_th = thread::spawn(move || {
        let shell_name = fetchit::get_shell_name();
        tx2.send(shell_name).unwrap();
    });

    let session_th = thread::spawn(move || {
        let session = fetchit::get_session_name();
        tx3.send(session).unwrap();
    });

    let uptime_th = thread::spawn(move || {
        let uptime = fetchit::get_sys_uptime();
        tx4.send(uptime).unwrap();
    });

    let total_packages_th = thread::spawn(move || {
        let total_packages = packages::get_num_packages().to_string();
        tx5.send(total_packages).unwrap();
    });

    let hostname_th = thread::spawn(move || {
        let hostname = fetchit::get_hostname();
        tx6.send(hostname).unwrap();
    });

    os_name_th.join().unwrap();
    kernel_th.join().unwrap();
    shell_name_th.join().unwrap();
    session_th.join().unwrap();
    uptime_th.join().unwrap();
    total_packages_th.join().unwrap();
    hostname_th.join().unwrap();

    let os_name = rx0.recv().unwrap();
    let kernel = rx1.recv().unwrap();
    let shell_name = rx2.recv().unwrap();
    let session = rx3.recv().unwrap();
    let uptime = rx4.recv().unwrap();
    let total_packages = rx5.recv().unwrap();
    let hostname = rx6.recv().unwrap();

    // Create a vector to store the lengths of all the strings
    let string_length_vector = vec![
        os_name.len(),
        kernel.len(),
        shell_name.len(),
        session.len(),
        uptime.len(),
        total_packages.len(),
        hostname.len(),
    ];

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

    // Define the default ascii art.
    let mut ascii_string = "     ______________        
    |  __________  |       
    | :          : |       
    | :   Rust   : |       
    | :__________: |       
    |______________|       
    \\   =========   \\      
     \\ ==== ____ === \\     
      \\_____\\___\\_____\\    "
        .to_string();

    // Check for custom file, if given.
    let custom_ascii_string = match args.file_path {
        Some(x) => match fs::read_to_string(x) {
            Ok(y) => y,
            Err(_) => "Unknown".to_string(),
        },
        None => "Unknown".to_string(),
    };

    // Update the ascii art if a file was passed, but
    // a check for the required length is also done.
    if custom_ascii_string != *"Unknown" {
        let mut ascii_lines = 0;
        for _ in custom_ascii_string.lines() {
            ascii_lines += 1;
        }
        if ascii_lines >= 9 {
            ascii_string = custom_ascii_string;
        }
    }

    let mut ascii_vec: Vec<String> = Vec::new();

    for line in ascii_string.lines() {
        ascii_vec.push(line.to_string());
    }

    let top_color = args
        .top_color
        .unwrap_or_else(|| "red".into())
        .parse()
        .unwrap_or(Color::Red);
    let bottom_color = args
        .bottom_color
        .unwrap_or_else(|| "blue".into())
        .parse()
        .unwrap_or(Color::Blue);
    for (i, item) in ascii_vec.iter_mut().enumerate().take(9) {
        let color = if i < 6 { top_color } else { bottom_color };
        *item = item.color(color).to_string();
    }

    let mut box_side = "│".to_string();
    let mut box_top = "─".to_string().repeat(final_length);
    let mut box_top_left_corner = "╭".to_string();
    let mut box_top_right_corner = "╮".to_string();
    let mut box_bottom_left_corner = "╰".to_string();
    let mut box_bottom_right_corner = "╯".to_string();

    let outer_box_color = args
        .outer_box_color
        .unwrap_or_else(|| "blue".into())
        .parse()
        .unwrap_or(Color::Blue);
    box_side = box_side.color(outer_box_color).to_string();
    box_top = box_top.color(outer_box_color).to_string();
    box_top_left_corner = box_top_left_corner.color(outer_box_color).to_string();
    box_top_right_corner = box_top_right_corner.color(outer_box_color).to_string();
    box_bottom_left_corner = box_bottom_left_corner.color(outer_box_color).to_string();
    box_bottom_right_corner = box_bottom_right_corner.color(outer_box_color).to_string();

    // The number `14` defines the total characters, upto the output
    // for each system info value. For example,
    // ` OS          ` this will be printed before the name of the OS is
    // printed, and this text contains 14 characters. The `11` and `5` above
    // are chosen based on that formatting.

    println!();
    println!(
        "{} {}{}{}",
        ascii_vec[0], box_top_left_corner, box_top, box_top_right_corner
    );
    println!(
        "{} {} {}        {}  {}{}{}",
        ascii_vec[1],
        box_side,
        "OS".to_string().red().bold().italic(),
        "".to_string().red(),
        os_name,
        " ".to_string()
            .repeat(final_length - 14 - string_length_vector[0]),
        box_side
    );
    println!(
        "{} {} {}    {}  {}{}{}",
        ascii_vec[2],
        box_side,
        "KERNEL".to_string().magenta().bold().italic(),
        "".to_string().magenta(),
        kernel,
        " ".to_string()
            .repeat(final_length - 14 - string_length_vector[1]),
        box_side
    );
    println!(
        "{} {} {}     {}  {}{}{}",
        ascii_vec[3],
        box_side,
        "SHELL".to_string().yellow().bold().italic(),
        "".to_string().yellow(),
        shell_name,
        " ".to_string()
            .repeat(final_length - 14 - string_length_vector[2]),
        box_side
    );
    println!(
        "{} {} {}   {}  {}{}{}",
        ascii_vec[4],
        box_side,
        "SESSION".to_string().blue().bold().italic(),
        "".to_string().blue(),
        session,
        " ".to_string()
            .repeat(final_length - 14 - string_length_vector[3]),
        box_side
    );
    println!(
        "{} {} {}    {} {}{}{}",
        ascii_vec[5],
        box_side,
        "UPTIME".to_string().cyan().bold().italic(),
        "祥".to_string().cyan(),
        uptime,
        " ".to_string()
            .repeat(final_length - 14 - string_length_vector[4]),
        box_side
    );
    println!(
        "{} {} {}  {}  {}{}{}",
        ascii_vec[6],
        box_side,
        "PACKAGES".to_string().green().bold().italic(),
        "".to_string().green(),
        total_packages,
        " ".to_string()
            .repeat(final_length - 14 - string_length_vector[5]),
        box_side
    );
    println!(
        "{} {} {}  {}  {}{}{}",
        ascii_vec[7],
        box_side,
        "HOSTNAME".to_string().white().bold().italic(),
        "".to_string().white(),
        hostname,
        " ".to_string()
            .repeat(final_length - 14 - string_length_vector[6]),
        box_side
    );
    println!(
        "{} {}{}{}",
        ascii_vec[8], box_bottom_left_corner, box_top, box_bottom_right_corner
    );
    println!();
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct FetchitArgs {
    /// Color for the top part of the ascii art
    /// : black, red, yellow, blue, magenta, cyan, white, green
    #[clap(short, long, value_parser)]
    top_color: Option<String>,

    /// Color for the bottom part of the ascii art
    /// : black, red, yellow, blue, magenta, cyan, white, green
    #[clap(short, long, value_parser)]
    bottom_color: Option<String>,

    /// Color for the box
    /// : black, red, yellow, blue, magenta, cyan, white, green
    #[clap(short, long, value_parser)]
    outer_box_color: Option<String>,

    /// File path for the ascii text file
    #[clap(short, long, parse(from_os_str))]
    file_path: Option<std::path::PathBuf>,
}
