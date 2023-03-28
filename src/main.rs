use termcolor::{ColorChoice, StandardStream};
use clap::Parser;

mod lib;
mod options;
mod output;


fn main() {
    let opts = options::Options::parse();

    let choice = match opts.color {
        options::Color::Never =>  ColorChoice::Never,
        options::Color::Auto =>   ColorChoice::Auto,
        options::Color::Always => ColorChoice::Always, 
    };
    let mut stdout = StandardStream::stdout(choice);
    
    match lib::generate_report(opts.filename, opts.expected) {
        Ok(report) => {
            for line in report {
                match output::print_report_line(&mut stdout, &line) {
                    Ok(()) => {},
                    Err(e) => eprint!("An IO error occured while trying to print results: {:?}", e),
                }
            }
        }
        Err(err) => match err {
            lib::TimesheetParseError::IOError(e) => eprintln!("IO Error {:?}", e),
            lib::TimesheetParseError::CSVError(e) => eprintln!("Error while parsing incoming csv: {:?}", e),
            lib::TimesheetParseError::DateError(e) => eprintln!("Format of date supplied is not recognized: {:?}", e)
        }
    }
}
