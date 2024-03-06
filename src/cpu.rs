#[allow(dead_code)]
pub struct CPU {
    pub reg_a: u8,
    pub reg_x: u8,
    pub status: Status,
    /// program counter
    pub pc: u8,
}

#[allow(dead_code)]
/// Processor Status Flags. Each flag is one bit in size.
/// # Flags:
/// - C  : Carry
///     The carry flag is set if the last operation caused an overflow from bit 7 of the result or an
///     underflow from bit 0. This condition is set during arithmetic, comparison and during logical
///     shifts. It can be explicitly set using the 'Set Carry Flag' (SEC) instruction and cleared with
///     'Clear Carry Flag' (CLC).
///
/// - Z  : Zero
///     The zero flag is set if the result of the last operation as was zero.
///
/// - I  : Interrupt Disable
///     The interrupt disable flag is set if the program has executed a 'Set Interrupt Disable' (SEI)
///     instruction. While this flag is set the processor will not respond to interrupts from devices
///     kuntil it is cleared by a 'Clear Interrupt Disable' (CLI) instruction.
///
/// - D  : Decimal Mode
///     While the decimal mode flag is set the processor will obey the rules of Binary Coded Decimal
///     (BCD) arithmetic during addition and subtraction. The flag can be explicitly set using 'Set
///     Decimal Flag' (SED) and cleared with 'Clear Decimal Flag' (CLD).
///
/// - B  : Break Command
///     The break command bit is set when a BRK instruction has been executed and an interrupt has
///     been generated to process it.
///
/// - V  : Overflow
///     The overflow flag is set during arithmetic operations if the result has yielded an invalid
///     2's complement result (e.g. adding to positive numbers and ending up with a negative
///     result: 64 + 64 => -128). It is determined by looking at the carry between bits 6 and 7 and
///     between bit 7 and the carry flag.
///
/// - N  : Negative Flag
///     The negative flag is set if the result of the last operation had bit 7 set to a one.
#[derive(Debug, Clone)]
pub struct Status {
    register: u8,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Flag {
    N = 7,
    V = 6,
    B = 4,
    D = 3,
    I = 2,
    Z = 1,
    C = 0,
}

#[allow(dead_code)]
impl Status {
    fn read_bit(&self, pos: u8) -> u8 {
        self.register & (1 << pos)
    }

    pub fn set_bit(&mut self, bit: Flag) {
        self.register |= 1 << (bit as u8);
    }

    pub fn unset_bit(&mut self, bit: Flag) {
        self.register &= !(1 << (bit as u8));
    }

    pub fn toggle_bit(&mut self, bit: Flag) {
        self.register ^= 1 << (bit as u8)
    }
    pub fn register(&self) -> u8 {
        self.register
    }

    // Read N Flag : Bit 7
    // # Return
    //  Returns 1 or 0 if the flag is set or unset respectively.
    pub fn n(&self) -> u8 {
        self.read_bit(7)
    }

    // Read V Flag : Bit 6
    // # Return
    //  Returns 1 or 0 if the flag is set or unset respectively.
    pub fn v(&self) -> u8 {
        self.read_bit(6)
    }

    // Read B Flag : Bit 4
    // # Return
    //  Returns 1 or 0 if the flag is set or unset respectively.
    pub fn b(&self) -> u8 {
        self.read_bit(4)
    }

    // Read D Flag : Bit 3
    // # Return
    //  Returns 1 or 0 if the flag is set or unset respectively.
    pub fn d(&self) -> u8 {
        self.read_bit(3)
    }

    // Read I Flag : Bit 2
    // # Return
    //  Returns 1 or 0 if the flag is set or unset respectively.
    pub fn i(&self) -> u8 {
        self.read_bit(2)
    }

    // Read Z flag : Bit 1
    // # Return
    //  Returns 1 or 0 if the flag is set or unset respectively.
    pub fn z(&self) -> u8 {
        self.read_bit(1)
    }

    // Read C Flag : Bit 0
    // # Return
    //  Returns 1 or 0 if the flag is set or unset respectively.
    pub fn c(&self) -> u8 {
        self.read_bit(0)
    }
}

#[allow(dead_code)]
impl CPU {
    pub fn new() -> Self {
        Self {
            reg_a: 0,
            reg_x: 0,
            status: Status { register: 0x00 },
            pc: 0,
        }
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn status_mut(&mut self) -> &mut Status {
        &mut self.status
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        'instruction_cycle: loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;
            match opcode {
                // BREAK
                0x00 => break 'instruction_cycle,

                // LDA - LoaD Accumulator +
                0xA9 => {
                    let param = program[self.pc as usize];
                    self.pc += 1;
                    self.reg_a = param;
                    self.update_nf_flags(self.reg_a);
                }

                // TAX - Transfer Accumulator to X
                0xAA => {
                    self.reg_x = self.reg_a;
                    self.update_nf_flags(self.reg_x);
                }

                // INX - INcrement X register
                0xE8 => {
                    self.reg_x = self.reg_x.wrapping_add(1); // Integer Overflow is OK here.
                    self.update_nf_flags(self.reg_x);
                }

                other => todo!("Unexpected Integer: {other:b}"),
            }
        }
    }

    fn update_nf_flags(&mut self, result: u8) {
        let status = self.status_mut();

        if result == 0 {
            status.set_bit(Flag::Z);
        } else {
            status.unset_bit(Flag::Z);
        }
        if result & 0b1000_0000 != 0 {
            status.set_bit(Flag::N)
        } else {
            status.unset_bit(Flag::N);
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.reg_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.reg_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 1)
    }

    #[test]
    #[should_panic]
    fn test_unknown_case() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0x5, 0x00]);
    }
}
