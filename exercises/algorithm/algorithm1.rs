/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::ptr::NonNull;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            val,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let node = Box::new(Node::new(obj));
        let node_ptr = NonNull::new(Box::into_raw(node)).unwrap();

        match self.end {
            None => self.start = Some(node_ptr),
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = Some(node_ptr) },
        }
        self.end = Some(node_ptr);
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, mut node: Option<NonNull<Node<T>>>, mut index: i32) -> Option<&T> {
        while let Some(next_ptr) = node {
            let next_node = unsafe { &*next_ptr.as_ptr() };
            if index == 0 {
                return Some(&next_node.val);
            } else {
                node = next_node.next;
                index -= 1;
            }
        }
        None
    }

    pub fn merge(list_a: Self, list_b: Self) -> Self
    where
        T: Ord + Clone, // Add Clone bound
    {
        let mut result = Self::new();

        let mut current_a = list_a.start;
        let mut current_b = list_b.start;

        while let (Some(a_ptr), Some(b_ptr)) = (current_a, current_b) {
            let a = unsafe { &*a_ptr.as_ptr() };
            let b = unsafe { &*b_ptr.as_ptr() };

            if a.val < b.val {
                result.add(a.val.clone()); // Clone the value
                current_a = a.next;
            } else {
                result.add(b.val.clone()); // Clone the value
                current_b = b.next;
            }
        }

        // Append the remaining nodes from list_a or list_b
        let mut remaining = current_a.or(current_b);
        while let Some(next_ptr) = remaining {
            let val = unsafe { &*next_ptr.as_ptr() }.val.clone(); // Clone the value
            result.add(val);
            remaining = unsafe { (*next_ptr.as_ptr()).next };
        }

        result
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;

        while let Some(ptr) = current {
            let node = unsafe { &*ptr.as_ptr() };
            write!(f, "{}", node.val)?;

            if let Some(next_ptr) = node.next {
                write!(f, " -> ")?;
                current = Some(next_ptr);
            } else {
                break;
            }
        }

        Ok(())
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in vec_a {
            list_a.add(i);
        }
        for i in vec_b {
            list_b.add(i);
        }
        println!("list a: {}", list_a);
        println!("list b: {}", list_b);
        let list_c = LinkedList::merge(list_a, list_b);
        println!("merged List: {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in vec_a {
            list_a.add(i);
        }
        for i in vec_b {
            list_b.add(i);
        }
        println!("list a: {}", list_a);
        println!("list b: {}", list_b);
        let list_c = LinkedList::merge(list_a, list_b);
        println!("merged List: {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}