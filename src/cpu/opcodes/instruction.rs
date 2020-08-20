pub enum Instruction {
    Add(ArithmeticRegister)
}

pub enum ArithmeticRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}