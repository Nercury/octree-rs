use std::hash::Hash;
use std::fmt;
use cgmath::*;
use collision::*;
use Item;
use super::super::iterator;
use super::super::node::Node;

pub enum DebugItem<'a, S: 'a, K: 'a> {
    Node {
        bb: Aabb3<S>,
        depth: usize,
        goodness: f32,
        is_branch: bool,
    },
    Item {
        item: &'a Item<S, K>,
        depth: usize,
        goodness: f32,
        in_branch: bool,
    },
}

/// Debug iterator over all volumes and items.
pub struct DebugIter<'a, S: 'a, K: 'a>
    where S: BaseNum,
          K: Clone + Eq + Hash
{
    stack: Vec<iterator::State<'a, S, K>>,
}

impl<'a, S: 'a, K: 'a> DebugIter<'a, S, K>
    where S: BaseNum + BaseFloat,
          K: Clone + Eq + Hash
{
    pub fn new<'r>(node: &'r Node<S, K>) -> DebugIter<'r, S, K> {
        DebugIter {
            stack: vec![iterator::State {
                node: node,
                leaf_index: 0,
                node_index: 0,
            }],
        }
    }
}

fn calculate_goodness(depth: usize) -> f32 {
    let mut fdepth = depth as f32;

    let good_val = 10.0;
    let bad_val = 1.0;

    if fdepth < bad_val {
        fdepth = bad_val;
    }
    if fdepth > good_val {
        fdepth = good_val;
    }

    (fdepth - bad_val) / (good_val - bad_val)
}

impl<'a, S: 'a, K: 'a> Iterator for DebugIter<'a, S, K>
    where S: BaseNum + BaseFloat,
          K: Clone + Eq + Hash + fmt::Debug
{
    type Item = DebugItem<'a, S, K>;

    fn next<'i>(&'i mut self) -> Option<DebugItem<'a, S, K>> {
        let stack_depth = self.stack.len();

        while !self.stack.is_empty() {
            let iter_action = {
                let &mut iterator::State { ref node, ref mut leaf_index, ref mut node_index } =
                    self.stack.last_mut().unwrap();

                if *leaf_index < node.leafs.len() {
                    // leaf iteration state
                    let leaf_candidate = unsafe { node.leafs.get_unchecked(*leaf_index) };
                    *leaf_index += 1;

                    return Some(DebugItem::Item {
                        item: leaf_candidate,
                        depth: stack_depth,
                        goodness: calculate_goodness(stack_depth),
                        in_branch: node.is_branch,
                    });
                } else {
                    // node iteration state, does not produce leafs until
                    // we go into leaf state
                    if node.is_branch && *node_index < 8 {
                        // if we have subnode, switch to it
                        let node_candidate = node.get_node(*node_index);

                        *node_index += 1;

                        iterator::Action::Push(node_candidate)
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

                    return Some(DebugItem::Node {
                        bb: node.bb,
                        depth: stack_depth + 1,
                        goodness: calculate_goodness(stack_depth + 1),
                        is_branch: node.is_branch,
                    });
                }
            };
        }

        None
    }
}
