mod copy_handler;

use std::env;
use std::fs;
use std::process;

fn print_help() {
    println!("fclipperr: Universal Clipboard Utility");
    println!();
    println!("USAGE:");
    println!("    fclipperr <filename>");
    println!();
    println!("DESCRIPTION:");
    println!("    Copies the contents of the specified file to the clipboard.");
    println!("    Automatically detects local vs remote (SSH) environments and");
    println!("    uses the appropriate clipboard mechanism.");
    println!();
    println!("OPTIONS:");
    println!("    --help, -h    Display this help message");
    println!();
    println!("EXIT CODES:");
    println!("    0    Success (content copied)");
    println!("    1    Failure (file not found, permission denied, clipboard error)");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No filename provided");
        eprintln!();
        print_help();
        process::exit(1);
    }

    let arg = &args[1];

    if arg == "--help" || arg == "-h" {
        print_help();
        process::exit(0);
    }

    let filepath = arg;

    let content = match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filepath, e);
            process::exit(1);
        }
    };

    let strategy = copy_handler::detect_environment();

    if let Err(e) = copy_handler::copy_content(&content, strategy) {
        eprintln!("Error copying to clipboard: {}", e);
        process::exit(1);
    }

    process::exit(0);
}
