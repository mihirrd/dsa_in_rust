#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {

    fn new(val: i32) -> ListNode {
        ListNode {val, next: None}
    }
}

pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>>{
        let mut head = None;

        for &item in vec.iter().rev() {
            let mut new_node = ListNode::new(item);
            new_node.next = head;
            head = Some(Box::new(new_node));
        }
        head
}
