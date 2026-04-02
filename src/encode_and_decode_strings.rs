pub struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        strs.iter()
        .map(|s| format!("{}#{}", s.len(), s))
        .collect()
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut res = Vec::new();
        let mut current_s = &s[..]; // Usamos um slice para evitar percorrer a string do zero

        while !current_s.is_empty() {
            // Encontra a posição do '#'
            if let Some(sep_idx) = current_s.find('#') {
                // Extrai o número antes do '#'
                let len: usize = current_s[..sep_idx].parse().unwrap();

                // Define o início e fim da string real
                let start = sep_idx + 1;
                let end = start + len;

                res.push(current_s[start..end].to_string());

                // "Corta" a parte já processada do slice
                current_s = &current_s[end..];
            }
        }
        res
    }
}
