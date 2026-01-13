use std::io::{self, Write};
use colored::*;

use LC::tableau::{build_tableau, print_tree, printer::reset_speed, node::NodeStatus};
use LC::parser::parse_formula;
use LC::formula::Formula;
fn main() {

    let logo = r#"
  _______    _      _                  
 |__   __|  | |    | |                 
    | | __ _| |__  | | ___  __ _ _   _ 
    | |/ _` | '_ \ | |/ _ \/ _` | | | |
    | | (_| | |_) || |  __/ (_| | |_| |
    |_|\__,_|_.__(_)_|\___|\__,_|\__,_|
             LOGIC SOLVER v1.0
    "#;

    println!("{}", logo.bold().cyan());


    println!("{}", "=== LOGIC SOLVER (Tabele Semantice) ===".bold().cyan());
    println!("Moduri de utilizare:");
    println!("  1. Verificare simplă:  {}", "(P & Q)".yellow());
    println!("  2. Demonstrație:       {}", "prove (P -> P)".yellow());
    println!("Tastează 'exit' pentru a ieși.\n");

    loop {
        print!("{}", "Solver > ".blue());
        io::stdout().flush().unwrap();

        let mut input_line = String::new();
        if io::stdin().read_line(&mut input_line).expect("Eroare la citire") == 0 {
            break;
        }
        let input = input_line.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }
        if input.is_empty() { continue; }

        let (is_proof_mode, clean_input) = if input.starts_with("prove ") {
            (true, &input[6..])
        } else {
            (false, input)
        };

        println!("{}", "---------------------------------------".dimmed());
        
        let parsed_formula = parse_formula(clean_input);
        
        if is_proof_mode {
            println!("Mod: {}", "Demonstrație Tautologie".bold().purple());
            println!("Ipoteză: Formula {} este mereu adevărată.", input.bold());
            println!("Metodă: Încercăm să demonstrăm că Negația este o Contradicție.\n");

            let negated_formula = Formula::not(parsed_formula.clone());
            println!("Formula Negată: {}", negated_formula.to_string().bold().yellow());

            let root = build_tableau(vec![negated_formula]);
            reset_speed();
            print_tree(&root, "".to_string(), true);

            println!("{}", "---------------------------------------".dimmed());
            match root.status {
                NodeStatus::Closed => {
                    println!("{}", "REZULTAT: TAUTOLOGIE DEMONSTRATĂ".on_green().white().bold());
                    println!("Toate ramurile negației sunt închise (contradicții).");
                    println!("Deci, formula originală nu poate fi falsă niciodată.");
                },
                _ => {
                    println!("{}", "REZULTAT: NU ESTE TAUTOLOGIE".on_red().white().bold());
                    println!("Arborele negației are ramuri deschise.");
                    println!("Am găsit un contra-exemplu (o situație în care formula e falsă).");
                }
            }

        } else {
            println!("Mod: {}", "Verificare Satisfiabilitate".bold().blue());
            println!("Formula Parsată: {}", parsed_formula.to_string().bold().yellow());
            
            let root = build_tableau(vec![parsed_formula]);
            reset_speed();
            print_tree(&root, "".to_string(), true);
            
            println!("{}", "---------------------------------------".dimmed());
            if let NodeStatus::Closed = root.status {
                println!("{}", "Formulă Nesatisfiabilă (Contradicție)".red());
            } else {
                println!("{}", "Formulă Satisfiabilă (Există soluții)".green());
            }
        }
        println!("\n");
    }
}
