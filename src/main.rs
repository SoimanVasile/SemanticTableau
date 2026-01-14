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
            "1" => run_tableau(),
            "2" => run_minimizer(),
            "0" => break,
            _ => println!("Opțiune invalidă!"),
        }
    }
}

fn run_tableau() {
    println!("{}", "--- MOD LOGICĂ ---".purple());
    println!("Scrie formula (ex: P & Q) sau 'prove (P -> P)' pentru a demonstra.");
    
    let input = ui::read_line("Logic > ");
    if input.is_empty() { return; }

    // 1. Detectăm dacă utilizatorul vrea demonstrație
    let (is_proof_mode, clean_input) = if input.trim().to_lowercase().starts_with("prove ") {
        (true, &input.trim()[6..]) // Sărim peste "prove "
    } else {
        (false, input.as_str())
    };

    // 2. Parsăm formula
    let parsed = parse_formula(clean_input);

    // 3. Pregătim formula pentru Tableau
    // Dacă e 'prove', NEGĂM formula. Dacă negația e o contradicție, originalul e valid.
    let target_formula = if is_proof_mode {
        println!("Mod: {}", "Demonstrație (Negăm formula pt a căuta contradicția)".purple());
        Formula::not(parsed.clone())
    } else {
        println!("Mod: {}", "Verificare Satisfiabilitate".blue());
        parsed.clone()
    };

    // 4. Construim arborele
    let root = build_tableau(vec![target_formula]);
    print_tree(&root, "".to_string(), true);

    println!("{}", "---------------------------------------".dimmed());

    // 5. Interpretăm rezultatul
    if is_proof_mode {
        // Mod DEMONSTRAȚIE
        match root.status {
            tableau::node::NodeStatus::Closed => {
                println!("{}", "REZULTAT: TAUTOLOGIE (ADEVĂRAT)".on_green().white().bold());
                println!("Negația formulei este o contradicție (toate ramurile închise).");
                println!("Deci formula originală '{}' este mereu adevărată.", clean_input.bold());
            },
            _ => {
                println!("{}", "REZULTAT: NU ESTE TAUTOLOGIE (FALS)".on_red().white().bold());
                println!("Negația formulei are ramuri deschise (există contra-exemple).");
            }
        }
    } else {
        // Mod SATISFIABILITATE
        match root.status {
            tableau::node::NodeStatus::Closed => {
                println!("{}", "Formulă Nesatisfiabilă (Contradicție)".red());
            },
            _ => {
                println!("{}", "Formulă Satisfiabilă (Există soluții)".green());
            }
        }
    }
    println!();
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
