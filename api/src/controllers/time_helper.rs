use chrono::NaiveDateTime;

pub fn to_naive_datetime(date: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(date, "%Y/%m/%d %H:%M").unwrap()
}
pub fn to_ymdhm_str(date: &NaiveDateTime) -> String {
    date.format("%Y/%m/%d %H:%M").to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_to_naive_datetime() {
        let date = to_naive_datetime("2020/01/01 10:00");
        assert_eq!(
            date,
            NaiveDate::from_ymd_opt(2020, 1, 1)
                .and_then(|x| x.and_hms_opt(10, 0, 0))
                .unwrap()
        );
    }
    #[test]
    fn test_to_ymdhm_str() {
        let date = NaiveDate::from_ymd_opt(2020, 1, 1)
            .and_then(|x| x.and_hms_opt(10, 0, 0))
            .unwrap();
        assert_eq!(to_ymdhm_str(&date), "2020/01/01 10:00");
    }
}
