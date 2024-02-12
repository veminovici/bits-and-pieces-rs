use core::fmt;
use std::{
    iter,
    ptr::{self, NonNull},
};

type Link<T> = Option<NonNull<Node<T>>>;

pub struct Node<T> {
    /// Item is None only if the node is a head node.
    pub item: Option<T>,
    /// How high the node reaches.
    pub level: usize,
    /// The immediate previous element.
    pub prev: Link<T>,
    // Vector of links to the next node at the respective level.  This vector
    // *must* be of length `self.level + 1`.  links[0] stores a pointer to the
    // next node, which will have to be dropped.
    pub links: Vec<Link<T>>,
    // The corresponding length of each link
    pub links_len: Vec<usize>,
}

impl<T> fmt::Display for Node<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref t) = self.item {
            write!(f, "{}", t)
        } else {
            Ok(())
        }
    }
}

impl<T> Node<T> {
    /// Create a new head node.
    pub fn head(total_levels: usize) -> Self {
        Self {
            item: None,
            level: total_levels - 1,
            prev: None,
            links: iter::repeat(None).take(total_levels).collect(),
            links_len: iter::repeat(0).take(total_levels).collect(),
        }
    }

    /// Create a new Note with the given item. All pointers default to null.
    pub fn new(item: T, level: usize) -> Self {
        Self {
            item: Some(item),
            level,
            prev: None,
            links: iter::repeat(None).take(level + 1).collect(),
            links_len: iter::repeat(0).take(level + 1).collect(),
        }
    }

    /// Consumes the node returning the item it contains.
    pub fn into_inner(mut self) -> Option<T> {
        self.item.take()
    }

    /// Returns `true` is the node is a head-node.
    pub fn is_head(&self) -> bool {
        self.prev.is_none()
    }

    /// Returns the next node in the list, at the level 0.
    pub fn next_ref(&self) -> Option<&Self> {
        // SAFETY: all links either points to something or is null.
        unsafe { self.links[0].as_ref().map(|p| p.as_ref()) }
    }

    /// Returns the mut reference to the next node in the list, at the level 0.
    pub fn next_mut(&mut self) -> Option<&mut Self> {
        // SAFETY: all links either points to something or is null.
        unsafe { self.links[0].as_mut().map(|p| p.as_mut()) }
    }

    /// Takes the next node and set next_node.prev as null.
    ///
    /// SAFETY: please make sure no link at level 1 or greater becomes dangling.
    pub unsafe fn take_tail(&mut self) -> Option<Box<Self>> {
        self.links[0].take().map(|p| {
            let mut next = Box::from_raw(p.as_ptr());
            next.prev = None;
            self.links_len[0] = 0;
            next
        })
    }

    /// Replace the next node.
    /// Return the old node.
    ///
    /// SAFETY: please makes sure all links are fixed.
    pub unsafe fn replace_tail(&mut self, mut new_next: Box<Self>) -> Option<Box<Self>> {
        let mut old_next = self.take_tail();
        if let Some(old_next) = old_next.as_mut() {
            old_next.prev = None;
        }
        new_next.prev = Some(NonNull::new_unchecked(self as *mut _));
        self.links[0] = Some(NonNull::new_unchecked(Box::into_raw(new_next)));
        self.links_len[0] = 1;
        old_next
    }

    /// Retain all nodes who satisfies `pred`. Return the number removed nodes.
    ///
    /// Requires `self` being the head of the skiplist.
    ///
    /// `pred` is a function that takes two parameters, `Option<&V>` and  `&V`.
    /// `Option<&V>` is the value of current node (`None` if the current node is the head),
    /// `&V` is the value of the next node.
    /// If the `pred` returns `false`, then the next node is dropped.
    #[must_use]
    pub fn retain<F>(&mut self, mut pred: F) -> usize
    where
        F: FnMut(Option<&T>, &T) -> bool,
    {
        assert!(self.is_head());
        let mut removed_count = 0;
        // Aliasing mutable references is undefined behavior.
        // However if you create a pointer from a mutable reference,
        // it essentially borrows from it, we are free to alias it until
        // the next time we use that reference.
        let mut current_node = self as *mut Self;
        // `level_heads` records every head of the linked list.
        // A head is the last node of a given level that is not after current_node.
        let mut level_heads: Vec<_> = iter::repeat(current_node).take(self.level + 1).collect();
        // SAFETY: a huge block of pointer manipulation.
        unsafe {
            while let Some(mut next_node) = (*current_node).take_tail() {
                // next_node is removed from the list, so we can refer it by value.
                if pred(
                    (*current_node).item.as_ref(),
                    next_node.item.as_ref().unwrap(),
                ) {
                    // Keeping next_node.
                    // First we should update level_heads, then we put next_node back to the list.
                    for x in &mut level_heads[0..=next_node.level] {
                        *x = next_node.as_mut() as *mut _;
                    }
                    (*current_node).replace_tail(next_node);
                    current_node = (*current_node).next_mut().unwrap();
                } else {
                    // Remove next_node.
                    removed_count += 1;
                    // Fixes links above level 0.
                    for (level, head) in level_heads
                        .iter_mut()
                        .map(|&mut node_p| &mut *node_p)
                        .enumerate()
                        .skip(1)
                    {
                        if level <= next_node.level {
                            assert!(ptr::eq(
                                head.links[level].unwrap().as_ptr(),
                                next_node.as_mut()
                            ));
                            head.links_len[level] += next_node.links_len[level];
                            head.links_len[level] -= 1;
                            head.links[level] = next_node.links[level];
                        } else {
                            head.links_len[level] -= 1;
                        }
                    }
                    // Fix the link at level 0.
                    if let Some(new_next) = next_node.take_tail() {
                        (*current_node).replace_tail(new_next);
                    }
                }
            }
        }
        removed_count
    }
}
