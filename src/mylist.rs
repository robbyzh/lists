use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    size: u32,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}


fn t1<'a>()->Box<i32> {
    let a = Box::new(1);
    a
}

impl<T> LinkedList<T>
where
    T: std::fmt::Display + std::cmp::Eq + std::fmt::Debug,
{
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    fn prepend(&mut self, elem: T) {
        let new_head = Rc::new(RefCell::new(Node {
            data: elem,
            next: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                new_head.as_ref().borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            None => self.head = Some(new_head),
        }
        self.size += 1;
    }

    fn len(&self) -> u32 {
        self.size
    }

    fn exist(&self, elem: T) -> bool {
        let mut list_head = self.head.clone();
        loop {
            match list_head {
                Some(head) => {
                    if head.as_ref().borrow().data == elem {
                        break true;
                    } else {
                        list_head = head.as_ref().borrow().next.clone();
                    }
                }
                None => break false,
            }
        }
    }
    fn remove(&mut self, index: u32) -> Option<T> {
        if index >= self.size {
            return None;
        }

        let mut prev: Option<Rc<RefCell<Node<T>>>> = None;
        let mut curr: Rc<RefCell<Node<T>>> = self.head.clone().unwrap();
        for _i in 0..index {
            prev = Some(curr.clone());
            curr = curr.clone().as_ref().borrow_mut().next.clone().unwrap();
        }

        match prev {
            Some(node) => {
                node.as_ref().borrow_mut().next = curr.as_ref().borrow().next.clone();
            }
            None => self.head = curr.as_ref().borrow().next.clone(),
        }
        self.size -= 1;
        println!("current node={:?}", curr.clone());

        Some(Rc::try_unwrap(curr).ok().unwrap().into_inner().data)
    }

    fn stringify(&self) -> String {
        let mut list_head: Link<T> = self.head.clone();
        let mut s = String::from("");
        loop {
            match list_head.clone() {
                Some(head) => {
                    s += &(format!("{:?},", list_head.unwrap().as_ref().borrow().data));
                    list_head = head.as_ref().borrow().next.clone();
                }
                None => break s,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_list_base() {
        let mut list: LinkedList<u128> = LinkedList::new();
        for i in 1..=10 {
            list.prepend(i);
        }
        assert_eq!(list.len(), 10);
        println!("len = {}", list.len());
        assert_eq!(list.len(), 10);

        assert_eq!(Some(4), list.remove(6));
        assert_eq!(Some(10), list.remove(0));
        println!("stringify={}", list.stringify());
        println!("len = {}", list.len());
        for i in 1..10 {
            println!("i={}, exist={}", i, list.exist(i));
        }
        println!("len = {}", list.len());

        println!("{}",super::t1());
    }
}
