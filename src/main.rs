use bcrypt::{DEFAULT_COST, hash};
use std::env;

fn print_usage(program_name: &str) {
    eprintln!("Usage:");
    eprintln!("  {} [options] <password>", program_name);
    eprintln!("");
    eprintln!("Options:");
    eprintln!(
        "  -n, --rounds <number>    Number of bcrypt rounds (4-31, default: {})",
        DEFAULT_COST
    );
    eprintln!("  -h, --help               Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let mut password = String::new();
    let mut rounds = DEFAULT_COST;
    let mut i = 1;

    // Parse arguments
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage(&program_name);
                return;
            }
            "-n" | "--rounds" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<u32>() {
                        Ok(r) => {
                            if r < 4 || r > 31 {
                                eprintln!(
                                    "Rounds must be between 4 and 31. Using default: {}",
                                    DEFAULT_COST
                                );
                            } else {
                                rounds = r;
                            }
                        }
                        Err(_) => {
                            eprintln!("Invalid rounds value. Using default: {}", DEFAULT_COST);
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("Missing value for rounds parameter");
                    print_usage(&program_name);
                    std::process::exit(1);
                }
            }
            arg if arg.starts_with("-") => {
                eprintln!("Unknown option: {}", arg);
                print_usage(&program_name);
                std::process::exit(1);
            }
            _ => {
                // If it's not an option, it's the password
                password = args[i].clone();
                i += 1;
            }
        }
    }

    // Check if a password was provided
    if password.is_empty() {
        eprintln!("Error: Password is required");
        print_usage(&program_name);
        std::process::exit(1);
    }

    // Hash the password
    match hash(&password, rounds) {
        Ok(hashed) => {
            println!("Password: {}", password);
            println!("Rounds: {}", rounds);
            println!("Hashed: {}", hashed);
        }
        Err(e) => {
            eprintln!("Error hashing password: {}", e);
            std::process::exit(1);
        }
    }
}
