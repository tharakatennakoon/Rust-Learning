// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
 
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut out = ListNode::new(-1);
    let mut temp = &mut out;

    let mut p = l1;
    let mut q = l2;

    let mut c = 0;
    loop {
        let mut p_f = false;
        let mut p_val = 0;
        if let Some(ref l1_node) = p {
            p_val = l1_node.val;
            p = p.unwrap().next;
        }
        else {
            p_f = true;
        }

        let mut q_f = false;
        let mut q_val = 0;
        if let Some(ref l2_node) = q {
            q_val = l2_node.val;
            q = q.unwrap().next;
        }
        else {
            q_f = true;
        }

        if (p_f && q_f) {
            break;
        }

        let val = c + p_val + q_val;
        c = val / 10;

        temp.next = Some(Box::new(ListNode::new(val % 10)));
        temp = temp.next.as_mut().unwrap();

    }
    
    if c > 0 {
        temp.next = Some(Box::new(ListNode::new(c)));
        temp = temp.next.as_mut().unwrap();
    }

    return out.next;
}

#[cfg(test)]
mod tests_add2numbers {
    use super::add_two_numbers;
    use super::ListNode;

    fn helper(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut front = Box::new(ListNode::new(0));
        let mut current = &mut front;

        for x in v {
            current.next = Some(Box::new(ListNode::new(x)));
            current = current.next.as_mut().unwrap();
        }

        front.next
    }

    #[test]
    fn test1() {
        assert_eq!(
            add_two_numbers(helper(vec![2, 4, 3]), helper(vec![5, 6, 4])),
            helper(vec![7, 0, 8])
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            add_two_numbers(helper(vec![0]), helper(vec![0])),
            helper(vec![0])
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            add_two_numbers(helper(vec![9, 9, 9, 9, 9, 9, 9]), helper(vec![9, 9, 9, 9])),
            helper(vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }

    #[test]
    fn test4() {
        let v1 = helper(vec![2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,9]);
        let v2 = helper(vec![5,6,4,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,9,9,9,9]);
        let expected = helper(vec![7,0,8,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,4,8,6,1,4,3,9,1]);

        assert_eq!(
            add_two_numbers(v1, v2),
            expected
        );
    }
}
