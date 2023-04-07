use std::fs::{File, OpenOptions};
use std::io::{Error, Write}; // bring trait into scope
use std::path::PathBuf;

use clap::{Parser, ValueHint};
use run_script::ScriptOptions;

#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short = 'l', long = "logfile", help = "write to logfile", value_hint = ValueHint::FilePath)]
    logfile_path: Option<PathBuf>,

    #[clap(short = 'c', long = "command", help = "Command to execute", value_hint = ValueHint::FilePath)]
    command: Option<String>,

    #[clap(short = 'e', long = "stderr", help = "Trigger output when stderr is not empty")]
    stderr: bool,
}

//fn write_logfile(logfile_path: Option<PathBuf>, stdout: &[u8], stderr: &[u8]) {
fn write_logfile(logfile_path: Option<PathBuf>, stdout: &str, stderr: &str) -> Result<(), Error> {
    if let Some(logfile) = logfile_path.as_deref() {
        // println!("stdout: {:?}", stdout);
        // println!("stderr: {:?}", stderr);
//        let mut f = File::create(logfile).expect("Unable to create file");

        // Open a file with append option
        let mut f = if logfile.exists() {
            OpenOptions::new()
                .append(true)
                .open(logfile)
                .expect("Cannot open logfile")
        } else {
            File::create(logfile).expect("Unable to create file")
        };

        write!(f, "{}", stdout)?;
        write!(f, "{}", stderr)?;
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let options = ScriptOptions::new();
    let args = vec![];

    if let Some(command) = cli.command {
        let (code, output, error) = run_script::run(&command, &args, &options).unwrap();

        write_logfile(cli.logfile_path, &output, &error)?;
        if code != 0 || (cli.stderr && !error.is_empty()) {
            println!("Exit Code: {}", code);
            //println!("Output: {}", output);
            println!("Error: {}", error);
        }
    }

    // if let Some(command) = cli.command {
    //     match cmd!(sh, "{command}").output() {
    //         Ok(res) => {
    //             println!("ok");
    //             println!("res: {:?}", res);
    //             write_logfile(cli.logfile_path, &res.stdout, &res.stderr);
    //         },
    //         Err(e) => println!("Error: {:?}", e)
    //     }

    // }
    // // println!("out: {:?}", &out);
    // // println!("out: {:?}", out.to_string());
    Ok(())
}
