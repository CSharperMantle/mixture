use crate::sim::mix::*;

/// An instruction of the MIX machine.
pub struct Instruction {
    /// The signed address, `A`, read little-endian.
    pub addr: i16,

    /// The field, `F`.
    pub field: u8,

    /// The index, `I`.
    pub index: u8,

    /// The operation code, `C`.
    pub opcode: Opcode,
}

impl Instruction {
    /// Create a new instruction.
    /// 
    /// # Arguments
    /// * `addr` - The signed address, `A`, read little-endian.
    /// * `field` - The field, `F`.
    /// * `index` - The index, `I`.
    /// * `opcode` - The operation code, `C`.
    /// 
    /// # Example
    /// ```rust
    /// use mixture::sim::mix::instr::*;
    /// 
    /// let instr = Instruction::new(2000, 0x03, 0x02, Opcode::Lda);
    /// assert_eq!(instr.addr, 2000);
    /// assert_eq!(instr.field, 0x03);
    /// assert_eq!(instr.index, 0x02);
    /// assert_eq!(instr.opcode, Opcode::Lda);
    /// ```
    pub fn new(addr: i16, field: u8, index: u8, opcode: Opcode) -> Self {
        Instruction {
            addr,
            field,
            index,
            opcode,
        }
    }
}

impl std::convert::TryFrom<mem::Word<6, false>> for Instruction {
    type Error = &'static str;

    /// Convert a `Word<6, false>` to an `Instruction`.
    ///
    /// # Arguments
    /// * `source` - The `Word<6, false>` to convert.
    ///
    /// # Returns
    /// * `Ok(Instruction)` - The conversion was successful.
    /// * `Err(&'static str)` - The conversion failed.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::mix::mem::*;
    /// use mixture::sim::mix::instr::*;
    ///
    /// let mut word = Word::<6, false>::new();
    /// word.set(0..=5, &[0, 0xD0, 0x07, 0x02, 0x03, 0x08]).unwrap();
    ///
    /// let instr = Instruction::try_from(word).unwrap();
    /// assert_eq!(instr.opcode, Opcode::Lda);
    /// assert_eq!(instr.field, 0x03);
    /// assert_eq!(instr.index, 0x02);
    /// assert_eq!(instr.addr, 2000);
    /// ```
    fn try_from(source: mem::Word<6, false>) -> Result<Self, Self::Error> {
        let sign = if source[0] == 0 { 1 } else { -1 };
        let addr = sign * i16::from_le_bytes([source[1], source[2]]);
        let opcode = match Opcode::try_from(source[5..=5][0]) {
            Ok(opcode) => opcode,
            Err(_) => return Err("Invalid opcode"),
        };
        Ok(Instruction {
            opcode,
            field: source[4..=4][0],
            index: source[3..=3][0],
            addr,
        })
    }
}

/// All possible operation codes.
#[derive(Debug, Eq, PartialEq, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Nop = 0,
    AddFadd = 1,
    SubFsub = 2,
    MulFmul = 3,
    DivFdiv = 4,
    NumCharHlt = 5,
    SlaSraSlaxSraxSlcSrc = 6,
    Move = 7,
    Lda = 8,
    Ld1 = 9,
    Ld2 = 10,
    Ld3 = 11,
    Ld4 = 12,
    Ld5 = 13,
    Ld6 = 14,
    Ldx = 15,
    Ldan = 16,
    Ld1n = 17,
    Ld2n = 18,
    Ld3n = 19,
    Ld4n = 20,
    Ld5n = 21,
    Ld6n = 22,
    Ldxn = 23,
    Sta = 24,
    St1 = 25,
    St2 = 26,
    St3 = 27,
    St4 = 28,
    St5 = 29,
    St6 = 30,
    Stx = 31,
    Stj = 32,
    Stz = 33,
    Jbus = 34,
    Ioc = 35,
    In = 36,
    Out = 37,
    Jred = 38,
    JmpJsjJovJnov = 39,
    IncaDecaEntaEnna = 40,
    Inc1Dec1Ent1Enn1 = 41,
    Inc2Dec2Ent2Enn2 = 42,
    Inc3Dec3Ent3Enn3 = 43,
    Inc4Dec4Ent4Enn4 = 44,
    Inc5Dec5Ent5Enn5 = 45,
    Inc6Dec6Ent6Enn6 = 46,
    IncxDecxEntxEnnx = 47,
    CmpaFcmp = 48,
    Cmp1 = 49,
    Cmp2 = 50,
    Cmp3 = 51,
    Cmp4 = 52,
    Cmp5 = 53,
    Cmp6 = 54,
    Cmpx = 55,
}
