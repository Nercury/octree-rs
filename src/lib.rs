// Copyright 2017 Nerijus Arlauskas
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate cgmath;
extern crate collision;

mod node;
mod item;
mod iterator;

pub use item::Item;
pub use iterator::all::OctreeIter;
pub use iterator::debug::{DebugIter, DebugItem};
pub use iterator::ray_intersection::{RayIntersection, RayIntersectionsIter};

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt;
use node::Node;
use cgmath::*;
use collision::*;

/// Hierarchical storage of items sorted by location in subdivided 3D space.
pub struct Octree<S, K>
    where S: BaseNum,
          K: Clone + Eq + Hash
{
    branch_size: usize,
    root: Box<Node<S, K>>,
    object_node: HashMap<K, *mut Node<S, K>>,
}

impl<S, K> Octree<S, K>
    where K: Clone + Eq + Hash,
          S: BaseFloat
{
    /// Create a new tree with default branch size of `16` and the world size enclosed by `bb`
    /// volume.
    pub fn new(bb: Aabb3<S>) -> Octree<S, K> {
        assert!(bb.volume() > S::zero());

        // println!("new Octree {:?}", bb);

        Octree {
            branch_size: 16,
            root: Box::new(Node::new(bb)),
            object_node: HashMap::new(),
        }
    }

    /// Create a new tree with specified branch size and world size enclosed by `bb` volume.
    pub fn with_branch_size(branch_size: usize, bb: Aabb3<S>) -> Octree<S, K> {
        assert!(branch_size > 0);
        assert!(bb.volume() > S::zero());

        // println!("new Octree {:?}", bb);

        Octree {
            branch_size: branch_size,
            root: Box::new(Node::new(bb)),
            object_node: HashMap::new(),
        }
    }

    /// Insert or update tree item's `bb` volume.
    ///
    /// If the item's `bb` volume is outside of the world, the item will still be added,
    /// however, the space for it will not be subdivided, and the item will always incur
    /// constant performance penalty.
    pub fn update(&mut self, id: K, bb: Aabb3<S>)
        where K: fmt::Debug
    {
        // println!("update {:?}, {:?}", id, bb);

        let maybe_node = self.object_node.get_mut(&id).map(|v| unsafe { v.as_mut() }.unwrap());
        if let Some(node) = maybe_node {
            node.update(self.branch_size, id, bb, &mut self.object_node);
            return;
        }

        self.root.insert(self.branch_size, id, bb, &mut self.object_node);
    }

    /// Remove tree item.
    pub fn remove(&mut self, id: K) -> Option<Item<S, K>> {
        match self.object_node.entry(id.clone()) {
            Entry::Occupied(mut e) => {
                let leaf = unsafe { e.get_mut().as_mut() }.unwrap().remove(id);
                e.remove();
                leaf
            }
            _ => None,
        }
    }

    /// Get iterator over all items that intersect the specified `ray`.
    ///
    /// The item order is unspecified and can vary wildly between tree modifications.
    pub fn ray_intersections<'a>(&'a self, ray: Ray3<S>) -> RayIntersectionsIter<'a, S, K> {
        RayIntersectionsIter::new(ray, &self.root)
    }

    /// Get iterator over all items including the node volumes.
    ///
    /// Useful for debugging.
    pub fn debug_items<'a>(&'a self) -> DebugIter<'a, S, K> {
        DebugIter::new(&self.root)
    }
}

impl<'a, S, K> IntoIterator for &'a Octree<S, K>
    where S: BaseNum,
          K: Clone + Eq + Hash + fmt::Debug
{
    type Item = &'a Item<S, K>;
    type IntoIter = OctreeIter<'a, S, K>;

    fn into_iter(self) -> OctreeIter<'a, S, K> {
        OctreeIter::new(&self.root)
    }
}

#[cfg(test)]
mod tests {
    use collision::*;
    use cgmath::*;
    use super::*;

    #[test]
    fn should_insert_if_not_fits_inside_world() {
        let mut oc = Octree::with_branch_size(1,
                                              Aabb3::new(Point3::new(0.0, 0.0, 0.0),
                                                         Point3::new(10.0, 10.0, 10.0)));
        oc.update(1,
                  Aabb3::new(Point3::new(0.0, 0.0, 0.0), Point3::new(30.0, 30.0, 30.0)));

        let list: Vec<_> = oc.into_iter().collect();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, 1);
    }

    #[test]
    fn should_insert_if_fits_inside_world() {
        let mut oc = Octree::with_branch_size(1,
                                              Aabb3::new(Point3::new(0.0, 0.0, 0.0),
                                                         Point3::new(10.0, 10.0, 10.0)));
        oc.update(1,
                  Aabb3::new(Point3::new(1.0, 1.0, 1.0), Point3::new(5.0, 5.0, 5.0)));

        let list: Vec<_> = oc.into_iter().collect();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, 1);
    }

    #[test]
    fn should_insert_and_subdivide() {
        let mut oc = Octree::with_branch_size(1,
                                              Aabb3::new(Point3::new(0.0, 0.0, 0.0),
                                                         Point3::new(10.0, 10.0, 10.0)));
        oc.update(1,
                  Aabb3::new(Point3::new(2.0, 2.0, 2.0), Point3::new(5.0, 5.0, 5.0)));

        oc.update(2,
                  Aabb3::new(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0)));

        let list: Vec<_> = oc.into_iter().collect();
        assert_eq!(list.len(), 2);

        assert_eq!(2,
                   oc.ray_intersections(Ray3::new(Point3::new(0.0, 0.0, 0.0),
                                                  vec3(1.0, 1.0, 1.0).normalize()))
                       .count());

        assert_eq!(0,
                   oc.ray_intersections(Ray3::new(Point3::new(0.0, 0.0, 0.0),
                                                  vec3(1.0, 0.0, 0.0).normalize()))
                       .count());

        let hits: Vec<_> = oc.ray_intersections(Ray3::new(Point3::new(1.5, 1.5, 0.0),
                                                          vec3(0.0, 0.0, 1.0).normalize()))
            .collect();

        assert_eq!(1, hits.len());
        assert_eq!(hits[0].item().id, 2);

        let hits: Vec<_> = oc.ray_intersections(Ray3::new(Point3::new(3.0, 3.0, 0.0),
                                                          vec3(0.001, 0.001, 1.0).normalize()))
            .collect();

        assert_eq!(1, hits.len());
        assert_eq!(hits[0].item().id, 1);
    }

    #[test]
    fn should_be_able_to_remove_all() {
        let mut oc = Octree::with_branch_size(1,
                                              Aabb3::new(Point3::new(0.0, 0.0, 0.0),
                                                         Point3::new(10.0, 10.0, 10.0)));
        oc.update(1,
                  Aabb3::new(Point3::new(2.0, 2.0, 2.0), Point3::new(5.0, 5.0, 5.0)));

        oc.update(2,
                  Aabb3::new(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0)));

        assert!(oc.root.is_branch);

        assert_eq!(oc.remove(3), None);
        assert_eq!(oc.remove(2),
                   Some(Item {
                       id: 2,
                       bb: Aabb3::new(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0)),
                   }));

        let list: Vec<_> = oc.into_iter().cloned().collect();
        assert_eq!(1, list.len());
        assert_eq!(list[0].id, 1);

        assert_eq!(oc.remove(1),
                   Some(Item {
                       id: 1,
                       bb: Aabb3::new(Point3::new(2.0, 2.0, 2.0), Point3::new(5.0, 5.0, 5.0)),
                   }));

        assert!(!oc.root.is_branch);
        assert!(oc.root.leafs.is_empty());
    }

    #[test]
    fn items_moved_outside_of_the_world_should_be_moved_to_root_node() {
        let mut oc = Octree::with_branch_size(1,
                                              Aabb3::new(Point3::new(0.0, 0.0, 0.0),
                                                         Point3::new(10.0, 10.0, 10.0)));
        oc.update(1,
                  Aabb3::new(Point3::new(2.0, 2.0, 2.0), Point3::new(5.0, 5.0, 5.0)));

        oc.update(2,
                  Aabb3::new(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0)));

        oc.update(2,
                  Aabb3::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(-1.0, -1.0, -1.0)));

        let list: Vec<_> = oc.into_iter().cloned().collect();

        assert_eq!(2, list.len());
        assert!(!oc.root.is_branch);
        assert_eq!(2, oc.root.leafs.len());

        oc.update(1,
                  Aabb3::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(-1.0, -1.0, -1.0)));

        let list: Vec<_> = oc.into_iter().cloned().collect();

        assert_eq!(2, list.len());
        assert!(!oc.root.is_branch);
        assert_eq!(2, oc.root.leafs.len());

        assert_eq!(list[0].bb,
                   Aabb3::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(-1.0, -1.0, -1.0)));
        assert_eq!(list[1].bb,
                   Aabb3::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(-1.0, -1.0, -1.0)));
    }

    #[test]
    fn items_moved_outside_the_world_should_not_create_subnodes() {
        let mut oc = Octree::with_branch_size(1,
                                              Aabb3::new(Point3::new(0.0, 0.0, 0.0),
                                                         Point3::new(10.0, 10.0, 10.0)));

        oc.update(1,
                  Aabb3::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(-1.0, -1.0, -1.0)));

        oc.update(2,
                  Aabb3::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(-1.0, -1.0, -1.0)));

        oc.update(1,
                  Aabb3::new(Point3::new(-3.0, -2.0, -2.0), Point3::new(-1.0, -1.0, -1.0)));

        let list: Vec<_> = oc.into_iter().cloned().collect();

        assert_eq!(2, list.len());
        assert!(!oc.root.is_branch);
    }

    #[test]
    fn items_moved_into_the_world_should_be_moved_to_subnodes() {
        let mut oc = Octree::with_branch_size(1,
                                              Aabb3::new(Point3::new(0.0, 0.0, 0.0),
                                                         Point3::new(10.0, 10.0, 10.0)));

        oc.update(1,
                  Aabb3::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(-1.0, -1.0, -1.0)));

        oc.update(2,
                  Aabb3::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(-1.0, -1.0, -1.0)));

        let list: Vec<_> = oc.into_iter().cloned().collect();

        assert_eq!(2, list.len());
        assert!(!oc.root.is_branch);

        oc.update(2,
                  Aabb3::new(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0)));

        assert_eq!(2, list.len());
        assert!(oc.root.is_branch);

        oc.update(1,
                  Aabb3::new(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0)));

        assert_eq!(2, list.len());
        assert!(oc.root.is_branch);

        oc.remove(1);
        oc.remove(2);

        let list: Vec<_> = oc.into_iter().cloned().collect();

        assert_eq!(0, list.len());
        assert!(!oc.root.is_branch);
    }
}

