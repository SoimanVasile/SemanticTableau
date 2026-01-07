use super::node::{TableauNode, NodeStatus};

pub fn print_tree(node: &TableauNode, prefix: String, is_last: bool) {
    let marker = if is_last { "└── " } else { "├── " };
    
    let forms_str = node.formulas.iter()
        .map(|f| format!("{:?}", f))
        .collect::<Vec<_>>()
        .join(", ");

    let status_str = match node.status {
        NodeStatus::Closed => " [❌ ÎNCHIS]",
        NodeStatus::Open => " [✅ DESCHIS]",
        NodeStatus::Intermediate => "",
    };

    println!("{}{}{}{}", prefix, marker, forms_str, status_str);

    let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
    
    for (i, child) in node.children.iter().enumerate() {
        print_tree(child, child_prefix.clone(), i == node.children.len() - 1);
    }
}
