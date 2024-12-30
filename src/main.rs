use parsing_dates::parse_date_bad;
use parsing_dates::Date;

fn main() {
    let dates = [
        "2024-02-31",
        "64000-6-15",
        "01985-04-05",
        "",
        "3615-05",
        "2439-xa-15",
        "1973-2-3",
    ];
    for date in dates {
        let date_good = date.parse::<Date>();
        let date_bad = parse_date_bad(date);
        println!("Date: {:?}", date);
        if let Ok(date_good) = date_good {
            println!("\t Good parsing: {:?}", date_good);
        } else {
            println!("\t Good parsing: {}", date_good.unwrap_err());
        }
        println!("\t Bad parsing: {:?}", date_bad);
    }
}
