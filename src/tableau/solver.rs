use crate::formula::Formula;
use super::node::{TableauNode, NodeStatus};

pub fn build_tableau(formulas: Vec<Formula>) -> TableauNode {
    for (i, f1) in formulas.iter().enumerate() {
        for f2 in formulas.iter().skip(i + 1) {
            if f1.is_literal() && f2.is_literal() && f1.is_contradiction_with(f2) {
                return TableauNode {
                    formulas,
                    children: vec![],
                    status: NodeStatus::Closed,
                };
            }
        }
    }

    let mut next_formulas = formulas.clone();
    let index_opt = next_formulas.iter().position(|f| !f.is_literal());

    if let Some(idx) = index_opt {
        let current = next_formulas.remove(idx);
        let mut children = Vec::new();

match current.clone() {
            // ---------------------------------------------------------
            // 1. REGULI ALPHA (O singură ramură - "AND-like")
            // ---------------------------------------------------------
            
            // A & B  -> Adaugă A, Adaugă B
            Formula::And(a, b) => {
                let mut path = next_formulas;
                path.push(*a); path.push(*b);
                children.push(build_tableau(path));
            },
            
            // !(A v B) -> Adaugă !A, Adaugă !B (De Morgan)
            Formula::Not(bx) if matches!(*bx, Formula::Or(_, _)) => {
                if let Formula::Or(a, b) = *bx {
                    let mut path = next_formulas;
                    path.push(Formula::not(*a)); path.push(Formula::not(*b));
                    children.push(build_tableau(path));
                }
            },
            
            // !(A -> B) -> Adaugă A, Adaugă !B  <-- ASTA ÎȚI LIPSEA PENTRU (P -> P)
            Formula::Not(bx) if matches!(*bx, Formula::Implies(_, _)) => {
                if let Formula::Implies(a, b) = *bx {
                    let mut path = next_formulas;
                    path.push(*a);               // Ipoteza devine adevărată
                    path.push(Formula::not(*b)); // Concluzia devine falsă
                    children.push(build_tableau(path));
                }
            },
            
            // !!A -> Adaugă A (Dubla negație) <-- ASTA ÎȚI LIPSEA PENTRU (P | !P)
            Formula::Not(bx) if matches!(*bx, Formula::Not(_)) => {
                if let Formula::Not(a) = *bx {
                    let mut path = next_formulas;
                    path.push(*a);
                    children.push(build_tableau(path));
                }
            },

            // ---------------------------------------------------------
            // 2. REGULI BETA (Ramificare - "OR-like")
            // ---------------------------------------------------------
            
            // A v B -> Ramură cu A | Ramură cu B
            Formula::Or(a, b) => {
                let mut p1 = next_formulas.clone(); p1.push(*a);
                let mut p2 = next_formulas;         p2.push(*b);
                children.push(build_tableau(p1));
                children.push(build_tableau(p2));
            },
            
            // A -> B -> Ramură cu !A | Ramură cu B
            Formula::Implies(a, b) => {
                let mut p1 = next_formulas.clone(); p1.push(Formula::not(*a));
                let mut p2 = next_formulas;         p2.push(*b);
                children.push(build_tableau(p1));
                children.push(build_tableau(p2));
            },
            
            // !(A & B) -> Ramură cu !A | Ramură cu !B (De Morgan)
            Formula::Not(bx) if matches!(*bx, Formula::And(_, _)) => {
                if let Formula::And(a, b) = *bx {
                    let mut p1 = next_formulas.clone(); p1.push(Formula::not(*a));
                    let mut p2 = next_formulas;         p2.push(Formula::not(*b));
                    children.push(build_tableau(p1));
                    children.push(build_tableau(p2));
                }
            },

            _ => {}
        }        
let computed_status = if children.iter().all(|c| c.status == NodeStatus::Closed) {
            NodeStatus::Closed
        } else {
            // Dacă măcar un copil e deschis (sau intermediar), nodul curent nu e închis.
            NodeStatus::Intermediate 
        };

        return TableauNode {
            formulas,
            children,
            status: computed_status, // Aici era hardcodat Intermediate înainte
        };
    }

    // Asta e pentru nodurile frunză care nu au contradicții (rămân Open)
    TableauNode {
        formulas,
        children: vec![],
        status: NodeStatus::Open,
    }
}
