use std::io::{self, Write};
use colored::*;

// Citește o linie de text cu un mesaj în față
pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Eroare la citire");
    input.trim().to_string()
}

// Cere utilizatorului variabilele (ex: "x y z")
pub fn read_variables() -> Vec<String> {
    loop {
        let line = read_line("Introdu variabilele (ex: A B C): ");
        if line.is_empty() { continue; }
        
        let vars: Vec<String> = line.split_whitespace()
            .map(|s| s.to_string())
            .collect();
            
        if vars.is_empty() {
            println!("{}", "Trebuie să introduci cel puțin o variabilă!".red());
            continue;
        }
        return vars;
    }
}

// Cere mintermii (liniile unde funcția e 1)
pub fn read_minterms(num_vars: usize) -> Vec<Vec<u8>> {
    println!("{}", "Introdu valorile pentru care funcția este 1 (ex: 0 0 1).".yellow());
    println!("Scrie '{}' când ai terminat.", "gata".bold());

    let mut minterms = Vec::new();

    loop {
        let prompt = format!("Minterm ({} biți) > ", num_vars);
        let line = read_line(&prompt);

        if line.eq_ignore_ascii_case("gata") {
            break;
        }

        let bits: Vec<u8> = line.split_whitespace()
            .filter_map(|s| s.parse::<u8>().ok())
            .filter(|&b| b == 0 || b == 1) // Doar 0 și 1
            .collect();

        if bits.len() != num_vars {
            println!("{}", format!("Eroare: Trebuie exact {} valori (0 sau 1).", num_vars).red());
            continue;
        }

        minterms.push(bits);
    }
    minterms
}
