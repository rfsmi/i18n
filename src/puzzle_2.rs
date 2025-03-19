use chrono::DateTime;
use itertools::Itertools;

const DT_FMT: &'static str = "%Y-%m-%dT%H:%M:%S%:z";

pub fn solve(input: &str) -> String {
    let counts = input
        .trim()
        .lines()
        .filter_map(|l| DateTime::parse_from_str(l, DT_FMT).ok())
        .counts();
    let (time, _) = counts.into_iter().find(|&(_, c)| c >= 4).unwrap();
    time.to_utc().format(DT_FMT).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let t1 = DateTime::parse_from_str("2019-06-05T08:15:00-04:00", DT_FMT).unwrap();
        let t2 = DateTime::parse_from_str("2019-06-05T08:15:00+04:00", DT_FMT).unwrap();
        let t3 = DateTime::parse_from_str("2019-06-05T16:15:00+04:00", DT_FMT).unwrap();
        assert_ne!(t1, t2);
        assert_eq!(t1, t3);
    }
}
