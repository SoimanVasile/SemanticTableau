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
            Formula::And(a, b) => {
                let mut path = next_formulas;
                path.push(*a); path.push(*b);
                children.push(build_tableau(path));
            },
            Formula::Not(bx) if matches!(*bx, Formula::Or(_, _)) => {
                if let Formula::Or(a, b) = *bx {
                    let mut path = next_formulas;
                    path.push(Formula::not(*a)); path.push(Formula::not(*b));
                    children.push(build_tableau(path));
                }
            },
             Formula::Implies(a, b) => {
                // Beta rule example
                let mut p1 = next_formulas.clone(); p1.push(Formula::not(*a));
                let mut p2 = next_formulas;         p2.push(*b);
                children.push(build_tableau(p1));
                children.push(build_tableau(p2));
            },
            Formula::Or(a, b) => {
                let mut p1 = next_formulas.clone(); p1.push(*a);
                let mut p2 = next_formulas;         p2.push(*b);
                children.push(build_tableau(p1));
                children.push(build_tableau(p2));
            },
           _ => {}
        }

        return TableauNode {
            formulas,
            children,
            status: NodeStatus::Intermediate,
        };
    }

    TableauNode {
        formulas,
        children: vec![],
        status: NodeStatus::Open,
    }
}
