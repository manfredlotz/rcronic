use std::io::Error;
use std::path::PathBuf;
use std::process;

use clap::{Parser, ValueHint};

use duct::cmd;

#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[clap(short = 'l', long = "logfile", help = "write to logfile", value_hint = ValueHint::FilePath)]
    logfile_path: Option<PathBuf>,

    #[clap(short = 'c', long = "command", help = "Command to execute", value_hint = ValueHint::FilePath)]
    command: Option<String>,

    #[clap(
        short = 'e',
        long = "stderr",
        help = "Trigger output when stderr is not empty"
    )]
    stderr: bool,
}

fn print_stdout(output: &[u8]) {
    for line in String::from_utf8_lossy(output).lines() {
        log::info!("{}", line);
    }
}

fn print_stderr(output: &[u8]) {
    for line in String::from_utf8_lossy(output).lines() {
        log::error!("{}", line);
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    if let Some(logfile) = cli.logfile_path {
        // Apply globally
        fern::Dispatch::new()
            // Perform allocation-free log formatting
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {:5}] {}",
                    humantime::format_rfc3339(std::time::SystemTime::now()),
                    record.level(),
                    //                record.target(),
                    message
                ))
            })
            // Add blanket level filter -
            .level(log::LevelFilter::Debug)
            // - and per-module overrides
            .level_for("rcronic", log::LevelFilter::Info)
            // Output to stdout, files, and other Dispatch configurations
            .chain(std::io::stdout())
            .chain(fern::log_file(logfile)?)
            .apply()
            .unwrap();
    } else {
        fern::Dispatch::new()
            // Perform allocation-free log formatting
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {:5}] {}",
                    humantime::format_rfc3339(std::time::SystemTime::now()),
                    record.level(),
                    //                record.target(),
                    message
                ))
            })
            // Add blanket level filter -
            .level(log::LevelFilter::Debug)
            // - and per-module overrides
            .level_for("rcronic", log::LevelFilter::Info)
            // Output to stdout, files, and other Dispatch configurations
            .chain(std::io::stdout())
            .apply()
            .unwrap();
    };

    if let Some(command) = cli.command {
        let output_obj = cmd!("sh", "-c", &command)
            .stderr_capture()
            .stdout_capture()
            .unchecked()
            .run()?;

        if !&output_obj.status.success() || (!output_obj.stderr.is_empty() && cli.stderr) {
            log::info!("rcronic detected error in command `{}`", &command);
            print_stdout(&output_obj.stdout);
            print_stderr(&output_obj.stderr);
            if output_obj.status.success() {
                log::info!("Return code: {}", &output_obj.status);
            } else {
                log::error!("Return code: {}", &output_obj.status);
                process::exit(1);
            }
        // } else {
        //     log::info!("Return code: {}", &output_obj.status);
        //
        }
    }

    Ok(())
}
