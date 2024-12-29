use std::str::FromStr;

#[derive(Debug)]
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

fn parse_date_good(raw: &str) -> Option<Date> {
    let mut components = raw.split('-');
    let year = components.next()?.parse().ok()?;
    if year < 1970 || year > 9999 {
        return None;
    }
    let month = components.next()?.parse().ok()?;
    if month < 1 || month > 12 {
        return None;
    }
    let day = components.next()?.parse().ok()?;
    if day < 1 || day > days_in_month(month, year) {
        return None;
    }
    let date = Date { year, month, day };
    Some(date)
}

fn parse_date_bad(raw: &str) -> Option<Date> {
    let mut date = Date {
        year: 0,
        month: 0,
        day: 0,
    };
    let mut parts = raw.split('-');
    for idx in 0..3 {
        match (idx, u16::from_str(parts.next()?).ok()?) {
            (0, n @ 1970..) => date.year = n,
            (1, n @ 1..=12) => date.month = n,
            (2, n @ 1..=31) => date.day = n,
            _ => return None,
        }
    }
    parts.next().is_none().then_some(date)
}

fn main() {
    let dates = ["2024-02-31", "64000-6-15"];
    for date in dates {
        let date_good = parse_date_good(date);
        let date_bad = parse_date_bad(date);
        println!("Date: {}", date);
        println!("\t Good parsing: {:?}", date_good);
        println!("\t Bad parsing: {:?}", date_bad);
    }
}
