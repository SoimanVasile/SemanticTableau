use crate::formula::Formula;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeStatus {
    Open,
    Closed,
    Intermediate,
}

pub struct TableauNode {
    pub formulas: Vec<Formula>,
    pub children: Vec<TableauNode>,
    pub status: NodeStatus,
}
