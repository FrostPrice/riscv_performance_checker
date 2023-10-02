/////////////////////////////////////////
/*
Importante:
Cada formato de insrução terá um CPI:
R(String),
I(String),
S(String),
B(String),
U(String),
J(String),
L(String),
*/

#[derive(PartialEq, Clone, Debug)]
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
    full_inst: String,
    opcode: OpCodeType,
    rd: String,
    rs1: String,
    rs2: String,
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

        // 00000000000000000000 00000 0000000
        let end = instruction.len() - 7;
        let mut rd = instruction[end - 5..end].to_string();

        // 000000000000 00000 000000000000000
        let end = instruction.len() - 15;
        let rs1 = instruction[end - 5..end].to_string();

        // 0000000 00000 00000000000000000000
        let end = instruction.len() - 20;
        let rs2 = instruction[end - 5..end].to_string();

        // TODO: Remember this
        match opcode {
            OpCodeType::S(_) => {
                rd = rs2.clone();
            }
            _ => (),
        }

        Self {
            full_inst: instruction.to_string(),
            opcode,
            rd,
            rs1,
            rs2,
        }
    }

    pub fn get_full_inst(self) -> String {
        self.full_inst
    }

    pub fn get_opcode(self) -> OpCodeType {
        self.opcode
    }

    pub fn get_rd(self) -> String {
        self.rd
    }

    pub fn get_rs1(self) -> String {
        self.rs1
    }

    pub fn get_rs2(self) -> String {
        self.rs2
    }
}
