use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::Tz;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{multispace0, space0},
    combinator::{fail, map_res},
    multi::many1,
    sequence::preceded,
    IResult, Parser,
};

type DT = DateTime<Utc>;

fn parse(input: &str) -> Vec<(DT, DT)> {
    /*
        Departure: Europe/London                  Mar 04, 2020, 10:00
        Arrival:   Europe/Paris                   Mar 04, 2020, 11:59
    */
    fn tz(input: &str) -> IResult<&str, Tz> {
        map_res(take_while(|c: char| !c.is_whitespace()), str::parse).parse(input)
    }
    fn dt(input: &str) -> IResult<&str, NaiveDateTime> {
        let fmt = "%b %d, %Y, %H:%M";
        match NaiveDateTime::parse_and_remainder(input, fmt) {
            Ok((dt, rem)) => Ok((rem, dt)),
            Err(_) => fail().parse(input),
        }
    }
    fn time(input: &str) -> IResult<&str, DT> {
        (preceded(space0, tz), preceded(space0, dt))
            .map(|(tz, dt)| dt.and_local_timezone(tz).unwrap().to_utc())
            .parse(input)
    }
    let time_pair = (
        preceded((multispace0, tag("Departure:"), space0), time),
        preceded((multispace0, tag("Arrival:"), space0), time),
    );
    many1(time_pair).parse(input).unwrap().1
}

pub fn solve(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|(depart, arrive)| (arrive - depart).num_minutes())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve(
                "Departure: Europe/London                  Mar 04, 2020, 10:00
                Arrival:   Europe/Paris                   Mar 04, 2020, 11:59

                Departure: Europe/Paris                   Mar 05, 2020, 10:42
                Arrival:   Australia/Adelaide             Mar 06, 2020, 16:09

                Departure: Australia/Adelaide             Mar 06, 2020, 19:54
                Arrival:   America/Argentina/Buenos_Aires Mar 06, 2020, 19:10

                Departure: America/Argentina/Buenos_Aires Mar 07, 2020, 06:06
                Arrival:   America/Toronto                Mar 07, 2020, 14:43

                Departure: America/Toronto                Mar 08, 2020, 04:48
                Arrival:   Europe/London                  Mar 08, 2020, 16:52
"
            ),
            3143
        );
    }
}
