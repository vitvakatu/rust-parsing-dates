use parsing_dates::parse_date_good;
use parsing_dates::parse_date_bad;

fn main() {
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
        let date_bad = parse_date_bad(date);
        println!("Date: {:?}", date);
        println!("\t Good parsing: {:?}", date_good);
        println!("\t Bad parsing: {:?}", date_bad);
    }
}