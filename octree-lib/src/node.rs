use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ptr;
use std::fmt;
use std::cmp::Eq;
use std::hash::Hash;
use cgmath::*;
use collision::*;
use super::item::Item;

pub struct Node<S, K>
    where S: BaseNum,
          K: Clone + Eq + Hash
{
    pub is_branch: bool,
    pub leafs: Vec<Item<S, K>>,
    pub bb: Aabb3<S>,

    center: Point3<S>,
    parent: *mut Node<S, K>,
    nodes: [*mut Node<S, K>; 8],
}

impl<S, K> fmt::Debug for Node<S, K>
    where K: fmt::Debug + Clone + Eq + Hash,
          S: BaseNum
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
            .field("is_branch", &self.is_branch)
            .field("leafs", &self.leafs)
            .field("bb", &self.bb)
            .field("center", &self.center)
            .field("parent", &self.parent)
            .field("nodes", &self.nodes)
            .finish()
    }
}

impl<S, K> Node<S, K>
    where S: BaseNum,
          K: Clone + Eq + Hash
{
    pub fn new(bb: Aabb3<S>) -> Node<S, K> {
        Node {
            is_branch: false,
            bb: bb,
            center: bb.center(),
            leafs: Vec::new(),
            parent: ptr::null_mut(),
            nodes: [ptr::null_mut(); 8],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.leafs.is_empty() && !self.is_branch
    }

    pub fn insert(&mut self,
                  branch_size: usize,
                  id: K,
                  bb: Aabb3<S>,
                  object_node: &mut HashMap<K, *mut Node<S, K>>)
        where S: BaseFloat
    {
        // println!("insert");

        if self.leafs.len() >= branch_size {
            if let Some(index) = self.select_node_index(&bb) {
                if !self.is_branch {
                    // println!("not branch and leafs.len >= max");
                    self.subdivide();
                    self.move_leafs_to_children(branch_size, object_node);
                    // println!("branch = true");
                    self.is_branch = true;
                }

                // println!("selected branch index {:?}", index);
                self.get_node_mut(index).insert(branch_size, id, bb, object_node);
                return;
            }
        }

        // println!("add leaf {:?}", self.leafs.len());

        self.leafs.push(Item {
            bb: bb,
            id: id.clone(),
        });

        // update item's parent node in object node index
        match object_node.entry(id) {
            Entry::Vacant(e) => {
                e.insert(self);
            }
            Entry::Occupied(mut e) => *e.get_mut() = self,
        };
    }

    /// This must be called only if it is known that `id` exists in this node.
    ///
    /// The track of object nodes is kept in `object_node`.
    pub fn update(&mut self,
                  branch_size: usize,
                  id: K,
                  bb: Aabb3<S>,
                  object_node: &mut HashMap<K, *mut Node<S, K>>)
        where S: BaseFloat,
              K: fmt::Debug
    {
        // println!("update id {:?}", id);

        if self.can_contain(&bb) {
            // println!("can contain {:?}", bb);

            if self.is_branch || self.leafs.len() >= branch_size {
                // println!("is branch || leafs > max");

                // if we are in branch that means item is crossing octree boundary
                // and we should search if item can be inserted into child node instead
                if let Some(index) = self.select_node_index(&bb) {
                    // println!("found new subnode");

                    self.remove(id.clone())
                        .expect("failed to remove leaf that was known to exist");
                    // object node will be updated when item is inserted somewhere

                    if !self.is_branch {
                        // println!("not branch yet");
                        self.subdivide();
                        self.move_leafs_to_children(branch_size, object_node);
                        // println!("branch = true");
                        self.is_branch = true;
                    }

                    self.get_node_mut(index).insert(branch_size, id, bb, object_node);
                    return;
                }
            }

            // println!("update leaf item bb");
            // if not is branch or can't be moved to a child, simply update bb
            self.get_leaf_by_id_unchecked_mut(id).bb = bb;
            return;
        }

        // does not fit into this node
        // println!("does not fit in");

        for i in 0..self.leafs.len() {
            if unsafe { self.leafs.get_unchecked(i) }.id == id {
                // println!("swap remove {:?}", id);
                self.leafs.swap_remove(i);
                break;
            }
        }
        // object node will be updated when item is inserted somewhere

        // always find new parent or at least the root
        self.find_new_parent(&bb).insert(branch_size, id, bb, object_node);

        if !self.is_branch && self.leafs.is_empty() {
            self.try_cleanup_parent();
        }
    }

    /// Note that remove *does not* take care of removing node from `object_node` index.
    ///
    /// It should be done by the caller.
    pub fn remove(&mut self, id: K) -> Option<Item<S, K>> {
        let mut leaf = None;

        for i in 0..self.leafs.len() {
            if unsafe { self.leafs.get_unchecked(i) }.id == id {
                leaf = Some(self.leafs.swap_remove(i));
                break;
            }
        }

        if leaf.is_some() && !self.is_branch && self.leafs.is_empty() {
            self.try_cleanup_parent();
        }

        leaf
    }

    pub fn can_contain(&self, other: &Aabb3<S>) -> bool
        where S: BaseFloat
    {
        if self.parent.is_null() {
            return true;
        }

        self.contains_bb(other)
    }

    pub fn contains_bb(&self, other: &Aabb3<S>) -> bool
        where S: BaseFloat
    {
        other.min.x >= self.bb.min.x && other.min.y >= self.bb.min.y &&
            other.min.z >= self.bb.min.z && other.max.x <= self.bb.max.x &&
            other.max.y <= self.bb.max.y && other.max.z <= self.bb.max.z
    }

    fn find_new_parent(&mut self, bb: &Aabb3<S>) -> &mut Node<S, K>
        where S: BaseFloat
    {
        assert!(!self.parent.is_null());

        // println!("parent candidate {:?}", self.parent);
        let mut node = unsafe { self.parent.as_mut() }.unwrap();

        while !node.can_contain(bb) {
            // println!("parent candidate {:?}", node.parent);
            node = unsafe { node.parent.as_mut() }.unwrap();
        }

        node
    }

    fn try_cleanup_parent(&mut self) {
        if !self.parent.is_null() {
            unsafe { self.parent.as_mut() }.unwrap().downgrade_to_leaf_if_children_empty();
        }
    }

    pub fn downgrade_to_leaf_if_children_empty(&mut self) {
        for i in 0..8 {
            if !self.get_node(i).is_empty() {
                return;
            }
        }

        self.clean_nodes();
        self.is_branch = false;

        if self.leafs.is_empty() {
            self.try_cleanup_parent();
        }
    }

    fn move_leafs_to_children(&mut self,
                              branch_size: usize,
                              object_node: &mut HashMap<K, *mut Node<S, K>>)
        where S: BaseFloat
    {
        // println!("move_leafs_to_children");

        let mut i = 0;
        while i < self.leafs.len() {
            // if we can place the leaf item in subnode, move it, otherwise
            // retain it in the parent node
            let node_index = {
                let val = unsafe { self.leafs.get_unchecked(i) };
                self.select_node_index(&val.bb)
            };

            if let Some(index) = node_index {
                let item = self.leafs.swap_remove(i);

                self.get_node_mut(index).insert(branch_size, item.id, item.bb, object_node);
                continue;
            }

            i += 1;
        }
    }

    fn subdivide(&mut self) {
        // println!("subdivide");

        let v = self.bb;
        let c = self.center;

        // 0 .. 4 ---------------------------- where z < center.z
        // 0 .. 2 ---------------- where y < center.y
        // 0 where x < center.x
        self.set_node(0, node(v.min, c));

        // 1 where x > center.x
        self.set_node(1,
                      node(Point3::new(c.x, v.min.y, v.min.z),
                           Point3::new(v.max.x, c.y, c.z)));

        // 2 .. 4 ---------------- where y > center.y
        // 2 where x < center.x
        self.set_node(2,
                      node(Point3::new(v.min.x, c.y, v.min.z),
                           Point3::new(c.x, v.max.y, c.z)));

        // 3 where x > center.x
        self.set_node(3,
                      node(Point3::new(c.x, c.y, v.min.z),
                           Point3::new(v.max.x, v.max.y, c.z)));

        // 4 .. 8 ---------------------------- where z > center.z
        // 4 .. 6 ---------------- where y < center.y
        // 4 where x < center.x
        self.set_node(4,
                      node(Point3::new(v.min.x, v.min.y, c.z),
                           Point3::new(c.x, c.y, v.max.z)));

        // 5 where x > center.x
        self.set_node(5,
                      node(Point3::new(c.x, v.min.y, c.z),
                           Point3::new(v.max.x, c.y, v.max.z)));

        // 6 .. 8 ---------------- where y > center.y
        // 6 where x < center.x
        self.set_node(6,
                      node(Point3::new(v.min.x, c.y, c.z),
                           Point3::new(c.x, v.max.y, v.max.z)));

        // 7 where x > center.x
        self.set_node(7, node(c, v.max));
    }

    /// Item must exist!
    fn get_leaf_by_id_unchecked_mut(&mut self, id: K) -> &mut Item<S, K> {
        for leaf in &mut self.leafs {
            if leaf.id == id {
                return leaf;
            }
        }

        panic!("item leaf not found");
    }

    fn set_node(&mut self, index: usize, mut node: Box<Node<S, K>>) {
        assert!(unsafe { (*self.nodes.get_unchecked_mut(index)) }.is_null(),
                "placing node to non-null slot would leak other node");

        // take a pointer to heap allocated memory out of the box and
        // forget the box
        node.parent = self;
        self.nodes[index] = Box::<Node<S, K>>::into_raw(node);
    }

    fn take_node(&mut self, index: usize) -> Box<Node<S, K>> {
        assert!(!unsafe { (*self.nodes.get_unchecked_mut(index)) }.is_null(),
                "can not take null node");

        // conjure up a box and place heap allocated pointer back into
        // the box, so that box can now dealocate the contained object as usual
        let mut raw = unsafe { self.nodes.get_unchecked_mut(index) };
        let ret = unsafe { Box::<Node<S, K>>::from_raw(*raw) };
        *raw = ptr::null_mut();
        ret
    }

    pub fn get_node_mut(&mut self, index: usize) -> &mut Node<S, K> {
        assert!(!self.nodes[index].is_null());
        unsafe { self.nodes.get_unchecked_mut(index).as_mut() }.unwrap()
    }

    pub fn get_node(&self, index: usize) -> &Node<S, K> {
        assert!(!self.nodes[index].is_null());
        unsafe { self.nodes.get_unchecked(index).as_ref() }.unwrap()
    }

    fn select_node_index(&self, bb: &Aabb3<S>) -> Option<usize>
        where S: BaseFloat
    {
        let mut index = 0;

        if !self.contains_bb(bb) {
            // println!("{:?} can not contain {:?}", self.bb, bb);
            return None;
        }

        if bb.min.z <= self.center.z && bb.max.z <= self.center.z {
            // pass
        } else if bb.min.z >= self.center.z && bb.max.z >= self.center.z {
            index += 4;
        } else {
            // println!("{:?} on border z", bb);
            return None;
        }

        if bb.min.y <= self.center.y && bb.max.y <= self.center.y {
            // pass
        } else if bb.min.y >= self.center.y && bb.max.y >= self.center.y {
            index += 2;
        } else {
            // println!("{:?} on border y", bb);
            return None;
        }

        if bb.min.x <= self.center.x && bb.max.x <= self.center.x {
            // pass
        } else if bb.min.x >= self.center.x && bb.max.x >= self.center.x {
            index += 1;
        } else {
            // println!("{:?} on border x", bb);
            return None;
        }

        // println!("select index {:?} for {:?}", index, bb);

        Some(index)
    }

    fn clean_nodes(&mut self) {
        for i in 0..8 {
            self.take_node(i);
        }
    }
}

impl<S, K> Drop for Node<S, K>
    where K: Clone + Eq + Hash,
          S: BaseNum
{
    fn drop(&mut self) {
        if self.is_branch {
            self.clean_nodes();
        }
    }
}

fn node<S, K>(p1: Point3<S>, p2: Point3<S>) -> Box<Node<S, K>>
    where S: BaseNum,
          K: Clone + Eq + Hash
{
    Box::new(Node::new(Aabb3::new(p1, p2)))
}
