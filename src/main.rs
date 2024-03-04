use std::fs::OpenOptions;
use std::io::Error;
use std::path::PathBuf;

use clap::{Parser, ValueHint};
use duct::cmd;
use simplelog::*;

#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[clap(short = 'l', long = "logfile", help = "write to logfile", value_hint = ValueHint::FilePath)]
    logfile_path: Option<PathBuf>,

    #[clap(short = 'c', long = "command", help = "Command to execute", value_hint = ValueHint::FilePath)]
    command: String,

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

    let config = ConfigBuilder::new()
        .set_level_color(Level::Error, Some(Color::Magenta))
        .set_level_color(Level::Trace, Some(Color::Green))
        .set_time_format_rfc3339()
        .set_time_offset_to_local()
        .unwrap()
        .build();

    let output_obj = cmd!("sh", "-c", &cli.command)
        .stderr_capture()
        .stdout_capture()
        .unchecked()
        .run()?;

    // - command had error
    // - or there is stderr output and we requested -e
    // Then we want output to terminal and to logger if logging was requested
    match (
        &output_obj.status.success(),
        cli.logfile_path,
        cli.stderr,
        output_obj.stderr.is_empty(),
    ) {
        (true, None, false, _) => (), //println!("print no output at all"),

        (false, Some(logfile), _, _) | (true, Some(logfile), true, _) => {
            //println!("output stderr/stdout to terminal and logfile");
            log::info!("rcronic detected error in command `{}`", &cli.command);
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(logfile)
                .expect("Cannot open log file");
            CombinedLogger::init(vec![
                TermLogger::new(
                    LevelFilter::Info,
                    config.clone(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                ),
                WriteLogger::new(
                    LevelFilter::Info,
                    config.clone(),
                    //File::create(logfile).unwrap(),
                    file,
                ),
            ])
            .unwrap();
        }

        (true, None, true, _) | (false, None, _, _) => {
            //println!("output stderr/stdour to terminal");

            TermLogger::init(
                LevelFilter::Warn,
                config,
                TerminalMode::Mixed,
                ColorChoice::Auto,
            )
            .unwrap();
        }
        (true, Some(logfile), false, _) => {
            //println!("output stderr/stdour to logfile");
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(logfile)
                .expect("Cannot open log file");
            WriteLogger::init(
                LevelFilter::Info,
                config,
                //File::create(logfile).unwrap(),
                file,
            )
            .unwrap();
        }
    }

    print_stdout(&output_obj.stdout);
    print_stderr(&output_obj.stderr);

    Ok(())
}
