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

pub fn parse_date_good(raw: &str) -> Result<Date, String> {
    let mut components = raw.split('-');
    let year = components
        .next()
        .ok_or("Year is missing".to_owned())?
        .parse()
        .map_err(|_| "Year is not a number".to_owned())?;
    if year < 1970 || year > 9999 {
        return Err("Year is out of range".to_owned());
    }
    let month = components
        .next()
        .ok_or("Month is missing".to_owned())?
        .parse()
        .map_err(|_| "Month is not a number".to_owned())?;
    if month < 1 || month > 12 {
        return Err("Month is out of range".to_owned());
    }
    let day = components
        .next()
        .ok_or("Day is missing".to_owned())?
        .parse()
        .map_err(|_| "Day is not a number".to_owned())?;
    if day < 1 || day > days_in_month(month, year) {
        return Err("Day is out of range".to_owned());
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
            "2000-02-29"
        ];
        for date in dates {
            let expected = Date {
                year: date[0..4].parse().unwrap(),
                month: date[5..7].parse().unwrap(), 
                day: date[8..10].parse().unwrap()
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
