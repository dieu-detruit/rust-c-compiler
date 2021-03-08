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
    Function(String, Vec<Node>),
    FunctionCall(String, Vec<Node>),
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
        Node::Boolean(b) => if *b { "True" } else { "False" }.into(),
        Node::Unary(_unary_arg, _unary_type) => String::new(),
        Node::Binary(binary_arg, binary_type) => {
            return match binary_type {
                BinaryType::Add => "+",
                BinaryType::Sub => "-",
                BinaryType::Mul => "*",
                BinaryType::Div => "/",
                BinaryType::Lt => "<",
                BinaryType::LtEq => "<=",
                BinaryType::NotEqual => "!=",
                _ => "",
            }
            .to_string()
                + format!(
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
        Node::FunctionCall(name, arg_list) => {
            arg_list
                .iter()
                .fold(format!("Call {} (", name), |out, arg| {
                    out + &sprint_node(&arg) + ", "
                })
                + ")"
        }
        Node::Block(statements) => {
            statements
                .iter()
                .fold("Block {\n".to_string(), |out, stmt| {
                    out + &sprint_node(&stmt) + "\n"
                })
                + "}"
        }
        Node::Empty => "Do Nothing".to_string(),
        _ => String::new(),
    }
}
