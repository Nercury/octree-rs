use std::hash::Hash;
use cgmath::*;
use super::node::Node;

pub mod all;
pub mod ray_intersection;
pub mod debug;

pub struct State<'a, S: 'a, K: 'a>
    where S: BaseNum,
          K: Clone + Eq + Hash
{
    pub node: &'a Node<S, K>,
    pub leaf_index: usize,
    pub node_index: usize,
}

pub enum Action<'a, S: 'a, K: 'a>
    where S: BaseNum,
          K: Clone + Eq + Hash
{
    Pop,
    Push(&'a Node<S, K>),
}
