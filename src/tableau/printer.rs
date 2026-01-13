use super::node::{TableauNode, NodeStatus};
use colored::*;
use std::{thread, time};
use std::sync::atomic::{AtomicUsize, Ordering};

const ANIMATION_DELAY: u64 = 100;
static NODE_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn reset_speed() {
    NODE_COUNTER.store(0, Ordering::Relaxed);
}

pub fn print_tree(node: &TableauNode, prefix: String, is_last: bool) {
    let count = NODE_COUNTER.fetch_add(1, Ordering::Relaxed);

    let delay = 0;
    // let delay = if count < 15{
    // ANIMATION_DELAY
    // }
    // else if count < 35 {
        // ANIMATION_DELAY/6
    // }
    // else{
        // 0
    // };
        

    if delay > 0{
        thread::sleep(time::Duration::from_millis(delay));
    }

    let marker = if is_last { "└── " } else { "├── " };
    
    let forms_str = node.formulas.iter()
        .map(|f| format!("{}", f))
        .collect::<Vec<_>>()
        .join(", ");

    let status_str = match node.status {
        NodeStatus::Closed => " [❌ ÎNCHIS]".red().bold(),
        NodeStatus::Open => " [✅ DESCHIS]".green().bold(),
        NodeStatus::Intermediate => "".normal(),
    };

    println!("{}{}{}{}", prefix, marker, forms_str, status_str);

    let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
    
    for (i, child) in node.children.iter().enumerate() {
        print_tree(child, child_prefix.clone(), i == node.children.len() - 1);
    }
}
