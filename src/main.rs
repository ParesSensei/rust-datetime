fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Duration, NaiveDate};

    #[test]
    fn test_date() {
        let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 3, 30).unwrap();

        println!("{}", date.year());
        println!("{}", date.month());
        println!("{}", date.day());
    }

    #[test]
    fn test_duration() {
        let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 3, 30).unwrap();
        let duration: Duration = Duration::days(10);
        let new_date = date + duration;

        println!("hari saat ini : {}", date);
        println!("10 hari kedepan :{}", new_date);
    }
}