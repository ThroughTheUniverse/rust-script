use std::fmt::Display;

pub enum OpCode {
    Constant,
    None,
    True,
    False,
    Pop,
    GetLocal,
    SetLocal,
    GetGlobal,
    DefineGlobal,
    SetGlobal,
    GetProperty,
    SetProperty,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Negate,
    Print,
    Jump,
    JumpIfFalse,
    Loop,
    Call,
    Invoke,
    Return,
    Struct,
    Method,
    Modulo,
    Power,
    End,
    Unknown,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        use OpCode::*;
        match value {
            0 => Constant,
            1 => None,
            2 => True,
            3 => False,
            4 => Pop,
            5 => GetLocal,
            6 => SetLocal,
            7 => GetGlobal,
            8 => DefineGlobal,
            9 => SetGlobal,
            10 => GetProperty,
            11 => SetProperty,
            12 => Equal,
            13 => Greater,
            14 => Less,
            15 => Add,
            16 => Subtract,
            17 => Multiply,
            18 => Divide,
            19 => Not,
            20 => Negate,
            21 => Print,
            22 => Jump,
            23 => JumpIfFalse,
            24 => Loop,
            25 => Call,
            26 => Invoke,
            27 => Return,
            28 => Struct,
            29 => Method,
            30 => Modulo,
            31 => Power,
            32 => End,
            _ => Unknown,
        }
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OpCode::*;
        match self {
            Constant => write!(f, "Constant"),
            None => write!(f, "None"),
            True => write!(f, "True"),
            False => write!(f, "False"),
            Pop => write!(f, "Pop"),
            GetLocal => write!(f, "Getlocal"),
            SetLocal => write!(f, "Setlocal"),
            GetGlobal => write!(f, "GetGlobal"),
            DefineGlobal => write!(f, "DefineGlobal"),
            SetGlobal => write!(f, "SetGlobal"),
            GetProperty => write!(f, "GetProperty"),
            SetProperty => write!(f, "SetProperty"),
            Equal => write!(f, "Equal"),
            Greater => write!(f, "Greater"),
            Less => write!(f, "Less"),
            Add => write!(f, "Add"),
            Subtract => write!(f, "Subtract"),
            Multiply => write!(f, "Multiply"),
            Divide => write!(f, "Divide"),
            Not => write!(f, "Not"),
            Negate => write!(f, "Negate"),
            Print => write!(f, "Print"),
            Jump => write!(f, "Jump"),
            JumpIfFalse => write!(f, "JumpIfFalse"),
            Loop => write!(f, "Loop"),
            Call => write!(f, "Call"),
            Invoke => write!(f, "Invoke"),
            Return => write!(f, "Return"),
            Struct => write!(f, "Struct"),
            Method => write!(f, "Method"),
            Modulo => write!(f, "Modulo"),
            Power => write!(f, "Power"),
            End => write!(f, "End"),
            Unknown => write!(f, "Unknown"),
        }
    }
}

impl OpCode {
    pub fn to_offset(&self) -> usize {
        use OpCode::*;
        match self {
            Constant => 2,
            None => 1,
            True => 1,
            False => 1,
            Pop => 1,
            GetLocal => 2,
            SetLocal => 2,
            GetGlobal => 2,
            DefineGlobal => 2,
            SetGlobal => 2,
            GetProperty => 2,
            SetProperty => 2,
            Equal => 1,
            Greater => 1,
            Less => 1,
            Add => 1,
            Subtract => 1,
            Multiply => 1,
            Divide => 1,
            Not => 1,
            Negate => 1,
            Print => 1,
            Jump => 3,
            JumpIfFalse => 3,
            Loop => 3,
            Call => 2,
            Invoke => 3,
            Return => 1,
            Struct => 2,
            Method => 2,
            Modulo => 1,
            Power => 1,
            End => 1,
            Unknown => 1,
        }
    }
}
