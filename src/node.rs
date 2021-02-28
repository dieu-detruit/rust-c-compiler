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
    Return(Box<Node>),
    If(Box<(Node, Node)>),
    IfElse(Box<(Node, Node, Node)>),
    For(Box<(Node, Node, Node, Node)>),
    While(Box<(Node, Node)>),
    Block(Vec<Node>),
    Empty,
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

impl Node {
    pub fn is_block(&self) -> bool {
        match self {
            Node::Block(_) => true,
            _ => false,
        }
    }
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
        Node::LVar(offset) => format!("[var {}]", offset),
        Node::Assign(assign_arg) => {
            format!(
                "Assign {0} <- {1}",
                &sprint_node(&assign_arg.0),
                &sprint_node(&assign_arg.1)
            )
        }
        Node::Return(return_arg) => format!("return {}", &sprint_node(&*return_arg)),
        Node::If(if_arg) => format!(
            "If ({0}) Then {1}",
            &sprint_node(&if_arg.0),
            &sprint_node(&if_arg.1)
        ),
        Node::IfElse(if_arg) => format!(
            "If ({0}) Then {1} Else {2}",
            &sprint_node(&if_arg.0),
            &sprint_node(&if_arg.1),
            &sprint_node(&if_arg.2)
        ),
        Node::For(for_arg) => format!(
            "For ({0}; {1}; {2}) {3}",
            &sprint_node(&for_arg.0),
            &sprint_node(&for_arg.1),
            &sprint_node(&for_arg.2),
            &sprint_node(&for_arg.3)
        ),
        Node::While(while_arg) => format!(
            "while ({0}) {1}",
            &sprint_node(&while_arg.0),
            &sprint_node(&while_arg.1)
        ),
        Node::Block(statements) => {
            statements
                .iter()
                .fold(String::from("Block {\n"), |out, stmt| {
                    out + &sprint_node(&stmt) + "\n"
                })
                + "}"
        }
        Node::Empty => String::from("Do Nothing"),
    }
}
