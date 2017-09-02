use std::hash::Hash;
use std::fmt;
use cgmath::*;
use collision::*;
use Item;
use super::super::iterator;
use super::super::node::Node;

/// Ray intersection with the item.
pub struct RayIntersection<'a, S: 'a, K: 'a> {
    hit_point: Point3<S>,
    hit_item: &'a Item<S, K>,
}

impl<'a, S: 'a, K: 'a> RayIntersection<'a, S, K>
    where S: Clone
{
    /// Retrieve the intersection point.
    pub fn point(&self) -> Point3<S> {
        self.hit_point.clone()
    }

    /// Retrieve the reference to item.
    pub fn item<'r>(&'r self) -> &'a Item<S, K> {
        self.hit_item
    }
}

/// Iterator over Ray intersections in the tree.
pub struct RayIntersectionsIter<'a, S: 'a, K: 'a>
    where S: BaseNum,
          K: Clone + Eq + Hash
{
    ray: Ray3<S>,
    stack: Vec<iterator::State<'a, S, K>>,
}

impl<'a, S: 'a, K: 'a> RayIntersectionsIter<'a, S, K>
    where S: BaseNum + BaseFloat,
          K: Clone + Eq + Hash
{
    pub fn new<'r>(ray: Ray3<S>, node: &'r Node<S, K>) -> RayIntersectionsIter<'r, S, K> {
        RayIntersectionsIter {
            ray: ray,
            stack: vec![iterator::State {
                node: node,
                leaf_index: 0,
                node_index: 0,
            }],
        }
    }
}

impl<'a, S: 'a, K: 'a> Iterator for RayIntersectionsIter<'a, S, K>
    where S: BaseNum + BaseFloat,
          K: Clone + Eq + Hash + fmt::Debug
{
    type Item = RayIntersection<'a, S, K>;

    fn next<'i>(&'i mut self) -> Option<RayIntersection<'a, S, K>> {
        while !self.stack.is_empty() {
            let iter_action = {
                let &mut iterator::State { ref node, ref mut leaf_index, ref mut node_index } =
                    self.stack.last_mut().unwrap();

                if *leaf_index < node.leafs.len() {
                    // leaf iteration state
                    let leaf_candidate = unsafe { node.leafs.get_unchecked(*leaf_index) };
                    *leaf_index += 1;

                    if let Some(point) = (self.ray, leaf_candidate.bb).intersection() {
                        // println!("ok! {:?}", point);
                        return Some(RayIntersection {
                            hit_point: point,
                            hit_item: leaf_candidate,
                        });
                    }

                    continue;
                } else {
                    // node iteration state, does not produce leafs until
                    // we go into leaf state
                    if node.is_branch && *node_index < 8 {
                        // if we have subnode, switch to it
                        let node_candidate = node.get_node(*node_index);

                        *node_index += 1;

                        if node_candidate.is_empty() {
                            continue;
                        }

                        if let None = (self.ray, node_candidate.bb).intersection() {
                            continue;
                        }

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
                }
            };
        }

        None
    }
}
