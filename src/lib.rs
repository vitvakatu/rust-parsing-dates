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

#[derive(Debug, PartialEq, Eq)]
pub enum DateParseError {
    YearMissing,
    YearOutOfRange,
    YearNotANumber(std::num::ParseIntError),
    MonthMissing,
    MonthOutOfRange,
    MonthNotANumber(std::num::ParseIntError),
    DayMissing,
    DayOutOfRange,
    DayNotANumber(std::num::ParseIntError),
}

impl std::fmt::Display for DateParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateParseError::YearMissing => write!(f, "Year is missing"),
            DateParseError::YearOutOfRange => write!(f, "Year is out of range"),
            DateParseError::YearNotANumber(e) => write!(f, "Year is not a number: {}", e),
            DateParseError::MonthMissing => write!(f, "Month is missing"),
            DateParseError::MonthOutOfRange => write!(f, "Month is out of range"),
            DateParseError::MonthNotANumber(e) => write!(f, "Month is not a number: {}", e),
            DateParseError::DayMissing => write!(f, "Day is missing"),
            DateParseError::DayOutOfRange => write!(f, "Day is out of range"),
            DateParseError::DayNotANumber(e) => write!(f, "Day is not a number: {}", e),
        }
    }
}

impl std::error::Error for DateParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DateParseError::YearNotANumber(e) => Some(e),
            DateParseError::MonthNotANumber(e) => Some(e),
            DateParseError::DayNotANumber(e) => Some(e),
            _ => None,
        }
    }
}

pub fn parse_date_good(raw: &str) -> Result<Date, DateParseError> {
    let mut components = raw.split('-');
    let year = components
        .next()
        .ok_or(DateParseError::YearMissing)?
        .parse()
        .map_err(DateParseError::YearNotANumber)?;
    if year < 1970 || year > 9999 {
        return Err(DateParseError::YearOutOfRange);
    }
    let month = components
        .next()
        .ok_or(DateParseError::MonthMissing)?
        .parse()
        .map_err(DateParseError::MonthNotANumber)?;
    if month < 1 || month > 12 {
        return Err(DateParseError::MonthOutOfRange);
    }
    let day = components
        .next()
        .ok_or(DateParseError::DayMissing)?
        .parse()
        .map_err(DateParseError::DayNotANumber)?;
    if day < 1 || day > days_in_month(month, year) {
        return Err(DateParseError::DayOutOfRange);
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
