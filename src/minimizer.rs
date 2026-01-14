use std::collections::{HashSet};
use colored::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Term {
    pub bits: Vec<i8>,
    pub used: bool,
}

impl Term {
    pub fn new(raw_bits: &[u8]) -> Self {
        Term {
            bits: raw_bits.iter().map(|&b| b as i8).collect(),
            used: false,
        }
    }

    pub fn to_string(&self, var_names: &[String]) -> String {
        let mut s = String::new();
        let mut first = true;
        for (i, &b) in self.bits.iter().enumerate() {
            if b != -1 {
                if !first { s.push_str(""); }
                if b == 0 { s.push('!'); }
                s.push_str(&var_names[i]);
                first = false;
            }
        }
        if s.is_empty() { "1 (TRUE)".to_string() } else { s }
    }

    pub fn combine(&self, other: &Term) -> Option<Term> {
        let mut diff_count = 0;
        let mut new_bits = self.bits.clone();

        for i in 0..self.bits.len() {
            if self.bits[i] != other.bits[i] {
                diff_count += 1;
                new_bits[i] = -1;
            }
        }

        if diff_count == 1 {
            Some(Term { bits: new_bits, used: false })
        } else {
            None
        }
    }
}

pub struct BooleanFunction {
    pub var_names: Vec<String>,
    pub minterms: Vec<Vec<u8>>,
    minterm_indices: HashSet<usize>,
}

impl BooleanFunction {
    pub fn new(names: Vec<String>, minterms: Vec<Vec<u8>>) -> Self {
        let mut indices = HashSet::new();
        for bits in &minterms {
            let mut idx = 0;
            for &b in bits {
                idx = (idx << 1) | (b as usize);
            }
            indices.insert(idx);
        }

        BooleanFunction { var_names: names, minterms, minterm_indices: indices }
    }

    pub fn print_veitch(&self) {
        let n = self.var_names.len();
        match n {
            1 => self.print_1var(),
            2 => self.print_2vars(),
            3 => self.print_3vars(),
            4 => self.print_4vars(),
            5 => self.print_5vars(),
            _ => println!("Diagramele ASCII sunt suportate doar pentru 1-5 variabile."),
        }
    }

    fn get_cell(&self, idx: usize) -> String {
        if self.minterm_indices.contains(&idx) {
            " 1 ".green().bold().to_string()
        } else {
            " 0 ".dimmed().to_string()
        }
    }

    fn print_1var(&self) {
        let a = &self.var_names[0];
        println!("\n=== DIAGRAMA VEITCH ({}) ===", a);
        println!("      !{}   {} ", a, a);
        println!("     +---+---+");
        
        print!("     |");
        print!("{}|", self.get_cell(0));
        print!("{}|", self.get_cell(1));
        println!("\n     +---+---+");
    }

    fn print_2vars(&self) {
        let a = &self.var_names[0];
        let b = &self.var_names[1];
        println!("\n=== DIAGRAMA VEITCH ({}, {}) ===", a, b);
        println!("      !{}   {}  ({})", b, b, b);
        println!("     +---+---+");
        
        println!("!{}   |{}|{}|", a, self.get_cell(0), self.get_cell(1));
        println!("     +---+---+");
        println!(" {}   |{}|{}|", a, self.get_cell(2), self.get_cell(3));
        println!("     +---+---+");
    }

    fn print_3vars(&self) {
        let a = &self.var_names[0];
        let b = &self.var_names[1];
        let c = &self.var_names[2];
        
        let col_map = [0, 1, 3, 2]; 

        println!("\n=== DIAGRAMA VEITCH ({}, {}, {}) ===", a, b, c);
        println!("    !{0}!{1}  !{0}{1}   {0}{1}   {0}!{1} ({0} {1})", b, c);
        println!("     +---+---+---+---+");

        for r in 0..2 {
            let prefix = if r == 0 { format!("!{}", a) } else { format!(" {}", a) };
            print!("{}   |", prefix);
            
            for &c_val in &col_map {
                let idx = (r << 2) | c_val;
                print!("{}|", self.get_cell(idx));
            }
            println!("\n     +---+---+---+---+");
        }
    }

    fn print_4vars(&self) {
        let names = &self.var_names;
        let (a,b,c,d) = (&names[0], &names[1], &names[2], &names[3]);
        
        let gray = [0, 1, 3, 2];

        println!("\n=== DIAGRAMA VEITCH 4 VARS ===");
        println!("(Sus: {}, {} | Stânga: {}, {})", c, d, a, b);
        println!("      !{0}!{1} !{0}{1}  {0}{1} {0}!{1}", c, d);
        println!("      +---+---+---+---+");

        for &r_val in &gray {
            // Header rând
            let r_label = match r_val {
                0 => format!("!{}!{}", a, b),
                1 => format!("!{}{}", a, b),
                3 => format!(" {}{}", a, b),
                2 => format!(" {}!{}", a, b),
                _ => "".to_string()
            };
            print!("{:>5} |", r_label);

            for &c_val in &gray {
                // Index = (AB << 2) | CD
                let idx = (r_val << 2) | c_val;
                print!("{}|", self.get_cell(idx));
            }
            println!("\n      +---+---+---+---+");
        }
    }

    fn print_5vars(&self) {
        let names = &self.var_names;
        let (a,b) = (&names[0], &names[1]);
        
        let row_gray = [0, 1, 3, 2];
        let col_gray = [0, 1, 3, 2, 6, 7, 5, 4];

        println!("\n=== DIAGRAMA VEITCH 5 VARS ===");
        println!("Rânduri: {}{} | Coloane: {}{}{}", a, b, names[2], names[3], names[4]);
        
        println!("       000 001 011 010 110 111 101 100 (Bits: {}{}{})", names[2], names[3], names[4]);
        println!("      +---+---+---+---+---+---+---+---+");

        for &r_val in &row_gray {
             let r_label = match r_val {
                0 => format!("!{}!{}", a, b),
                1 => format!("!{}{}", a, b),
                3 => format!(" {}{}", a, b),
                2 => format!(" {}!{}", a, b),
                _ => "".to_string()
            };
            print!("{:>5} |", r_label);

            for &c_val in &col_gray {
                let idx = (r_val << 3) | c_val;
                print!("{}|", self.get_cell(idx));
            }
             println!("\n      +---+---+---+---+---+---+---+---+");
        }
    }

    pub fn simplify(&self) {
        if self.minterms.is_empty() {
            println!("Funcția este mereu 0 (Fals).");
            return;
        }
        
        let mut terms: Vec<Term> = self.minterms.iter()
            .map(|bits| Term::new(bits))
            .collect();
        let mut prime_implicants = Vec::new();

        loop {
            let mut next_terms = HashSet::new();
            let mut used_indices = HashSet::new();
            let mut any_merge = false;

            for i in 0..terms.len() {
                for j in (i + 1)..terms.len() {
                    if let Some(res) = terms[i].combine(&terms[j]) {
                        next_terms.insert(res);
                        used_indices.insert(i);
                        used_indices.insert(j);
                        any_merge = true;
                    }
                }
            }

            for (i, t) in terms.iter().enumerate() {
                if !used_indices.contains(&i) {
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

        println!("\n=== REZULTAT SIMPLIFICARE ===");
        let mut result_strings = Vec::new();
        for t in &unique_primes {
            result_strings.push(t.to_string(&self.var_names));
        }
        result_strings.sort(); // Sortare alfabetică
        println!("{}", result_strings.join(" v ").bold().cyan());
    }
}
