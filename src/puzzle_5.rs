use std::collections::HashMap;

use nom::AsChar;

struct Grid {
    grid: HashMap<(i32, i32), char>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut grid = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c.is_space() {
                    continue;
                }
                grid.insert((y as i32, x as i32), c);
            }
        }
        let &(_, x) = grid.keys().max_by_key(|(_, x)| x).unwrap();
        let &(y, _) = grid.keys().max_by_key(|(y, _)| y).unwrap();
        Self {
            grid,
            width: x + 1,
            height: y + 1,
        }
    }

    fn solve(&self) -> usize {
        let (mut y, mut x) = (0, 0);
        let mut result = 0;
        while y < self.height {
            if let Some('💩') = self.grid.get(&(y, x)) {
                result += 1;
            }
            (y, x) = (y + 1, (x + 2) % self.width);
        }
        result
    }
}

pub fn solve(input: &str) -> usize {
    Grid::new(input).solve()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve(
                "\
 ⚘   ⚘ 
  ⸫   ⸫
🌲   💩  
     ⸫⸫
 🐇    💩
⸫    ⸫ 
⚘🌲 ⸫  🌲
⸫    🐕 
  ⚘  ⸫ 
⚘⸫⸫   ⸫
  ⚘⸫   
 💩  ⸫  
     ⸫⸫\
"
            ),
            2
        );
    }
}
