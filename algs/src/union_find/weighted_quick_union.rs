use super::UnionBeforeInsert;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub struct Weighted_Quick_Union<T> {
    item_to_id: HashMap<T, usize>,
    parents: RefCell<Vec<usize>>,
    sizes: Vec<usize>,
    enable_path_compression: bool,
}

impl<T> Default for Weighted_Quick_Union<T> {
    fn default() -> Self {
        Weighted_Quick_Union {
            item_to_id: HashMap::new(),
            parents: RefCell::new(Vec::new()),
            sizes: Vec::new(),
            enable_path_compression: false,
        }
    }
}

impl<T> Weighted_Quick_Union<T>
where
    T: Debug + Hash + Eq,
{
    pub fn new() -> Self {
        Weighted_Quick_Union::default()
    }

    pub fn with_path_compression(mut self) -> Self {
        self.enable_path_compression = true;
        self
    }

    pub fn insert(&mut self, item: T) {
        let new_id = self.parents.borrow().len();
        self.item_to_id.entry(item).or_insert(new_id);
        self.parents.borrow_mut().push(new_id);
        self.sizes.push(1);
    }

    fn root_of(&self, child_id: usize) -> usize {
        let mut cur_id = child_id;
        let mut parents = self.parents.borrow_mut();
        while parents[cur_id] != cur_id {
            if self.enable_path_compression {
                let grandparent_id = parents[parents[cur_id]];
                parents[cur_id] = grandparent_id;
            }
            cur_id = parents[cur_id];
        }

        cur_id
    }

    // the size of the union
    pub fn size(&self) -> usize {
        self.parents.borrow().len()
    }

    pub fn is_connected(&self, p: &T, q: &T) -> bool {
        let Some(pid) = self.item_to_id.get(p) else {
            return false;
        };

        let Some(qid) = self.item_to_id.get(q) else {
            return false;
        };

        self.root_of(*pid) == self.root_of(*qid)
    }

    pub fn union(&mut self, p: &T, q: &T) -> Result<(), UnionBeforeInsert> {
        let Some(pid) = self.item_to_id.get(p) else {
            return Err(UnionBeforeInsert);
        };

        let Some(qid) = self.item_to_id.get(q) else {
            return Err(UnionBeforeInsert);
        };

        let proot = self.root_of(*pid);
        let qroot = self.root_of(*qid);

        let mut parents = self.parents.borrow_mut();

        if self.sizes[proot] > self.sizes[qroot] {
            parents[qroot] = proot;
            self.sizes[proot] += self.sizes[qroot];
        } else {
            parents[proot] = qroot;
            self.sizes[qroot] += self.sizes[proot];
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_weighted_quick_union_basics() {
        let mut wqu = Weighted_Quick_Union::new();
        wqu.insert(0);
        wqu.insert(1);
        wqu.insert(2);
        wqu.insert(3);
        wqu.insert(4);

        assert!(wqu.union(&0, &1).is_ok());
        assert!(wqu.union(&1, &2).is_ok());

        assert!(wqu.union(&2, &3).is_ok());
        assert!(wqu.union(&3, &4).is_ok());

        assert_eq!(wqu.sizes, [1, 5, 1, 1, 1]);
        assert!(wqu.parents.borrow().iter().all(|&x| x == 1));
    }

    #[test]
    fn test_weighted_quick_union_with_path_compression() {
        let mut wqu = Weighted_Quick_Union::new().with_path_compression();
        wqu.insert(0);
        wqu.insert(1);
        wqu.insert(2);
        wqu.insert(3);
        wqu.insert(4);

        assert!(wqu.union(&0, &1).is_ok());
        assert!(wqu.union(&1, &2).is_ok());

        assert!(wqu.union(&2, &3).is_ok());
        assert!(wqu.union(&3, &4).is_ok());

        assert_eq!(wqu.sizes, [1, 5, 1, 1, 1]);
        assert!(wqu.parents.borrow().iter().all(|&x| x == 1));
    }
}
