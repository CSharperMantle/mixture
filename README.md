# `mixture`: MIX infrastructure

## Introduction

`mixture` provides a robust simulation environment for MIX computers
used extensively in *The Art of Computer Programming* series written
by D. E. Knuth.

This crate sports:

* MIX simulation via [`MixVM`]
* I/O device simulation via [`IODevice`] (enabled by `io` feature)
* `#[no_std]` compatibility

## Crate features

* `std` - Enable `std` support.
    * `io` - Enable I/O module of MIX.
* `x-ieee754` - Enable IEEE 754-compatible floating-point extension.
* `x-binary` - Enable binary operation extension (TAOCP Section 4.5.2).

All features are enabled by default.

## Example

This example is a simple program that writes a hello message to `stdout`. It demonstrates basic
workflow of creating, initializing, adding I/O device to, and running a MIX virtual machine
with `mixture`.

```rust
use mixture::{ErrorCode, FullWord, IODevice, Instruction, MixVM, Opcode};

// Define common constants
const ADDR_TEXT_HELLO: i16 = 2000;
const DEV_PRINTER: u8 = 18;

// Define a line printer I/O device; this is not necessary: results can be read
// from VM memory directly, but using an I/O device is more illustrative.
struct LinePrinter {}

impl LinePrinter {
    const BLOCK_SIZE: usize = 3;
}

impl IODevice for LinePrinter {
    fn read(&mut self, _: &mut [FullWord]) -> Result<(), ()> {
        // Our device is a printer, so it never reads.
        Err(())
    }

    fn write(&mut self, data: &[FullWord]) -> Result<(), usize> {
        if data.len() != Self::BLOCK_SIZE {
            // Show that we have not printed anything before we bail out.
            return Err(0);
        }
        for i in 0..Self::BLOCK_SIZE {
            let bytes = &data[i][1..=5]; // Ignore sign byte.
            for &b in bytes {
                print!("{}", char::from_u32(b as u32).unwrap_or('?'));
            }
        }
        Ok(())
    }

    fn control(&mut self, _: i16) -> Result<(), ()> {
        // Our device has no special commands defined.
        Ok(())
    }

    fn is_busy(&self) -> Result<bool, ()> {
        Ok(false)
    }

    fn is_ready(&self) -> Result<bool, ()> {
        Ok(true)
    }

    fn get_block_size(&self) -> usize {
        Self::BLOCK_SIZE
    }
}

fn main() {
    // Create a VM
    let mut mix = MixVM::new();
    mix.reset();

    // Fill in some instructions
    // 0  OUT  ADDR_TEXT_HELLO(DEV_PRINTER)
    // 1  HLT  0
    mix.mem[0] = Instruction::new(ADDR_TEXT_HELLO, DEV_PRINTER, 0, Opcode::Out).into();
    mix.mem[1] = Instruction::new(0, 2, 0, Opcode::Special).into();

    // Fill in constants
    mix.mem[ADDR_TEXT_HELLO as u16] =
        FullWord::from_bytes([FullWord::POS, b'H', b'E', b'L', b'L', b'O']);
    mix.mem[ADDR_TEXT_HELLO as u16 + 1] =
        FullWord::from_bytes([FullWord::POS, b' ', b'W', b'O', b'R', b'L']);
    mix.mem[ADDR_TEXT_HELLO as u16 + 2] =
        FullWord::from_bytes([FullWord::POS, b'D', b'!', b' ', b'\r', b'\n']);

    // Attach device
    mix.io_devices[DEV_PRINTER as usize] = Some(Box::new(LinePrinter {}));

    // Start the VM
    mix.restart();

    // Run main loop
    let error_code = loop {
        let result = mix.step();
        match result {
            Ok(()) => continue,
            Err(c) => break c,
        }
    };

    // Check if the VM has shut down gracefully.
    // Now you should see "HELLO WORLD! " with line break in stdout.
    assert_eq!(error_code, ErrorCode::Halted);
}
```

## The MIX ISA

> MIX is the world's first polyunsaturated computer. Like most machines, it has
> an identifying number - the 1009. This number was found by taking 16 actual
> computers very similar to MIX and on which MIX could easily be simulated, then
> averaging their numbers with equal weight:
>
> ⌊(360 + 650 + 709 + 7070 + U3 + SS80 + 1107 + 1604 + G20 + B220 + S2000 + 920 + 601 + H800 + PDP-4 + II)⌋ / 16 = 1009.
>
> The same number may also be obtained in a simpler way by taking Roman
> numerals.
>
> D. E. Knuth, *The Art of Computer Programming (Volume 1, 3rd. ed.)*

The MIX computers are a model computer architecture, including hardware
abstractions and instruction set architecture (ISA), proposed by D. E.
Knuth. Bearing many features from the 1960s and 1970s, it is now considered
obsolete, but it still provides adequate context for ISA development.

### Words

The basic unit of memory in MIX is a *byte*. Note that in MIX, each byte could
contain *arbitrary* amount of information. MIX only places a lower limit on byte
size: A byte must be able to hold at least 64 distinct values. *A sound algorithm
implementation in MIX should work properly regardless of how big a byte is.*

> [!NOTE]
> **Specific to `mixture`:** The byte size in `mixture` implementation is 256.

Two adjacent bytes can express the numbers 0 through 4,095.<br>
Three adjacent bytes can express the numbers 0 through 262,143.<br>
Four adjacent bytes can express the numbers 0 through 16,777,215.<br>
Five adjacent bytes can express the numbers 0 through 1,073,741,823.<br>

A *computer word* consists of five bytes and a sign. The sign portion has only two
possible values, `+` and `-`.

> [!NOTE]
> **Specific to `mixture`:** The sign in `mixture` is also encoded in a byte, whose
> only valid content is [`Word<N, P>::POS`] and [`Word<N, P>::NEG`]. Other
> values written into the sign byte are considered undefined.

### Machine states

There are nine registers in MIX.

* `A`-register (Accumulator) consists of five bytes and a sign, used in most operations as
  operand.
* `X`-register (Extension), likewise, comprises five bytes and a sign, used in some operations
  as an extension to `A`-register at more significant digits.
* `I`-registers (Index registers) `I1`, `I2`, `I3`, `I4`, `I5`, and `I6` each hold two
  bytes together with a sign, used often as indices while counting and addressing.
* `J`-register (Jump address) holds two bytes; it behaves as if its sign is
  always `+`, used as return address while performing jumps.

Each register is denoted with a prefix '`r`' added to its name, e.g. '`rA`' for 'register
`A`'.

> [!NOTE]
> **Specific to `mixture`:** `rA` and `rX` have a type of [`FullWord`]. `rI1-6`
> have a type of [`HalfWord`]. `rJ` has a type of [`PosHalfWord`]. All three types
> are aliases for different instantiations of [`Word<N, P>`].

MIX has some more states that are not registers, and could only be manipulated through
specific instructions:

* An *overflow toggle* (flag for overflow);
* A *comparison indicator* (having three values: `LESS`, `EQUAL` or `GREATER`);
* A *memory* (4000 words of storage, each containing five bytes and a sign);
* *input-output devices* (cards, tapes, disks, etc.).

> [!NOTE]
> **Specific to `mixture`:** There are several implementation-specific details of the above
> states.
>
> * The overflow toggle is a private field that is not exposed to users.
> * The comparison indicator could contain values in the enum [`CompIndicator`].
> * The memory area is a struct named [`Mem`].
> * I/O devices are only available while crate feature `io` is enabled, and are described by
>   dynamic trait [`IODevice`].

### Partial fields of words

The bytes and sign are indexed as follows:

|   0   |   1   |   2   |   3   |   4   |   5   |
| :---: | :---: | :---: | :---: | :---: | :---: |
|   ±   | Byte  | Byte  | Byte  | Byte  | Byte  |

MIX programmers are allowed to use only part of a word in most instructions. In such cases,
a *field specification* could be given to denote part of a word. Given a range `(L:R)`, it
interprets bytes from `L` to `R` (both inclusive) as a whole. Examples of field specifications
are:

* `(0:0)`, the sign only.
* `(0:2)`, the sign and the first two bytes.
* `(0:5)`, the whole word; this is the most common field specification.
* `(1:5)`, the whole word except for the sign.
* `(4:4)`, the fourth byte only.
* `(4:5)`, the two least significant bytes.

> [!NOTE]
> **Specific to `mixture`:** [`Word`][Word<N, P>]s are able to be indexed by both [`usize`]
> scalars and [`core::ops::RangeInclusive<usize>`] ranges, so as to match the semantics proposed
> in the book.

The use of field specification varies among instructions. When encoded in an instruction,
field specification is packed into one-byte scalar equals to `8 * L + R`.

> [!NOTE]
> **Specific to `mixture`:** There is a trait named [`ToRangeInclusive<T>`] to simplify
> this conversion process. It is already implemented for [`u8`] by default. 

### Instruction format

Each instruction in MIX could fit in a single word. The encoding of instructions is as follows:

|   0   |   1   |   2   |   3   |   4   |   5   |
| :---: | :---: | :---: | :---: | :---: | :---: |
|   ±   |  `A`  |  `A`  |  `I`  |  `F`  |  `C`  |

* `C`: the *operation code* specifying what operation is to be performed. For example, `C = 8`
  specifies the operation [`LDA`][Opcode::LdA], "load the register A".

> [!NOTE]
> **Specific to `mixture`:** See [`Opcode`] for a complete list of supported opcodes.

* `F`: the *modification* or *field* of an instruction. It is usually a packed field
  specification said above. For example, if `C = 8` and `F = 11`, the operation would be "load
  the register A with `(1:3)` field." Sometimes `F` is used to specify additional parameters,
  as it is on I/O instructions; in other cases it further refines the operation to carry out,
  as on [register modification][Opcode::ModifyA] instruction series.
* `I`: the *index* part of an instruction. It must be an integer in the range between 0 and 6
  (both inclusive).
* `A`: the *address* of the instruction. Note that the sign is part of the address.

The use of `I` and `A` will be discussed in the [Addressing](#addressing) section.

#### Addressing

Memory addressing in MIX utilizes the `A` and `I` fields in *every* instruction. During
instruction decoding, the target memory location `M` is produced using these two fields. `M`
refers to some memory location at most times, and in this sense, it must be in the range of
0 to 3999 due to the number of memory cells in MIX. The notation `CONTENT(M)` is used in TAOCP
to denote the content of memory location `M`.

If `I = 0`, then direct addressing is used, which means `M` is set to `A` without change.
Otherwise, if `I = i` (where `i` is an integer between 1 and 6), the content of index register
`rIi` is added algebraically to `A` to produce `M`, resembling a base-offset addressing mode.

> [!NOTE]
> **Specific to `mixture`:** If `I` is out of valid range, the VM will halt, and
> [`ErrorCode::InvalidIndex`] is returned as error.

If the produced `M` does not fit in two bytes, the result is undefined.

> [!NOTE]
> **Specific to `mixture`:** `mixture` uses [`u16`] for addresses, and will (depending on build
> flags) panic or silently wrap should any numerical overflow or underflow happens.

### Operators

For a brief sketch of the operators, see the inline documentation of variants in [`Opcode`].

For a detailed explanation of instruction semantics and effects, please refer to
*The Art of Computer Programming (Volume 1, 3rd. ed.)* by D. E. Knuth, or visit
[Esolang/MIX_(Knuth)](https://esolangs.org/wiki/MIX_(Knuth)).

## Extensions to MIX

> [!NOTE]
> **Specific to `mixture`:** This section decribes features exclusive to `mixture`
> and/or implemented in a non-TAOCP way.

### `x-ieee754`: IEEE 754-compatible floating-point arithmetic

This extension adds support for `binary32` floating-point format as described in
[IEEE 754-2008](https://standards.ieee.org/ieee/754/4211/) and Rust [`f32`].

#### Data layout

A `binary32` scalar fully occupies a [`FullWord`]. When loading `binary32` scalars
in MIX, the only valid field specification is `(0:5)`. Loading only a part of a
[`FullWord`], or manipulating any byte in a `binary32` scalar with instructions
not prefixed with `F32` will yield undefined result.


|   0   |   1   |   2   |   3   |   4   |   5   |
| :---: | :---: | :---: | :---: | :---: | :---: |
|   ±   |   X   |  `f`  |  `f`  |  `f`  |  `f`  |

In reality, only `(2:5)` part of a [`FullWord`] is used to store a `binary32` scalar.
The sign byte is only set while storing computation results; its value is not honored when reading
computation operands. Thus, such a wrongly-signed word representing the quantity `+1.0f32` will
never be constructed by `mixture`:

|   0   |   1   |   2    |   3    |   4    |   5    |
| :---: | :---: | :----: | :----: | :----: | :----: |
| **-** |   X   | `0x3F` | `0x80` | `0x00` | `0x00` |

but can be created by hand. If such a quantity is used in a calculation, its actual value will be
`+1.0f32`, rather than `-1.0f32`.

#### Conversion instructions

* `F32CVTF322I4B` ([`Opcode::Special`], `F = 3`): Convert and round IEEE 754 `binary32`
  to 4-bytes integer.
* `F32CVTF322I2B` ([`Opcode::Special`], `F = 4`): Convert and round IEEE 754 `binary32`
  to 2-bytes integer.
* `F32CVTF322I1B` ([`Opcode::Special`], `F = 5`): Convert and round IEEE 754 `binary32`
  to 1-byte integer.
* `F32CVTI4B2F32` ([`Opcode::Special`], `F = 6`): Convert 4-bytes integer to IEEE 754 `binary32`.
* `F32CVTI2B2F32` ([`Opcode::Special`], `F = 7`): Convert 2-bytes integer to IEEE 754 `binary32`.
* `F32CVTI1B2F32` ([`Opcode::Special`], `F = 8`): Convert 1-byte integer to IEEE 754 `binary32`.

These instructions convert source scalars stored in `rA` into destination type, and store the
result in `rA(0:5)`.

For integer-to-`binary32`, the source is fetched from least significant byte and is always
considered positive. For example, if `rA` contains:

|   0   |   1   |   2   |   3   |   4   |   5   |
| :---: | :---: | :---: | :---: | :---: | :---: |
|   -   |  `9`  |  `8`  |  `7`  |  `6`  |  `5`  |

performing `F32CVTI1B2F32` will yield `binary32` representation of `+5` in `rA(0:5)`.

|   0   |   1   |   2    |   3    |   4    |   5    |
| :---: | :---: | :----: | :----: | :----: | :----: |
|   +   |   X   | `0x40` | `0xA0` | `0x00` | `0x00` |

##### Special result behavior

When the source scalar is outside the destination type's representable range, the overflow toggle
be turned on and source will be clamped to destination type.

Converting a NaN to integer will result in overflow toggle being turned on. The result will be zero.

#### Arithmetic instructions

* `F32ADD` ([`Opcode::Add`], `F = 7`): IEEE 754 `binary32` addition.
* `F32SUB` ([`Opcode::Sub`], `F = 7`): IEEE 754 `binary32` subtraction.
* `F32MUL` ([`Opcode::Mul`], `F = 7`): IEEE 754 `binary32` multiplication.
* `F32DIV` ([`Opcode::Div`], `F = 7`): IEEE 754 `binary32` division.

These instrutions takes `rA` as left operand and `V` as right operand. Result is stored in `rA`.

##### Special result behavior

If the calculation result is either NaN or ±Infinity, overflow toggle will be turned on. The result
will still be stored as-is in `rA`.

#### Comparison instructions

* `F32CMPA` ([`Opcode::CmpA`], `F = 7`): Compare `rA` with `V` as `binary32` values.
* `F32CMPX` ([`Opcode::CmpX`], `F = 7`): Compare `rX` with `V` as `binary32` values.

Compare `rA` or `rX` against `V` as `binary32` values. Stores result in comparison indicator.

Note that NaNs are unordered against any other values.

##### Special result behavior

Trivial.

#### Jump instructions

The regular [`Opcode::Jmp`] variants also works for `binary32` comparisons.

* `F32JORD` ([`Opcode::Jmp`], `F = 11`): Jump on ordered.
* `F32JUNORD` ([`Opcode::Jmp`], `F = 12`): Jump on unordered.

Perform jump according to last `binary32` comparison result.

##### Special result behavior

Trivial.

### `x-binary`: Binary operations (TAOCP Section 4.5.2)

This extension adds support for binary operations to registers and words, allowing
for bit-wise shifts and conditional jumps on final bit of registers (even/odd).

#### Shift instructions

* `SLB` ([`Opcode::Shift`], `F = 6`): Shift left `rAX` binary.
  The contents of `rA` and `rX` are shifted left `M` binary places.
  The signs of `rA` and `rX` are not affected.)
* `SRB` ([`Opcode::Shift`], `F = 7`): Shift right `rAX` binary. `C = 6`; `F = 7`.
  The contents of `rA` and `rX` are shifted right `M` binary places.

#### Jump instructions

* `JAE` ([`Opcode::JA`], `F = 6`): Jump `rA` even.
* `JAO` ([`Opcode::JA`], `F = 7`): Jump `rA` odd.
* `JXE` ([`Opcode::JX`], `F = 6`): Jump `rX` even.
* `JXO` ([`Opcode::JX`], `F = 7`): Jump `rX` odd.

These instructions look at the least significant bit in `rA` and `rX`, performing jumps
according to its oddity.

### `x-binarith`: Binary arithmetic instructions

This extension adds support for binary arithmetics, resembling the classical 'bit operations' in
various programming languages. These operations are indeed not authentic MIX (which requires
operations to be independent of byte size and numerical base), so they are categorized under
[`Opcode::Special`] sections.

#### Arithmetic instructions

* `NOT` ([`Opcode::Special`], `F = 9`): Perform bitwise NOT on `rA`, then store result in `rA`.
* `AND` ([`Opcode::Special`], `F = 10`): Perform bitwise AND on `V` and `rA`, then store result in `rA`.
* `OR` ([`Opcode::Special`], `F = 11`): Perform bitwise OR on `V` and `rA`, then store result in `rA`.
* `XOR` ([`Opcode::Special`], `F = 12`): Perform bitwise XOR on `V` and `rA`, then store result in `rA`.

#### Sign treatment

In every operations listed in this extension, [`Word<N, P>::POS`] is treated as `0x0`, while
[`Word<N, P>::NEG`] is treated as `0x1`.
