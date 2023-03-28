use std::io::Write;
use termcolor::{StandardStream, Color, ColorSpec, WriteColor};
use crate::lib::ReportLine;

pub fn print_report_line(stream: &mut StandardStream, line: &ReportLine) -> Result<(), std::io::Error> {
    let prefix = match line.line_type {
        crate::lib::ReportLineType::Day   => "D",
        crate::lib::ReportLineType::Week  => "W",
        crate::lib::ReportLineType::Month => "M",
    };
    
    stream.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    write!(stream, "{}", prefix)?;
    stream.reset()?;
    write!(stream, " {:>15} ", line.label)?;

    let no_diff = (line.hours - line.expected).abs() < f64::EPSILON;

    let diff = if no_diff { 
        "".to_owned()
    } else {
        let diff = line.hours - line.expected;
        if diff > 0.0 {
            stream.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        } else {
            stream.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        }
        format!("{:>+6.2}", diff)
    };
    write!(stream, "{:>6}", diff)?;
    stream.reset()?;
    write!(stream, " ( ")?;
    if no_diff {
        stream.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    } else if line.hours < line.expected {
        stream.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    } else {
        stream.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    }

    write!(stream, "{:>6.2}", line.hours)?;
    stream.reset()?;
    writeln!(stream, " / {:>3} ) ", line.expected)?;
    Ok(())
}
