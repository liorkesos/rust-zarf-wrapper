use clap::{Arg, ArgMatches, Command};
use std::env;
use std::process::{self, Command as ProcessCommand, Stdio};

fn main() {
    let matches = Command::new("mycli")
        .version("1.0.0")
        .author("Your Name <your.email@example.com>")
        .about("A CLI tool with Zarf integration")
        .subcommand_negates_reqs(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("zarf")
                .about("Run Zarf commands")
                .allow_external_subcommands(true)
                .arg(
                    Arg::new("args")
                        .help("Arguments to pass to Zarf")
                        .num_args(0..)
                        .trailing_var_arg(true)
                        .allow_hyphen_values(true)
                )
        )
        .subcommand(
            Command::new("version")
                .about("Show version information")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("zarf", sub_matches)) => {
            handle_zarf_command(sub_matches);
        }
        Some(("version", _)) => {
            println!("mycli version {}", env!("CARGO_PKG_VERSION"));
            println!("Zarf version:");
            run_zarf_command(&["version"]);
        }
        Some((external, sub_matches)) => {
            // Handle other external subcommands if needed
            eprintln!("Unknown subcommand: {}", external);
            process::exit(1);
        }
        None => {
            // No subcommand provided, show help
            let mut cmd = Command::new("mycli")
                .version("1.0.0")
                .author("Your Name <your.email@example.com>")
                .about("A CLI tool with Zarf integration")
                .subcommand_negates_reqs(true)
                .allow_external_subcommands(true)
                .subcommand(
                    Command::new("zarf")
                        .about("Run Zarf commands")
                        .allow_external_subcommands(true)
                )
                .subcommand(
                    Command::new("version")
                        .about("Show version information")
                );
            
            cmd.print_help().unwrap();
            println!();
            process::exit(1);
        }
    }
}

fn handle_zarf_command(matches: &ArgMatches) {
    let args: Vec<&str> = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();

    if args.is_empty() {
        // No arguments provided to zarf, show zarf help
        run_zarf_command(&["--help"]);
    } else {
        run_zarf_command(&args);
    }
}

fn run_zarf_command(args: &[&str]) {
    let zarf_path = find_zarf_executable()
        .unwrap_or_else(|| {
            eprintln!("Error: 'zarf' command not found in PATH");
            eprintln!("Please ensure Zarf is installed and available in your PATH");
            process::exit(1);
        });

    let mut cmd = ProcessCommand::new(zarf_path);
    cmd.args(args);
    
    // Pass through stdin/stdout/stderr
    cmd.stdin(Stdio::inherit())
       .stdout(Stdio::inherit())
       .stderr(Stdio::inherit());

    match cmd.status() {
        Ok(status) => {
            if let Some(code) = status.code() {
                process::exit(code);
            } else {
                // Process was terminated by signal
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute zarf: {}", e);
            process::exit(1);
        }
    }
}

fn find_zarf_executable() -> Option<String> {
    // First try to find 'zarf' in PATH
    if let Ok(output) = ProcessCommand::new("which")
        .arg("zarf")
        .output()
    {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(path);
            }
        }
    }

    // Alternative approach: check if 'zarf' command exists by trying to run it
    if ProcessCommand::new("zarf")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        return Some("zarf".to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_zarf_executable() {
        // This test will pass if zarf is installed, otherwise it will verify the function returns None
        let result = find_zarf_executable();
        // We can't assert a specific value since it depends on the system
        // but we can verify the function doesn't panic
        println!("Zarf executable search result: {:?}", result);
    }
}
