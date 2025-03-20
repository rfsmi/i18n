use encoding::{all::ISO_8859_1, DecoderTrap, Encoding};
use std::collections::VecDeque;

fn decrypt(word: &str) -> String {
    let bytes = ISO_8859_1
        .encode(word, encoding::EncoderTrap::Strict)
        .unwrap();
    String::from_utf8(bytes).unwrap()
}

struct Clue {
    length: usize,
    known_i: usize,
    known_c: char,
}

struct Crossword {
    pool: Vec<Vec<char>>,
    clues: Vec<Clue>,
}

impl Crossword {
    fn new(input: &str) -> Self {
        let mut pool = Vec::new();
        let mut clues = Vec::new();
        let mut lines = input.trim().lines().map(str::trim);

        // Parse the word pool
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mut word = line.to_string();
            if (pool.len() + 1) % 3 == 0 {
                word = decrypt(&word);
            }
            if (pool.len() + 1) % 5 == 0 {
                word = decrypt(&word);
            }
            pool.push(word.chars().collect());
        }

        // Parse the clues
        for line in lines {
            if line.is_empty() {
                break;
            }
            let word: Vec<_> = line.chars().collect();
            let known_i = word.iter().position(|&c| c != '.').unwrap();
            clues.push(Clue {
                known_i,
                known_c: word[known_i],
                length: word.len(),
            });
        }

        Self { pool, clues }
    }

    fn solve(&self) -> usize {
        #[derive(Clone)]
        struct Solution(Vec<usize>);
        let mut queue: VecDeque<Solution> = [Solution(vec![])].into();
        while let Some(sol) = queue.pop_front() {
            if sol.0.len() == self.clues.len() {
                return sol.0.iter().sum::<usize>() + sol.0.len();
            }
            let clue = &self.clues[sol.0.len()];
            for (i, candidate) in self.pool.iter().enumerate() {
                if sol.0.iter().any(|&a| a == i) {
                    continue; // This candidate has already been used
                }
                if candidate.len() == clue.length && candidate[clue.known_i] == clue.known_c {
                    let mut sol = sol.clone();
                    sol.0.push(i);
                    queue.push_back(sol);
                }
            }
        }
        panic!();
    }
}

pub fn solve(input: &str) -> usize {
    Crossword::new(input).solve()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("religiÃ«n"), "religiën");
        assert_eq!(decrypt(&decrypt("pugilarÃÂ£o")), "pugilarão");
    }

    #[test]
    fn test() {
        assert_eq!(
            solve(
                "geléet
                träffs
                religiÃ«n
                tancées
                kÃ¼rst
                roekoeÃ«n
                skälen
                böige
                fÃ¤gnar
                dardÃ©es
                amènent
                orquestrÃ¡
                imputarão
                molières
                pugilarÃÂ£o
                azeitámos
                dagcrème
                zÃ¶ger
                ondulât
                blÃ¶kt

                   ...d...
                    ..e.....
                     .l...
                  ....f.
                ......t.."
            ),
            50
        );
    }
}
