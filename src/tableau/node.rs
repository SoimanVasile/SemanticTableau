use serde::Serialize;

use crate::formula::Formula;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum NodeStatus {
    Open,
    Closed,
    Intermediate,
}

#[derive(Debug, Clone, Serialize)]
pub struct TableauNode {
    pub formulas: Vec<Formula>,
    pub children: Vec<TableauNode>,
    pub status: NodeStatus,
}
