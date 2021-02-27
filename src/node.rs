pub struct LVar {
    pub offset: usize,
}

#[derive(Clone)]
pub enum Node {
    Unary(Box<Node>, UnaryType),
    Binary(Box<(Node, Node)>, BinaryType),
    Num(i32),
    Boolean(bool),
    LVar(usize),
    Assign(Box<(Node, Node)>),
}

#[derive(Clone)]
pub enum UnaryType {
    Not,
}

#[derive(Clone)]
pub enum BinaryType {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Lt,
    LtEq,
}

pub fn sprint_node(node: &Node) -> String {
    match node {
        Node::Num(n) => n.to_string(),
        Node::Boolean(b) => {
            if *b {
                String::from("True")
            } else {
                String::from("False")
            }
        }
        Node::Unary(_unary_arg, _unary_type) => String::from(""),
        Node::Binary(binary_arg, binary_type) => {
            return match binary_type {
                BinaryType::Add => String::from("+"),
                BinaryType::Sub => String::from("-"),
                BinaryType::Mul => String::from("*"),
                BinaryType::Div => String::from("/"),
                BinaryType::Lt => String::from("<"),
                BinaryType::LtEq => String::from("<="),
                BinaryType::NotEqual => String::from("!="),
                _ => String::from(""),
            } + format!(
                "({0}, {1})",
                &sprint_node(&binary_arg.0),
                &sprint_node(&binary_arg.1)
            )
            .as_str()
        }
        Node::LVar(offset) => format!("var {}", offset),
        Node::Assign(assign_arg) => {
            format!(
                "Assign {0} <- {1}",
                &sprint_node(&assign_arg.0),
                &sprint_node(&assign_arg.1)
            )
        }
    }
}
