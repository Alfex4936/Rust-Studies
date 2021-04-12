type ChildNode<T> = Option<Box<BTNode<T>>>;

enum Op<T> {
    Add,
    Sub,
    Div,
    Mul,
    Id(T),
}

struct BTNode<T> {
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: Op<T>,
}

impl BTNode<i32> {
    pub fn new(op: Op<i32>, l: BTNode<i32>, r: BTNode<i32>) -> Self {
        BTNode::<i32> {
            op: op,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }
    }
}

fn add_node(l: BTNode<i32>, r: BTNode<i32>) -> BTNode<i32> {
    BTNode::new(Op::Add, l, r)
}

fn sub_node(l: BTNode<i32>, r: BTNode<i32>) -> BTNode<i32> {
    BTNode::new(Op::Sub, l, r)
}

fn div_node(l: BTNode<i32>, r: BTNode<i32>) -> BTNode<i32> {
    BTNode::new(Op::Div, l, r)
}

fn mul_node(l: BTNode<i32>, r: BTNode<i32>) -> BTNode<i32> {
    BTNode::new(Op::Mul, l, r)
}

fn new_node(value: i32) -> BTNode<i32> {
    BTNode {
        op: Op::Id(value),
        left: None,
        right: None,
    }
}

struct BinaryTree<T> {
    head: Option<BTNode<T>>, // nullable
}

impl BinaryTree<i32> {
    pub fn new(head: BTNode<i32>) -> Self {
        BinaryTree::<i32> { head: Some(head) }
    }
    pub fn collapse(node: &Box<BTNode<i32>>) -> i32 {
        let mut r: Option<i32> = None;
        let mut l: Option<i32> = None;
        if let Some(left) = &node.left {
            l = Some(BinaryTree::collapse(left));
        }
        if let Some(right) = &node.right {
            r = Some(BinaryTree::collapse(right));
        }
        let r = if let Some(x) = r { x } else { 0 };
        let l = if let Some(x) = l { x } else { 0 };
        match node.op {
            Op::Add => l + r,
            Op::Sub => l - r,
            Op::Mul => l * r,
            Op::Div => {
                if r == 0 {
                    panic!("Attempted divide-by-zero operation.")
                }
                l / r
            }
            Op::Id(x) => x,
        }
    }
}

fn main() {
    // (4 - (2 * -2)) + (8 + (12 / 2))
    let bt = BinaryTree::new(add_node(
        sub_node(new_node(4), mul_node(new_node(2), new_node(-2))),
        add_node(new_node(8), div_node(new_node(12), new_node(2))),
    ));

    println!(
        "{}",
        BinaryTree::collapse(&Box::new(bt.head.expect("No head initialized.")))
    )
}
