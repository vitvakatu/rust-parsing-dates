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

type Range = std::ops::RangeInclusive<u16>;
#[derive(Debug, Snafu, PartialEq)]
pub enum DateParseError {
    #[snafu(display("Input is empty"))]
    InputIsEmpty,
    #[snafu(display("{}", source))]
    ComponentParseError { source: DateComponentParseError },
}

#[derive(Debug, Snafu, PartialEq)]
pub enum DateComponentParseError {
    #[snafu(display("{} is missing", component_name))]
    ComponentMissing { component_name: String },
    #[snafu(display("{} is not {} digits", component_name, expected_len))]
    ComponentLengthMismatch {
        component_name: String,
        expected_len: usize,
    },
    #[snafu(display("{} is not a number: {}", component_name, source))]
    ComponentNotANumber {
        component_name: String,
        source: std::num::ParseIntError,
    },
    #[snafu(display("{} is out of range: {}-{}", component_name, range.start(), range.end()))]
    ComponentOutOfRange {
        component_name: String,
        range: Range,
    },
}

fn parse_date_component<'a>(
    components: &mut impl Iterator<Item = &'a str>,
    range: Range,
    component_name: &str,
    expected_len: usize,
) -> Result<u16, DateComponentParseError> {
    let component = components
        .next()
        .context(ComponentMissingSnafu { component_name })?;
    ensure!(
        component.len() == expected_len,
        ComponentLengthMismatchSnafu {
            component_name,
            expected_len
        }
    );
    let component = component
        .parse()
        .context(ComponentNotANumberSnafu { component_name })?;
    if !range.contains(&component) {
        return ComponentOutOfRangeSnafu {
            component_name,
            range,
        }
        .fail();
    }
    Ok(component)
}

pub fn parse_date_good(raw: &str) -> Result<Date, DateParseError> {
    ensure!(!raw.is_empty(), InputIsEmptySnafu);
    let mut components = raw.split('-');
    let year = parse_date_component(&mut components, 1970..=9999, "year", 4)
        .context(ComponentParseSnafu)?;
    let month =
        parse_date_component(&mut components, 1..=12, "month", 2).context(ComponentParseSnafu)?;
    let day = parse_date_component(&mut components, 1..=days_in_month(month, year), "day", 2)
        .context(ComponentParseSnafu)?;
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
