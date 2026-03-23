use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // Solução usando contagem de caracteres com um array fixo
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut contagem = [0i32; 26]; // Para as 26 letras do alfabeto

        for (a, b) in s.as_bytes().into_iter().zip(t.as_bytes()) {
            contagem[(a - b'a') as usize] += 1;
            contagem[(b - b'a') as usize] -= 1;
        }

        !contagem.iter().any(|&value| value != 0)
    }

    pub fn is_anagram_hashmap(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counts: HashMap<char, i32> = HashMap::new();

        for (a, b) in s.chars().zip(t.chars()) {
            *counts.entry(a).or_insert(0) += 1;
            *counts.entry(b).or_insert(0) -= 1;
        }

        !counts.values().any(|&value| value != 0)
    }

    pub fn is_anagram_sorting(s: String, t: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();

        s_chars.sort_unstable();
        t_chars.sort_unstable();

        s_chars == t_chars
    }
}
