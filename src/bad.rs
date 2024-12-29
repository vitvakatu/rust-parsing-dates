use std::str::FromStr;
use crate::Date;

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