use std::hash::Hash;
use std::fmt;
use cgmath::*;
use Item;
use super::super::iterator;
use super::super::node::Node;

/// Iterator over all items in the tree.
pub struct OctreeIter<'a, S: 'a, K: 'a>
    where S: BaseNum,
          K: Clone + Eq + Hash
{
    stack: Vec<iterator::State<'a, S, K>>,
}

impl<'a, S: 'a, K: 'a> OctreeIter<'a, S, K>
    where S: BaseNum,
          K: Clone + Eq + Hash
{
    pub fn new<'r>(node: &'r Node<S, K>) -> OctreeIter<'r, S, K> {
        OctreeIter {
            stack: vec![iterator::State {
                node: node,
                leaf_index: 0,
                node_index: 0,
            }],
        }
    }
}

impl<'a, S: 'a, K: 'a> Iterator for OctreeIter<'a, S, K>
    where S: BaseNum,
          K: Clone + Eq + Hash + fmt::Debug
{
    type Item = &'a Item<S, K>;

    fn next<'i>(&'i mut self) -> Option<&'a Item<S, K>> {
        while !self.stack.is_empty() {
            let iter_action = {
                let &mut iterator::State { ref node, ref mut leaf_index, ref mut node_index } =
                    self.stack.last_mut().unwrap();

                if *leaf_index < node.leafs.len() {
                    // leaf iteration state
                    let leaf = unsafe { node.leafs.get_unchecked(*leaf_index) };
                    *leaf_index += 1;

                    return Some(leaf);
                } else {
                    // node iteration state, does not produce leafs until
                    // we go into leaf state
                    if node.is_branch && *node_index < 8 {
                        // if we have subnode, switch to it
                        let node = node.get_node(*node_index);
                        *node_index += 1;

                        if node.is_empty() {
                            continue;
                        }

                        iterator::Action::Push(node)
                    } else {
                        // otherwise, go up (out of subnodes or not a branch)
                        iterator::Action::Pop
                    }
                }
            };

            // do actions when out of previous mutable borrowed state
            match iter_action {
                iterator::Action::Pop => {
                    self.stack.pop();
                }
                iterator::Action::Push(node) => {
                    self.stack.push(iterator::State {
                        node: node,
                        leaf_index: 0,
                        node_index: 0,
                    });
                }
            };
        }

        None
    }
}
