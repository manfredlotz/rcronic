use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::path::PathBuf;

use clap::{Parser, ValueHint};
use run_script::ScriptOptions;

#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[clap(short = 'l', long = "logfile", help = "write to logfile", value_hint = ValueHint::FilePath)]
    logfile_path: Option<PathBuf>,

    #[clap(short = 'c', long = "command", help = "Command to execute", value_hint = ValueHint::FilePath)]
    command: Option<String>,

    #[clap(short = 'e', long = "stderr", help = "Trigger output when stderr is not empty")]
    stderr: bool,
}

fn write_logfile(logfile_path: Option<PathBuf>, stdout: &str, stderr: &str) -> Result<(), Error> {
    if let Some(logfile) = logfile_path.as_deref() {

        // Open log file with append option or create if not existing
        let mut f = if logfile.exists() {
            OpenOptions::new()
                .append(true)
                .open(logfile)
                .unwrap_or_else(|_| panic!("Cannot open logfile `{}`", logfile.display()))
        } else {
            File::create(logfile).unwrap_or_else(|_| panic!("Unable to create logfile `{}`", logfile.display()))
        };

        write!(f, "{}", stdout)?;
        write!(f, "{}", stderr)?;
    }
    Ok(())
}

fn print_errors(code: i32, output: &str , error: &str) {
    println!("Exit Code: {}", code);
    println!("STDOUT\n{}", output);
    println!("STDERR\n{}", error);
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let options = ScriptOptions::new();
    let args = vec![];

    if let Some(command) = cli.command {
        let (code, output, error) = run_script::run(&command, &args, &options).unwrap();

        write_logfile(cli.logfile_path, &output, &error)?;
        if code != 0 {
            println!("cronic: detected error in command `{}`", command);
            print_errors(code, &output, &error);
        } else if cli.stderr && !error.is_empty() {
            println!("cronic: detected error output in command `{}`", command);
            print_errors(code, &output, &error);
        }
    }

    Ok(())
}
