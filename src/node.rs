use crate::typename::{sprint_typename, Typename};

pub struct LVar {
    pub offset: usize,
    pub typename: Typename,
}

//pub struct Block<'a> {
//parent: Option<&'a Block<'a>>,
//pub local_vars: HashMap<String, LVar>,
//pub local_var_size: usize,
//}

//impl Block<'_> {
//pub fn get_var(&self, name: &str) -> Option<LVar> {
//match self.local_vars.get(&name) {
//None => match self.parent {
//None => None,
//Some(parent_ref) => parent_ref.get_var(name),
//},
//Some(var) => var,
//}
//}
//}

#[derive(Clone)]
pub enum Node {
    Unary(Box<Node>, UnaryType),           // arg, unary_type
    Binary(Box<(Node, Node)>, BinaryType), // (arg1, arg2), binary_type
    Num(i32),                              // n
    Boolean(bool),                         // boolean_value
    LVar(usize, Typename),                 // offset, typename
    Assign(Box<(Node, Node)>),             // lvalue, rvalue
    Return(Option<Box<Node>>),             // return arg
    If(Box<(Node, Node)>),                 // (cond, if_true)
    IfElse(Box<(Node, Node, Node)>),       // (cond, if_true, else)
    For(Box<(Node, Node, Node, Node)>),    // (init, cond, update, loop_content)
    While(Box<(Node, Node)>),              // (cond, loop_content)
    Block(Vec<Node>),                      // statement[]
    Function(String, Typename, Vec<Typename>, Box<Node>, usize), // name,  return_type, arg_type[], block, local_var_size
    FunctionCall(String, Vec<Node>),                             // name, arg[]
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

    pub fn expect_lvar(&self) -> (usize, Typename) {
        match self {
            Node::LVar(offset, typename) => (*offset, typename.clone()),
            _ => panic!("unexpected node (expect: lvar)"),
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
        Node::LVar(offset, typename) => {
            format!("[var {0}:{1}]", offset, sprint_typename(&typename))
        }
        Node::Assign(assign_arg) => {
            format!(
                "Assign {0} <- {1}",
                &sprint_node(&assign_arg.0),
                &sprint_node(&assign_arg.1)
            )
        }
        Node::Return(return_arg_optional) => match return_arg_optional {
            None => format!("return nothing"),
            Some(return_arg) => format!("return {}", &sprint_node(&*return_arg)),
        },
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
        Node::Function(name, return_type, arg_types, block, _local_var_size) => {
            format!(
                "function (type: {0}({1}), name: {2})\n",
                sprint_typename(return_type),
                arg_types.iter().fold(String::new(), |out, arg_type| {
                    out + &sprint_typename(&arg_type) + ", "
                }),
                name
            ) + &sprint_node(&block)
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
    }
}
