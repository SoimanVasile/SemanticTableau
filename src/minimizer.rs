use std::collections::{HashSet};
use colored::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Term {
    pub bits: Vec<i8>, // 0, 1, sau -1 (pentru "don't care")
    pub used: bool,    // Folosit în simplificare?
}

impl Term {
    pub fn new(raw_bits: &[u8]) -> Self {
        Term {
            bits: raw_bits.iter().map(|&b| b as i8).collect(),
            used: false,
        }
    }

    pub fn combine(&self, other: &Term) -> Option<Term> {
        let mut diff_count = 0;
        let mut new_bits = self.bits.clone();

        for i in 0..self.bits.len() {
            if self.bits[i] != other.bits[i] {
                diff_count += 1;
                new_bits[i] = -1; // Devine '-'
            }
        }

        if diff_count == 1 {
            Some(Term { bits: new_bits, used: false })
        } else {
            None
        }
    }

    // Formatează termenul frumos (ex: A!B)
    pub fn to_string(&self, var_names: &[String]) -> String {
        let mut s = String::new();
        let mut first = true;
        for (i, &b) in self.bits.iter().enumerate() {
            if b != -1 { // Dacă nu e "don't care"
                if !first { s.push_str(""); } // Spațiator opțional
                if b == 0 { s.push('!'); }
                s.push_str(&var_names[i]);
                first = false;
            }
        }
        if s.is_empty() { "1 (TRUE)".to_string() } else { s }
    }
}

pub struct BooleanFunction {
    pub var_names: Vec<String>,
    pub minterms: Vec<Vec<u8>>,
}

impl BooleanFunction {
    pub fn new(names: Vec<String>, minterms: Vec<Vec<u8>>) -> Self {
        BooleanFunction { var_names: names, minterms }
    }

    // Afișează Diagrama Veitch (doar pt 3 variabile)
    pub fn print_veitch(&self) {
        if self.var_names.len() != 3 {
            println!("Diagrama e disponibilă doar pentru 3 variabile momentan.");
            return;
        }

        // Convertim mintermii în set de indici pentru verificare rapidă O(1)
        let minterm_indices: HashSet<usize> = self.minterms.iter().map(|bits| {
            let mut idx = 0;
            for &b in bits { idx = (idx << 1) | (b as usize); }
            idx
        }).collect();

        let x = &self.var_names[0];
        let y = &self.var_names[1];
        let z = &self.var_names[2];

        println!("\n=== DIAGRAMA VEITCH ({}, {}, {}) ===", x, y, z);
        println!("      {0}{1}   {0}{2}   {1}{2}   {1}{0} ({1} {2})", "!".dimmed(), y, z);
        println!("     +----+----+----+----+");

        // Gray code logic manual pentru Veitch
        let col_map = [0, 1, 3, 2]; // 00, 01, 11, 10

        for r in 0..2 {
            let prefix = if r == 0 { format!("!{}", x) } else { format!(" {} ", x) };
            print!("{} |", prefix);
            for c in 0..4 {
                let real_c = col_map[c];
                let idx = (r << 2) | real_c;
                if minterm_indices.contains(&idx) {
                    print!(" {}  |", "1".green().bold());
                } else {
                    print!(" {}  |", "0".dimmed());
                }
            }
            println!("\n     +----+----+----+----+");
        }
    }

    // Implementarea QUINE-MCCLUSKEY (Simplified)
    pub fn simplify(&self) {
        if self.minterms.is_empty() {
            println!("Funcția este mereu 0 (Fals).");
            return;
        }

        // 1. Inițializare termeni
        let mut terms: Vec<Term> = self.minterms.iter()
            .map(|bits| Term::new(bits))
            .collect();

        let mut prime_implicants = Vec::new();

        // 2. Bucla de combinare
        loop {
            let mut next_terms = HashSet::new();
            let mut merged = vec![false; terms.len()];
            let mut any_merge = false;

            for i in 0..terms.len() {
                for j in (i + 1)..terms.len() {
                    if let Some(res) = terms[i].combine(&terms[j]) {
                        next_terms.insert(res);
                        merged[i] = true;
                        merged[j] = true;
                        any_merge = true;
                    }
                }
            }

            for (i, t) in terms.iter().enumerate() {
                if !merged[i] {
                    prime_implicants.push(t.clone());
                }
            }

            if !any_merge {
                break;
            }

            terms = next_terms.into_iter().collect();
        }

        let mut unique_primes: HashSet<Term> = HashSet::new();
        for p in prime_implicants {
            unique_primes.insert(p);
        }

        // Afișare
        println!("\n=== REZULTAT SIMPLIFICARE (Prime Implicants) ===");
        let mut result_strings = Vec::new();
        for t in &unique_primes {
            result_strings.push(t.to_string(&self.var_names));
        }
        
        result_strings.sort();
        
        println!("{}", result_strings.join(" v ").bold().cyan());
        println!("{}", "(Acesta este Sum of Prime Implicants)".dimmed());
    }
}
