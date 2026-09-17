//! Deterministic repository/dependency/impact graph.

use atc_core::{CoreError, Result};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeKind { Repository, File, Module, Dependency, Standard, Requirement, Test, Workflow, Schema, Documentation }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Relation { DependsOn, Implements, Verifies, Governs, Documents, Affects, Requires, Supersedes }

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Node { pub id: String, pub kind: NodeKind }

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Edge { pub from: String, pub relation: Relation, pub to: String }

#[derive(Clone, Debug, Default)]
pub struct ImpactGraph { nodes: HashMap<String, Node>, edges: Vec<Edge> }

impl ImpactGraph {
    pub fn add_node(&mut self, node: Node) -> Result<()> {
        if self.nodes.contains_key(&node.id) { return Err(CoreError::InvalidState(format!("duplicate graph node: {}", node.id))); }
        self.nodes.insert(node.id.clone(), node); Ok(())
    }
    pub fn add_edge(&mut self, edge: Edge) -> Result<()> {
        if !self.nodes.contains_key(&edge.from) || !self.nodes.contains_key(&edge.to) { return Err(CoreError::InvalidState("graph edge references unknown node".into())); }
        if self.edges.contains(&edge) { return Ok(()); }
        self.edges.push(edge); Ok(())
    }
    pub fn affected_by(&self, root: &str) -> Result<Vec<String>> {
        if !self.nodes.contains_key(root) { return Err(CoreError::InvalidId(format!("unknown graph node: {root}"))); }
        let mut seen = HashSet::new(); let mut queue = vec![root.to_string()];
        while let Some(current)=queue.pop() { for e in self.edges.iter().filter(|e| e.from==current) { if seen.insert(e.to.clone()) { queue.push(e.to.clone()); } } }
        let mut result: Vec<_> = seen.into_iter().collect(); result.sort(); Ok(result)
    }
    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }
}

#[cfg(test)]
mod tests { use super::*; #[test] fn traverses_impact() { let mut g=ImpactGraph::default(); g.add_node(Node{id:"a".into(),kind:NodeKind::File}).unwrap(); g.add_node(Node{id:"b".into(),kind:NodeKind::Test}).unwrap(); g.add_edge(Edge{from:"a".into(),relation:Relation::Verifies,to:"b".into()}).unwrap(); assert_eq!(g.affected_by("a").unwrap(), vec!["b"]); } }
