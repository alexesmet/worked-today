use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::ops::AddAssign;
use std::path::PathBuf;

use chrono::{NaiveDate, Datelike, Weekday};
use serde::Deserialize;
use num_traits::FromPrimitive;

pub enum TimesheetParseError {
    IOError(io::Error),
    CSVError(csv::Error),
    DateError(chrono::ParseError),
}

#[derive(Debug, Deserialize)]
struct TimesheetStringRecord {
    date: String,
    hours: f64
}

#[derive(Debug)]
struct TimesheetRecord {
    date: NaiveDate,
    hours: f64
}

impl TimesheetRecord {
    fn from_string_record(record: TimesheetStringRecord) -> Result<TimesheetRecord, chrono::ParseError> {
        let date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d")?;
        Ok(TimesheetRecord { date, hours: record.hours })
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(Eq, Hash, PartialEq)]
pub enum ReportLineType { Day, Week, Month }

//private
struct ReportLineData {
    line_type: ReportLineType,
    hours: f64,
}

#[derive(Debug)]
pub struct ReportLine {
    pub hours: f64,
    pub expected: f64,
    pub label: String,
    pub line_type: ReportLineType,
}

fn test_for_report_period(line_type: &ReportLineType, date_a: NaiveDate, date_b: NaiveDate) -> bool {
    match line_type {
        ReportLineType::Day => date_a == date_b,
        ReportLineType::Week => date_a.week(Weekday::Mon).first_day() == date_b.week(Weekday::Mon).first_day(),
        ReportLineType::Month => date_a.month() == date_b.month(),
    }
}

fn generate_report_label_for_date(line_type: &ReportLineType, date: NaiveDate) -> String {
    match line_type {
        ReportLineType::Day => format!("{:?} {:?}", date, date.weekday()),
        ReportLineType::Week => format!("Week #{}", date.iso_week().week()),
        ReportLineType::Month => chrono::Month::from_u32(date.month()).unwrap().name().to_string(),
    }
}

fn push_result_for_next_period(result_to_push: &mut Vec<ReportLine>, data: &mut ReportLineData, current_day: NaiveDate, expected_hours: f64) {
    result_to_push.push(ReportLine {
        hours: data.hours,
        expected: expected_hours,
        label: generate_report_label_for_date(&data.line_type, current_day),
        line_type: data.line_type,
    });
    data.hours = 0.0;
}

pub fn generate_report(filename: Option<PathBuf>, expected_per_day: f64) -> Result<Vec<ReportLine>, TimesheetParseError> {

    let stdin = io::stdin();
    let handle = stdin.lock();
    let buffered: Box<dyn BufRead> = match filename {
        Some(name) => Box::new(BufReader::new(File::open(name)
            .map_err(TimesheetParseError::IOError)?)),
        None => Box::new(handle),
    };

    let mut report_priority = [
        ReportLineData {
            line_type: ReportLineType::Day,
            hours: 0.0
        },
        ReportLineData {
            line_type: ReportLineType::Week,
            hours: 0.0
        },
        ReportLineData {
            line_type: ReportLineType::Month,
            hours: 0.0
        },
    ];

    let mut accumulated_expected_hours: HashMap<ReportLineType, f64> = HashMap::new();
    
    let mut opt_current_date = None;
    let mut result = Vec::new();
    let mut rdr = csv::Reader::from_reader(buffered);

    for each in report_priority.iter() {
        accumulated_expected_hours.insert(each.line_type, expected_per_day);
    }

    
    for des in rdr.deserialize() {
        let record: TimesheetStringRecord = des.map_err(TimesheetParseError::CSVError)?;
        let record = TimesheetRecord::from_string_record(record).map_err(TimesheetParseError::DateError)?;

        let current_day = opt_current_date.get_or_insert(record.date);

        for each in report_priority.iter_mut() {
            if !test_for_report_period(&each.line_type, *current_day, record.date) {
                push_result_for_next_period(&mut result, each, *current_day, accumulated_expected_hours.remove(&each.line_type).unwrap());
            }
            each.hours += record.hours;
        }

        if *current_day != record.date {
            for each in report_priority.iter() {
                accumulated_expected_hours
                    .entry(each.line_type)
                    .or_insert(0.0)
                    .add_assign(expected_per_day);
            }
        }

        opt_current_date = Some(record.date);

    }

    if let Some(last_day) = opt_current_date {
        for each in report_priority.iter_mut() {
            push_result_for_next_period(&mut result, each, last_day, accumulated_expected_hours.remove(&each.line_type).unwrap());
        }
    }

    Ok(result)
}




