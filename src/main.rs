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

// fn setup_logging(logfile_path: Option<PathBuf>) {
//
//     if let Some(logfile) = logfile_path {
//
//         let file = OpenOptions::new().write(true).append(true).create(true).open(logfile).expect("Cannot open log file");
//         CombinedLogger::init(vec![
//             TermLogger::new(
//                 LevelFilter::Info,
//                 config.clone(),
//                 TerminalMode::Mixed,
//                 ColorChoice::Auto,
//             ),
//             WriteLogger::new(
//                 LevelFilter::Info,
//                 config.clone(),
//                 //File::create(logfile).unwrap(),
//                 file,
//             ),
//         ])
//         .unwrap();
//     } else {
//         TermLogger::init(
//             LevelFilter::Warn,
//             config,
//             TerminalMode::Mixed,
//             ColorChoice::Auto,
//         )
//         .unwrap();
//     };
//
// }
//

// fn get_logger(term_logger: Option<&TermLogger>, write_logger: Option<&WriteLogger<&str>>) -> Result<(), SetLoggerError> {
//     let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = Vec::new();
//
//     if let Some(term_logger_conf) = term_logger {
//         loggers.push(term_logger_conf)
//     }
//     if let Some(write_logger_conf) = write_logger {
//         loggers.push(write_logger_conf)
//     }
//
//     CombinedLogger::init(loggers)
// }

// fn get_terminal_logger(config: simplelog::Config ) -> Box<simplelog::TermLogger> {
//     TermLogger::new(
//         LevelFilter::Info,
//         config.clone(),
//         TerminalMode::Mixed,
//         ColorChoice::Auto,
//     )
// }

// fn get_write_logger(config: simplelog::Config, logfile: &PathBuf  ) -> Box<simplelog::WriteLogger> {
//
//     let file = OpenOptions::new().write(true).append(true).create(true).open(logfile).expect("Cannot open log file");
//     WriteLogger::new(
//         LevelFilter::Info,
//         config.clone(),
//         //File::create(logfile).unwrap(),
//         file,
//     )
// }
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
    //process::exit(42);
    // if !&output_obj.status.success() {
    //     if let Some(logfile) = cli.logfile_path {
    //        // both logfile and stderr /stdout
    //         CombinedLogger::init(vec![term_logger, write_logger]);
    //     } else {
    //        // only stderr and stdout
    //         CombinedLogger::init(vec![term_logger]);
    //     }
    // } else {
    //     (!output_obj.stderr.is_empty() && cli.stderr) {
    //
    // }
    //
    //

    //         if !&output_obj.status.success()  {
    //             println!("output to stdout/stderr and if chosen the logfile");
    //             if let Some(logfile) = cli.logfile_path {
    //                 println!("output to logfile and stderr");
    // //                setup_logging(&logfile);
    //             }
    //
    //             log::info!("rcronic detected error in command `{}`", &command);
    //             print_stdout(&output_obj.stdout);
    //             print_stderr(&output_obj.stderr);
    //             log::error!("Return code: {}", &output_obj.status);
    //         } else {
    //             if let Some(logfile) = cli.logfile_path {
    //                 println!("output to logfile only");
    //             };
    //
    //         }

    // let config = ConfigBuilder::new()
    //     .set_level_color(Level::Error, Some(Color::Magenta))
    //     .set_level_color(Level::Trace, Some(Color::Green))
    //     .set_time_format_rfc3339()
    //     .set_time_offset_to_local()
    //     .unwrap()
    //     .build();
    // if let Some(logfile) = cli.logfile_path {
    //
    //     let file = OpenOptions::new().write(true).append(true).create(true).open(logfile).expect("Cannot open log file");
    //     CombinedLogger::init(vec![
    //         TermLogger::new(
    //             LevelFilter::Info,
    //             config.clone(),
    //             TerminalMode::Mixed,
    //             ColorChoice::Auto,
    //         ),
    //         WriteLogger::new(
    //             LevelFilter::Info,
    //             config.clone(),
    //             //File::create(logfile).unwrap(),
    //             file,
    //         ),
    //     ])
    //     .unwrap();
    // } else {
    //     TermLogger::init(
    //         LevelFilter::Warn,
    //         config,
    //         TerminalMode::Mixed,
    //         ColorChoice::Auto,
    //     )
    //     .unwrap();
    // };

    // if let Some(command) = cli.command {
    //     let output_obj = cmd!("sh", "-c", &command)
    //         .stderr_capture()
    //         .stdout_capture()
    //         .unchecked()
    //         .run()?;
    //
    //     if !&output_obj.status.success() || (!output_obj.stderr.is_empty() && cli.stderr) {
    //         log::info!("rcronic detected error in command `{}`", &command);
    //         print_stdout(&output_obj.stdout);
    //         print_stderr(&output_obj.stderr);
    //         if output_obj.status.success() {
    //             log::info!("Return code: {}", &output_obj.status);
    //         } else {
    //             log::error!("Return code: {}", &output_obj.status);
    //             process::exit(1);
    //         }
    //         // } else {
    //         //     log::info!("Return code: {}", &output_obj.status);
    //         //
    //     } else {
    //
    //         print_stdout(&output_obj.stdout);
    //     }
    // }

    Ok(())
}
