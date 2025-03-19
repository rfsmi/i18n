fn cost(msg: &str) -> Option<u32> {
    match (msg.chars().count(), msg.bytes().len()) {
        (..=140, ..=160) => Some(13), // Both tweet and sms
        (_, ..=160) => Some(11),      // Only sms
        (..=140, _) => Some(7),       // Only tweet
        _ => None,                    // Neither
    }
}

pub fn solve(input: &str) -> u32 {
    input.lines().filter_map(cost).sum()
}
