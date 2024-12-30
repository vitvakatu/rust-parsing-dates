use crate::Date;
use std::str::FromStr;

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

pub struct Version {
    pub major: u8,
    pub minor: u8,
}

pub struct VersionParseError;

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split('_').collect::<Vec<_>>().as_slice() {
            ["VK" | "VKSC", "VERSION", major, minor] => {
                let major = major.parse().map_err(|_| VersionParseError)?;
                let minor = minor.parse().map_err(|_| VersionParseError)?;
                Ok(Version { major, minor })
            }
            _ => Err(VersionParseError),
        }
    }
}
