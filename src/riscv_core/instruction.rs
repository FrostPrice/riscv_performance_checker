#[derive(Clone, Debug)]
pub enum OpCodeType {
    R(String),
    I(String),
    L(String),
    S(String),
    B(String),
    U(String),
    J(String),
}

#[derive(Clone, Debug)]
pub struct Instruction {
    opcode: OpCodeType,
}

impl Instruction {
    pub fn new(instruction: &str) -> Self {
        let opcode_bits = &instruction[instruction.len() - 7..];
        let opcode = match opcode_bits {
            "0110011" => OpCodeType::R(opcode_bits.to_string()),
            "1110011" | "0010011" | "0001111" | "1100111" => OpCodeType::I(opcode_bits.to_string()),
            "0000011" => OpCodeType::L(opcode_bits.to_string()),
            "0100011" => OpCodeType::S(opcode_bits.to_string()),
            "1100011" => OpCodeType::B(opcode_bits.to_string()),
            "0110111" => OpCodeType::U(opcode_bits.to_string()),
            "1101111" => OpCodeType::J(opcode_bits.to_string()),
            _ => unimplemented!("Opcode not implemented"),
        };

        Self { opcode }
    }

    pub fn get_opcode(self) -> OpCodeType {
        self.opcode
    }
}
