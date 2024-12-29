use std::str::FromStr;

#[derive(Debug)]
pub struct Date {
    year: u16,
    month: u16,
    day: u16,
}

pub fn parse_date_bad(raw: &str) -> Option<Date> {
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
    let date = parse_date_bad("2024-12-29");
    println!("{:?}", date);
}
