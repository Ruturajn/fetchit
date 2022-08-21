// @Author: Ruturajn <nanotiruturaj@gmail.com>
// @Date  : 18th August, 2022
// @Brief : This is a system fetch tool for Linux written in Rust.

use std::fs;
use colored::Colorize;
use clap::Parser;

// Bring the functions from `lib.rs`, and 
// `packages.rs` into scope.

pub mod packages;


fn main() {
    
    let args = FetchitArgs::parse();

    let os_name = match fetchit::get_os_name() {
        Ok(x) => x,
        Err(_) => String::from("Unknown"),
    };

    let kernel = fetchit::get_kernel_version();

    let shell_name = fetchit::get_shell_name();

    let session = fetchit::get_session_name();

    let uptime = fetchit::get_sys_uptime();

    let total_packages = packages::get_num_packages().to_string();

    let hostname = fetchit::get_hostname();
    // Create a vector to store the lengths of all the strings
    let string_length_vector = vec![os_name.len(),
                                    kernel.len(),
                                    shell_name.len(),
                                    session.len(),
                                    uptime.len(),
                                    total_packages.len(),
                                    hostname.len()];

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
    let mut ascii_string=
"     ______________        
    |  __________  |       
    | :          : |       
    | :   Rust   : |       
    | :__________: |       
    |______________|       
    \\   =========   \\      
     \\ ==== ____ === \\     
      \\_____\\___\\_____\\  ".to_string();

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

    match args.top_color {
        Some(x) => {
            if x == "blue" {
                ascii_vec[0] = ascii_vec[0].blue().to_string();
                ascii_vec[1] = ascii_vec[1].blue().to_string();
                ascii_vec[2] = ascii_vec[2].blue().to_string();
                ascii_vec[3] = ascii_vec[3].blue().to_string();
                ascii_vec[4] = ascii_vec[4].blue().to_string();
                ascii_vec[5] = ascii_vec[5].blue().to_string();
            } else if x == "yellow" {
                ascii_vec[0] = ascii_vec[0].yellow().to_string();
                ascii_vec[1] = ascii_vec[1].yellow().to_string();
                ascii_vec[2] = ascii_vec[2].yellow().to_string();
                ascii_vec[3] = ascii_vec[3].yellow().to_string();
                ascii_vec[4] = ascii_vec[4].yellow().to_string();
                ascii_vec[5] = ascii_vec[5].yellow().to_string();
            } else if x == "magenta" {
                ascii_vec[0] = ascii_vec[0].magenta().to_string();
                ascii_vec[1] = ascii_vec[1].magenta().to_string();
                ascii_vec[2] = ascii_vec[2].magenta().to_string();
                ascii_vec[3] = ascii_vec[3].magenta().to_string();
                ascii_vec[4] = ascii_vec[4].magenta().to_string();
                ascii_vec[5] = ascii_vec[5].magenta().to_string();
            } else if x == "cyan" {
                ascii_vec[0] = ascii_vec[0].cyan().to_string();
                ascii_vec[1] = ascii_vec[1].cyan().to_string();
                ascii_vec[2] = ascii_vec[2].cyan().to_string();
                ascii_vec[3] = ascii_vec[3].cyan().to_string();
                ascii_vec[4] = ascii_vec[4].cyan().to_string();
                ascii_vec[5] = ascii_vec[5].cyan().to_string();
            } else if x == "black" {
                ascii_vec[0] = ascii_vec[0].black().to_string();
                ascii_vec[1] = ascii_vec[1].black().to_string();
                ascii_vec[2] = ascii_vec[2].black().to_string();
                ascii_vec[3] = ascii_vec[3].black().to_string();
                ascii_vec[4] = ascii_vec[4].black().to_string();
                ascii_vec[5] = ascii_vec[5].black().to_string();
            } else if x == "white" {
                ascii_vec[0] = ascii_vec[0].white().to_string();
                ascii_vec[1] = ascii_vec[1].white().to_string();
                ascii_vec[2] = ascii_vec[2].white().to_string();
                ascii_vec[3] = ascii_vec[3].white().to_string();
                ascii_vec[4] = ascii_vec[4].white().to_string();
                ascii_vec[5] = ascii_vec[5].white().to_string();
            } else if x == "green" {
                ascii_vec[0] = ascii_vec[0].green().to_string();
                ascii_vec[1] = ascii_vec[1].green().to_string();
                ascii_vec[2] = ascii_vec[2].green().to_string();
                ascii_vec[3] = ascii_vec[3].green().to_string();
                ascii_vec[4] = ascii_vec[4].green().to_string();
                ascii_vec[5] = ascii_vec[5].green().to_string();
            } else {
                ascii_vec[0] = ascii_vec[0].red().to_string();
                ascii_vec[1] = ascii_vec[1].red().to_string();
                ascii_vec[2] = ascii_vec[2].red().to_string();
                ascii_vec[3] = ascii_vec[3].red().to_string();
                ascii_vec[4] = ascii_vec[4].red().to_string();
                ascii_vec[5] = ascii_vec[5].red().to_string();
            }
        },
        None => {
            ascii_vec[0] = ascii_vec[0].red().to_string();
            ascii_vec[1] = ascii_vec[1].red().to_string();
            ascii_vec[2] = ascii_vec[2].red().to_string();
            ascii_vec[3] = ascii_vec[3].red().to_string();
            ascii_vec[4] = ascii_vec[4].red().to_string();
            ascii_vec[5] = ascii_vec[5].red().to_string();
        }
    }

    match args.bottom_color {
        Some(x) => {
            if x == "blue" {
                ascii_vec[6] = ascii_vec[6].blue().to_string();
                ascii_vec[7] = ascii_vec[7].blue().to_string();
                ascii_vec[8] = ascii_vec[8].blue().to_string();
            } else if x == "yellow" {
                ascii_vec[6] = ascii_vec[6].yellow().to_string();
                ascii_vec[7] = ascii_vec[7].yellow().to_string();
                ascii_vec[8] = ascii_vec[8].yellow().to_string();
            } else if x == "magenta" {
                ascii_vec[6] = ascii_vec[6].magenta().to_string();
                ascii_vec[7] = ascii_vec[7].magenta().to_string();
                ascii_vec[8] = ascii_vec[8].magenta().to_string();
            } else if x == "cyan" {
                ascii_vec[6] = ascii_vec[6].cyan().to_string();
                ascii_vec[7] = ascii_vec[7].cyan().to_string();
                ascii_vec[8] = ascii_vec[8].cyan().to_string();
            } else if x == "black" {
                ascii_vec[6] = ascii_vec[6].black().to_string();
                ascii_vec[7] = ascii_vec[7].black().to_string();
                ascii_vec[8] = ascii_vec[8].black().to_string();
            } else if x == "white" {
                ascii_vec[6] = ascii_vec[6].white().to_string();
                ascii_vec[7] = ascii_vec[7].white().to_string();
                ascii_vec[8] = ascii_vec[8].white().to_string();
            }else if x == "green" {
                ascii_vec[6] = ascii_vec[6].green().to_string();
                ascii_vec[7] = ascii_vec[7].green().to_string();
                ascii_vec[8] = ascii_vec[8].green().to_string();
            } else {
                ascii_vec[6] = ascii_vec[6].red().to_string();
                ascii_vec[7] = ascii_vec[7].red().to_string();
                ascii_vec[8] = ascii_vec[8].red().to_string();
            }
        },
        None => {
            ascii_vec[6] = ascii_vec[6].blue().to_string();
            ascii_vec[7] = ascii_vec[7].blue().to_string();
            ascii_vec[8] = ascii_vec[8].blue().to_string();
        }
    }

    let mut box_side = "│".to_string();
    let mut box_top = "─".to_string().repeat(final_length);
    let mut box_top_left_corner = "╭".to_string();
    let mut box_top_right_corner = "╮".to_string();
    let mut box_bottom_left_corner = "╰".to_string();
    let mut box_bottom_right_corner = "╯".to_string();

    match args.outer_box_color {
        Some(x) => {
            if x == "blue" {
                box_side                = box_side.blue().to_string();
                box_top                 = box_top.blue().to_string();
                box_top_left_corner     = box_top_left_corner.blue().to_string();
                box_top_right_corner    = box_top_right_corner.blue().to_string();
                box_bottom_left_corner  = box_bottom_left_corner.blue().to_string();
                box_bottom_right_corner = box_bottom_right_corner.blue().to_string();
            } else if x == "yellow" {
                box_side                = box_side.yellow().to_string();
                box_top                 = box_top.yellow().to_string();
                box_top_left_corner     = box_top_left_corner.yellow().to_string();
                box_top_right_corner    = box_top_right_corner.yellow().to_string();
                box_bottom_left_corner  = box_bottom_left_corner.yellow().to_string();
                box_bottom_right_corner = box_bottom_right_corner.yellow().to_string();
            } else if x == "magenta" {
                box_side                = box_side.magenta().to_string();
                box_top                 = box_top.magenta().to_string();
                box_top_left_corner     = box_top_left_corner.magenta().to_string();
                box_top_right_corner    = box_top_right_corner.magenta().to_string();
                box_bottom_left_corner  = box_bottom_left_corner.magenta().to_string();
                box_bottom_right_corner = box_bottom_right_corner.magenta().to_string();
            } else if x == "cyan" {
                box_side                = box_side.cyan().to_string();
                box_top                 = box_top.cyan().to_string();
                box_top_left_corner     = box_top_left_corner.cyan().to_string();
                box_top_right_corner    = box_top_right_corner.cyan().to_string();
                box_bottom_left_corner  = box_bottom_left_corner.cyan().to_string();
                box_bottom_right_corner = box_bottom_right_corner.cyan().to_string();
            } else if x == "black" {
                box_side                = box_side.black().to_string();
                box_top                 = box_top.black().to_string();
                box_top_left_corner     = box_top_left_corner.black().to_string();
                box_top_right_corner    = box_top_right_corner.black().to_string();
                box_bottom_left_corner  = box_bottom_left_corner.black().to_string();
                box_bottom_right_corner = box_bottom_right_corner.black().to_string();
            } else if x == "white" {
                box_side                = box_side.white().to_string();
                box_top                 = box_top.white().to_string();
                box_top_left_corner     = box_top_left_corner.white().to_string();
                box_top_right_corner    = box_top_right_corner.white().to_string();
                box_bottom_left_corner  = box_bottom_left_corner.white().to_string();
                box_bottom_right_corner = box_bottom_right_corner.white().to_string();
            } else if x == "green" {
                box_side                = box_side.green().to_string();
                box_top                 = box_top.green().to_string();
                box_top_left_corner     = box_top_left_corner.green().to_string();
                box_top_right_corner    = box_top_right_corner.green().to_string();
                box_bottom_left_corner  = box_bottom_left_corner.green().to_string();
                box_bottom_right_corner = box_bottom_right_corner.green().to_string();
            } else {
                box_side                = box_side.red().to_string();
                box_top                 = box_top.red().to_string();
                box_top_left_corner     = box_top_left_corner.red().to_string();
                box_top_right_corner    = box_top_right_corner.red().to_string();
                box_bottom_left_corner  = box_bottom_left_corner.red().to_string();
                box_bottom_right_corner = box_bottom_right_corner.red().to_string();
            }
        },
        None => {
            box_side                = box_side.blue().to_string();
            box_top                 = box_top.blue().to_string();
            box_top_left_corner     = box_top_left_corner.blue().to_string();
            box_top_right_corner    = box_top_right_corner.blue().to_string();
            box_bottom_left_corner  = box_bottom_left_corner.blue().to_string();
            box_bottom_right_corner = box_bottom_right_corner.blue().to_string();
        }
    }


    // The number `14` defines the total characters, upto the output
    // for each system info value. For example,
    // ` OS          ` this will be printed before the name of the OS is
    // printed, and this text contains 14 characters. The `11` and `5` above
    // are chosen based on that formatting.

    println!();
    println!("{} {}{}{}", ascii_vec[0], box_top_left_corner, box_top, box_top_right_corner);
    println!("{} {} {}        {}  {}{}{}", ascii_vec[1], box_side, "OS".to_string().red().bold().italic(), "".to_string().red(), os_name, " ".to_string().repeat(final_length - 14 - string_length_vector[0]), box_side);
    println!("{} {} {}    {}  {}{}{}", ascii_vec[2], box_side, "KERNEL".to_string().magenta().bold().italic(), "".to_string().magenta(), kernel, " ".to_string().repeat(final_length - 14 - string_length_vector[1]), box_side);
    println!("{} {} {}     {}  {}{}{}", ascii_vec[3], box_side, "SHELL".to_string().yellow().bold().italic(), "".to_string().yellow(), shell_name, " ".to_string().repeat(final_length - 14 - string_length_vector[2]), box_side);
    println!("{} {} {}   {}  {}{}{}", ascii_vec[4], box_side, "SESSION".to_string().blue().bold().italic(), "".to_string().blue(), session, " ".to_string().repeat(final_length - 14 - string_length_vector[3]), box_side);
    println!("{} {} {}    {} {}{}{}", ascii_vec[5], box_side, "UPTIME".to_string().cyan().bold().italic(), "祥".to_string().cyan(), uptime, " ".to_string().repeat(final_length - 14 - string_length_vector[4]), box_side);
    println!("{} {} {}  {}  {}{}{}", ascii_vec[6], box_side, "PACKAGES".to_string().green().bold().italic(), "".to_string().green(), total_packages, " ".to_string().repeat(final_length - 14 - string_length_vector[5]), box_side);
    println!("{} {} {}  {}  {}{}{}", ascii_vec[7], box_side, "HOSTNAME".to_string().white().bold().italic(), "".to_string().white(), hostname, " ".to_string().repeat(final_length - 14 - string_length_vector[6]), box_side);
    println!("{}   {}{}{}", ascii_vec[8], box_bottom_left_corner, box_top, box_bottom_right_corner);
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
