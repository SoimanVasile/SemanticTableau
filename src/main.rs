use std::io::{self, Write};

mod formula;
mod tableau;
mod parser;

use tableau::{build_tableau, print_tree};
use parser::parse_formula;

fn main() {
    println!("=== LOGIC SOLVER (Tabele Semantice) ===");
    println!("Simboluri acceptate:");
    println!("  AND:     &  sau  ^");
    println!("  OR:      |  sau  v");
    println!("  NOT:     !  sau  ~");
    println!("  IMPLIES: ->");
    println!("  Exemplu: (P & Q) -> R");
    println!("Tastează 'exit' pentru a ieși.\n");

    loop {
        print!("Introdu formula > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Eroare la citire");
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }
        if input.is_empty() { continue; }

        println!("---------------------------------------");
        let formula = parse_formula(input);
        println!("Formula Parsată: {:?}", formula);
        
        let root = build_tableau(vec![formula]);
        print_tree(&root, "".to_string(), true);
        println!("---------------------------------------\n");
    }
}
