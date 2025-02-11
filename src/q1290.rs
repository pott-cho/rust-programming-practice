/// single linked list 생성
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node { next: None, val }
    }

    fn add(&mut self, val: i32) {
        let mut last = self;
        while let Some(ref mut next_node) = last.next {
            last = next_node;
        }
        last.next = Some(Box::new(Node::new(val)));
    }
}

impl Node {
    pub fn make_singly_linked_list(arr: &[i32]) -> Node {
        let mut root: Node = Node::new(arr[0]);
        for &val in &arr[1..] {
            root.add(val);
        }
        root
    }
}

/// 문제풀이
pub fn get_decimal_value(head: Node) -> i32 {
    let mut root = Some(Box::new(head));
    let mut v = Vec::new();
    while root.is_some() {
        if let Some(node) = root {
            v.push(node.val);
            root = node.next;
        }
    }

    let mut result = 0;
    for (mul, i) in (0..v.len()).rev().enumerate() {
        result += v[i] << mul;
    }

    result
}
