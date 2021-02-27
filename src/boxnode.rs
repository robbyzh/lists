type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct LinkList<T> {
    head: Link<T>,
}

impl<T> LinkList<T>
where
    T: std::fmt::Debug + std::fmt::Display,
{
    pub fn push_back(&mut self, ele: T) {
        let new_node = Node {
            elem: ele,
            next: None,
        };

        match self.head.as_mut() {
            None => self.head = Some(Box::new(new_node)),
            Some(mut curr_node) => {
                loop {
                    if let Some(ref mut next_node) = curr_node.next {
                        curr_node = next_node;
                    } else {
                        break;
                    }
                }
                curr_node.next.replace(Box::new(new_node));
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let mut curr_link = &mut self.head;
        let mut n = index;
        while n > 0 {
            curr_link = match curr_link.as_mut() {
                None => return None,
                Some(node) => &mut node.next,
            };
            n -= 1;
        }

        //这里看不懂
        match curr_link.take().map(|x| *x) {
            Some(Node { elem, next }) => {
                *curr_link = next;
                Some(elem)
            }
            None => None,
        }
    }

    //java reverseList(Node head) {
    //  Node prev = null;
    //  Node curr = head;
    //  while (curr != null) {
    //    Node tmp = curr.next;
    //    curr.next = prev;
    //    prev = curr;
    //    curr = tmp;
    //  }
    //  return prev;
    //}
    pub fn reverse(&mut self) {
        let mut prev_link = None;
        let mut curr_link = self.head.take();

        while let Some(mut curr_node) = curr_link {
            curr_link = curr_node.next.take();
            curr_node.next = prev_link;
            prev_link = Some(curr_node);
        }
        self.head = prev_link;
    }

    pub fn stringify(&self) -> String {
        let mut s = String::from("");
        //这里的curr_node变量的定义很重要
        if let Some(mut curr_node) = self.head.as_ref() {
            s = format!("{}", curr_node.elem);
            loop {
                if let Some(ref next_node) = curr_node.next {
                    curr_node = next_node;
                    s = format!("{},{}", s, curr_node.elem);
                } else {
                    break;
                }
            }
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::LinkList;
    #[test]
    fn test_push() {
        let mut list: LinkList<i32> = LinkList { head: None };
        for i in 1..5 {
            list.push_back(i);
        }

        assert_eq!("1,2,3,4", list.stringify());

        list.reverse();
        assert_eq!("4,3,2,1", list.stringify());
        list.remove(0);
        println!("list={}", list.stringify());

        let mut s = &mut Some(1);
        println!("{:?}", s);
        *s = Some(10);
        *s = Some(20);
        println!("{:?}", s);
        //你没有let绑定定义那个some（2）它就是个临时值
        //s = &mut Some(2);
        println!("{:?}", s);

        //String解引用成了 str
        let x = "hello".to_owned();
        match &*x {
            "hello" => {
                println!("hello");
            }
            _ => {}
        }
    }
}
