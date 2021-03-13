pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
    Rn(usize),
}

impl Register {
    pub fn get_name(&self, size: usize) -> String {
        match self {
            Register::RAX => match size {
                1 => "al",
                2 => "ax",
                4 => "eax",
                8 => "rax",
                _ => panic!("invalid size to access register"),
            }
            .to_string(),
            Register::RBX => match size {
                1 => "bl",
                2 => "bx",
                4 => "ebx",
                8 => "rbx",
                _ => panic!("invalid size to access register"),
            }
            .to_string(),
            Register::RCX => match size {
                1 => "cl",
                2 => "cx",
                4 => "ecx",
                8 => "rcx",
                _ => panic!("invalid size to access register"),
            }
            .to_string(),
            Register::RDX => match size {
                1 => "dl",
                2 => "dx",
                4 => "edx",
                8 => "rdx",
                _ => panic!("invalid size to access register"),
            }
            .to_string(),
            Register::RSI => match size {
                1 => "sil",
                2 => "si",
                4 => "esi",
                8 => "rsi",
                _ => panic!("invalid size to access register"),
            }
            .to_string(),
            Register::RDI => match size {
                1 => "dil",
                2 => "di",
                4 => "edi",
                8 => "rdi",
                _ => panic!("invalid size to access register"),
            }
            .to_string(),
            Register::RBP => match size {
                1 => "bpl",
                2 => "bp",
                4 => "ebp",
                8 => "rbp",
                _ => panic!("invalid size to access register"),
            }
            .to_string(),
            Register::RSP => match size {
                1 => "spl",
                2 => "sp",
                4 => "esp",
                8 => "rsp",
                _ => panic!("invalid size to access register"),
            }
            .to_string(),
            Register::Rn(n) => {
                if *n < 8 || 15 < *n {
                    panic!("register r{} does not exist", n);
                }
                match size {
                    1 => format!("r{}b", n),
                    2 => format!("r{}w", n),
                    4 => format!("r{}d", n),
                    8 => format!("r{}", n),
                    _ => panic!("invalid size to access register"),
                }
            }
        }
    }
}
