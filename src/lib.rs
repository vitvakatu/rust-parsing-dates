use snafu::prelude::*;

mod bad;
pub use bad::parse_date_bad;

#[derive(Debug, PartialEq)]
pub struct Date {
    year: u16,
    month: u16,
    day: u16,
}

fn days_in_month(month: u16, year: u16) -> u16 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }

        _ => unreachable!(),
    }
}

fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[derive(Debug, Snafu, PartialEq)]
pub enum DateParseError {
    #[snafu(display("Year is missing"))]
    YearMissing,
    #[snafu(display("Year is out of range: {}-{}", range.start, range.end))]
    YearOutOfRange { range: std::ops::Range<u16> },
    #[snafu(display("Year is not a number: {}", source))]
    YearNotANumber { source: std::num::ParseIntError },
    #[snafu(display("Month is missing"))]
    MonthMissing,
    #[snafu(display("Month is out of range: {}-{}", range.start, range.end))]
    MonthOutOfRange { range: std::ops::Range<u16> },
    #[snafu(display("Month is not a number: {}", source))]
    MonthNotANumber { source: std::num::ParseIntError },
    #[snafu(display("Day is missing"))]
    DayMissing,
    #[snafu(display("Day is out of range: {}-{}", range.start, range.end))]
    DayOutOfRange { range: std::ops::Range<u16> },
    #[snafu(display("Day is not a number: {}", source))]
    DayNotANumber { source: std::num::ParseIntError },
}


pub fn parse_date_good(raw: &str) -> Result<Date, DateParseError> {
    let mut components = raw.split('-');
    let year = components
        .next()
        .context(YearMissingSnafu)?
        .parse()
        .context(YearNotANumberSnafu)?;
    let allowed_years = 1970..9999;
    if !allowed_years.contains(&year) {
        return YearOutOfRangeSnafu { range: allowed_years }.fail();
    }
    let month = components
        .next()
        .context(MonthMissingSnafu)?
        .parse()
        .context(MonthNotANumberSnafu)?;
    let allowed_months = 1..12;
    if !allowed_months.contains(&month) {
        return MonthOutOfRangeSnafu { range: allowed_months }.fail();
    }
    let day = components
        .next()
        .context(DayMissingSnafu)?
        .parse()
        .context(DayNotANumberSnafu)?;
    let allowed_days = 1..days_in_month(month, year);
    if !allowed_days.contains(&day) {
        return DayOutOfRangeSnafu { range: allowed_days }.fail();
    }
    let date = Date { year, month, day };
    Ok(date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_dates() {
        let dates = [
            "1970-01-01",
            "2024-02-29",
            "9999-12-31",
            "2023-06-15",
            "2000-02-29",
        ];
        for date in dates {
            let expected = Date {
                year: date[0..4].parse().unwrap(),
                month: date[5..7].parse().unwrap(),
                day: date[8..10].parse().unwrap(),
            };
            let date_good = parse_date_good(date);
            assert_eq!(date_good, Ok(expected));
        }
    }

    #[test]
    fn invalid_dates() {
        let dates = [
            "2024-02-31",
            "64000-6-15",
            "01985-04-05",
            "",
            "3615-05",
            "1973-2-3",
        ];
        for date in dates {
            let date_good = parse_date_good(date);
            assert!(date_good.is_err());
        }
    }
}
