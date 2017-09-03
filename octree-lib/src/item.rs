use collision::*;
use cgmath::*;
use std::fmt;
use std::cmp;

/// Tree item with the `bb` volume and `id` identifier.
#[derive(Clone)]
pub struct Item<S, K> {
    pub bb: Aabb3<S>,
    pub id: K,
}

impl<S, K> fmt::Debug for Item<S, K>
    where K: fmt::Debug,
          S: BaseNum
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Item")
            .field("bb", &self.bb)
            .field("id", &self.id)
            .finish()
    }
}

impl<S, K> cmp::PartialEq for Item<S, K>
    where K: cmp::PartialEq,
          S: BaseNum
{
    fn eq(&self, other: &Item<S, K>) -> bool {
        self.id.eq(&other.id) && self.bb.eq(&other.bb)
    }
}

impl<S, K> cmp::Eq for Item<S, K>
    where K: cmp::Eq,
          S: BaseNum
{}
