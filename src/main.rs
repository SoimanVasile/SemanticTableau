use crate::formula::Formula;
use colored::*;
use std::io::{self};

mod formula;
mod tableau;
mod parser;
mod minimizer;
mod ui;

use tableau::{build_tableau, print_tree};
use parser::parse_formula;
use minimizer::BooleanFunction;

fn main() {
    loop {
        println!("\n{}", "=== LOGIC TOOLKIT ===".on_blue().white().bold());
        println!("1. Solver Logică (Tabele Semantice)");
        println!("2. Minimizare Circuite (Quine-McCluskey)");
        println!("0. Ieșire");
        
        let choice = ui::read_line("\nAlege modul > ");
        
        match choice.as_str() {
            "1" => {run_tableau(); return},
            "2" => run_minimizer(),
            "0" => break,
            _ => println!("Opțiune invalidă!"),
        }
    }
}

fn run_tableau() {
    println!("{}", "--- MOD LOGICĂ (TABLE SEMANTICE) ---".purple().bold());
    
    println!("Instrucțiune: Introdu formula logică pentru analiză.");
    println!("Exemplu valid: {} sau {}", 
        "P & (Q -> P)".yellow(), 
        "prove (P -> P)".yellow()
    );
    
    let input = ui::read_line("Logic > ");
    if input.is_empty() { return; }

    let (is_proof_mode, clean_input) = if input.trim().to_lowercase().starts_with("prove ") {
        (true, &input.trim()[6..])
    } else {
        (false, input.as_str())
    };

    let parsed_formula = parse_formula(clean_input);

    println!("{}", "--------------------------------------------------".dimmed());


    if is_proof_mode {
        println!("Formula Parsată (Interpretată): {}{}", "¬".yellow().bold(), parsed_formula.to_string().yellow().bold());
        println!("\n{}", "=== ETAPA 1: DEFINIREA PROBLEMEI ===".purple().bold());
        println!("{}: Vrem să demonstrăm că formula este o {}","Scop".blue().bold(), "TAUTOLOGIE".green().bold());
        println!("      (adică este adevărată indiferent de valorile variabilelor).");

        let negated_formula = Formula::not(parsed_formula.clone());
        
        println!("\n{}", "=== ETAPA 2: GENERARE ARBORE (Visualizare) ===".purple().bold());
        println!("Formula de lucru (Negată): {}", negated_formula.to_string().yellow().bold());
        println!("\n{}", "Se construiește arborele...".cyan().bold());
        
        let root = build_tableau(vec![negated_formula]);
        print_tree(&root, "".to_string(), true);


        println!("\n{}", "=== ETAPA 3: CONCLUZIE FINALĂ ===".purple().bold());
        println!("{}", "--------------------------------------------------".dimmed());
        
        match root.status {
            tableau::node::NodeStatus::Closed => {
                println!("{}: {}","REZULTAT".blue().bold(), " TAUTOLOGIE DEMONSTRATĂ ".on_green().white().bold());
                println!("{}: Toate ramurile negației s-au închis (au generat contradicții).", "Analiză:".blue().bold());
                println!("{}:   Nu există nicio situație în care formula să fie Falsă.", "Logica".blue().bold());
                println!("{}:  Formula {} este validă logic.", "Verdict".blue().bold(), clean_input.bold());
            },
            _ => {
                println!("{}: {}","REZULTAT".blue().bold(), " NU ESTE TAUTOLOGIE ".on_red().white().bold());
                println!("{}  Arborele negației a rămas cu ramuri deschise.", "Analiză:".blue().bold());
                println!("{}:   Am găsit cel puțin un scenariu (Contra-exemplu) unde negația e Adevărată.", "Logica".blue().bold());
                println!("{}:  Formula originală poate fi falsă.", "Verdict".blue().bold());
            }
        }
    } else {
        // --- MOD SATISFIABILITATE ---
        println!("Formula Parsată (Interpretată): {}", parsed_formula.to_string().yellow().bold());
        println!("\n{}", "=== VERIFICARE SATISFIABILITATE ===".purple().bold());
        println!("{}: Căutăm o combinație de valori (Model) pentru care formula e Adevărată.", "Scop".blue().bold());
        
        let root = build_tableau(vec![parsed_formula]);
        println!("\n{}:", "Arborele generat".cyan().bold());
        print_tree(&root, "".to_string(), true);

        println!("\n{}", "=== CONCLUZIE ===".purple().bold());
        if let tableau::node::NodeStatus::Closed = root.status {
            println!("{}: {}", "REZULTAT".blue().bold(), " CONTRADICȚIE (UNSAT) ".on_red().white().bold());
            println!("Formula nu poate fi adevărată niciodată.");
        } else {
            println!("{}: {}", "REZULTAT".blue().bold(), " SATISFIABILĂ (SAT) ".on_green().white().bold());
            println!("Există ramuri deschise. Orice ramură deschisă reprezintă o soluție posibilă.");
        }
    }
    let mut dummy = String::new();
    std::io::stdin().read_line(&mut dummy).unwrap();

}

fn run_minimizer() {
    println!("{}", "--- MOD CIRCUITE ---".cyan());
    
    // Folosim funcțiile din ui.rs
    let vars = ui::read_variables();
    let num_vars = vars.len();
    let minterms = ui::read_minterms(num_vars);

    if minterms.is_empty() {
        println!("Niciun minterm introdus. Funcția e 0.");
        return;
    }

    // Creăm obiectul și rulăm metodele
    let func = BooleanFunction::new(vars, minterms);
    
    func.print_veitch(); // Arată diagrama (dacă sunt 3 vars)
    func.simplify();     // Face magia Quine-McCluskey
    
    println!("\nApasă Enter pt a continua...");
    let _ = io::stdin().read_line(&mut String::new());
}
