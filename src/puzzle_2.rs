use itertools::*;
use regex::Regex;

#[derive(Clone, Copy)]
struct Date {
    y: usize,
    m: usize,
    d: usize,
}

#[derive(Clone, Copy)]
struct Datetime {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    second: usize,
}

#[derive(Default)]
struct Calendar {
    days_at_end_of_year: Vec<usize>,
    seconds_at_end_of_month: Vec<usize>,
    seconds_at_end_of_month_leap: Vec<usize>,
}

impl Calendar {
    fn is_leap(year: usize) -> bool {
        // If the year is divisible by four, it's a leap year. But if the year
        // can be divided by 100 as well as four, it's not a leap year. However,
        // if the year is divisible by 400, it is a leap year.
        year % 4 == 0 && year % 100 != 0 || year % 400 == 0
    }

    fn seconds_at_end_of_year(&mut self, year: usize) -> usize {
        if year >= self.days_at_end_of_year.len() {
            for y in self.days_at_end_of_year.len()..year {
                let length = if Calendar::is_leap(y) { 366 } else { 365 };
                let accum = self.days_at_end_of_year.last().copied().unwrap_or(0);
                self.days_at_end_of_year.push(accum + length);
            }
        }
        self.days_at_end_of_year[year] * 24 * 60 * 60
    }
}

// #[derive(Default)]
// struct Calendar {
//     days_at_end_of_year: Vec<u32>,
// }

// impl Calendar {
//     fn normalise(&mut self, date: Date) -> u32 {
//         let is_leap = date.y % 400 == 0 || (date.y % 4 == 0 && date.y % 100 != 0);
//         let month_lengths = [
//             31,
//             if is_leap { 29 } else { 28 },
//             31,
//             30,
//             31,
//             30,
//             31,
//             31,
//             30,
//             31,
//             30,
//             31,
//         ];
//         if date.y > self.days_at_end_of_year.len() {
//             for y in self.days_at_end_of_year.len()..date.y {
//                 let is_leap = y % 400 == 0 || (y % 4 == 0 && y % 100 != 0);
//                 let length = if is_leap { 366 } else { 365 };
//                 let accum = self.days_at_end_of_year.last().copied().unwrap_or(0);
//                 self.days_at_end_of_year.push(accum + length);
//             }
//         }
//         self.days_at_end_of_year[date.y - 1]
//             + (0..date.m).map(|m| month_lengths[m]).sum::<u32>()
//             + date.d as u32
//     }
// }

pub fn solve(input: &str) -> u32 {
    // 2019-06-05T08:15:00-04:00
    // 2019-06-05T08:15:00+04:00
    let mut calendar = Calendar::default();
    let mut times = Vec::new();
    let datetime_re = r"(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})(-+)(\d{2}):(\d{2})";
    let re = Regex::new(datetime_re).unwrap();
    for (_, group) in re.captures_iter(input).map(|c| c.extract()) {
        let [y, m, d, t_h, t_m, _, o_s, o_h, o_m] = group;
        let mut t = t_h.parse::<i32>().unwrap() * 60 + t_m.parse::<i32>().unwrap();
        let offset = o_h.parse::<i32>().unwrap() * 60 + o_m.parse::<i32>().unwrap();
        t -= offset * if o_s == "-" { -1 } else { 1 };
        let mut d = calendar.normalise(Date {
            y: y.parse().unwrap(),
            m: m.parse().unwrap(),
            d: d.parse().unwrap(),
        });
        loop {
            if t >= 24 * 60 {
                d += 1;
                t -= 24 * 60;
            } else if t < 0 {
                d -= 1;
                t += 24 * 60;
            } else {
                break;
            }
        }
        times.push((d, t as u32));
    }
    let counts = times.into_iter().counts();
    println!("{counts:?}");
    let ((d, t), _) = counts.into_iter().filter(|&(_, c)| c >= 4).next().unwrap();
    println!("{d}");
    println!("{t}");
    todo!()
}
