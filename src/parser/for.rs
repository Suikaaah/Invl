use crate::parser::{Statement, Variable};
use std::collections::LinkedList;

#[derive(Debug)]
pub struct For {
    pub vars: LinkedList<LinkedList<Variable>>,
    pub containers: LinkedList<(Variable, Option<Variable>)>,
    pub statement: Box<Statement>,
}
