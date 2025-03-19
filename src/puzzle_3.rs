fn is_valid(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();
    if !matches!(chars.len(), 4..=12) {
        return false;
    }
    if !chars.iter().any(|c| c.is_digit(10)) {
        return false;
    }
    if !chars.iter().any(|c| c.is_uppercase()) {
        return false;
    }
    if !chars.iter().any(|c| c.is_lowercase()) {
        return false;
    }
    if !chars.iter().any(|c| !c.is_ascii()) {
        return false;
    }
    true
}

pub fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(str::trim)
        .filter(|l| is_valid(l))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve(
                "d9Ō
                uwI.E9GvrnWļbzO
                ž-2á
                Ģ952W*F4
                ?O6JQf
                xi~Rťfsa
                r_j4XcHŔB
                71äĜ3"
            ),
            2
        );
    }
}
