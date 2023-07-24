# `mixture`: MIX infrastructure

## Introduction

`mixture` provides a robust simulation environment for MIX computers
used extensively in *The Art of Computer Programming* series written
by D. E. Knuth.

Crate highlights:

* MIX simulation via [`sim::MixVM`]
* I/O device simulation via [`sim::IODevice`] (enabled by `io` feature)
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
use mixture::sim::{ErrorCode, FullWord, IODevice, Instruction, MixVM, Opcode};

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
    mix.mem[0] = Instruction::new(ADDR_TEXT_HELLO, DEV_PRINTER, 0, Opcode::Out)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(0, 2, 0, Opcode::Special)
        .try_into()
        .unwrap();

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

> **Specific to `mixture`:** The byte size in `mixture` implementation is 256.

Two adjacent bytes can express the numbers 0 through 4,095.<br>
Three adjacent bytes can express the numbers 0 through 262,143.<br>
Four adjacent bytes can express the numbers 0 through 16,777,215.<br>
Five adjacent bytes can express the numbers 0 through 1,073,741,823.<br>

A *computer word* consists of five bytes and a sign. The sign portion has only two
possible values, `+` and `-`.

> **Specific to `mixture`:** The sign in `mixture` is also encoded in a byte, whose
> only valid content is [`sim::Word<N, P>::POS`] and [`sim::Word<N, P>::NEG`]. Other
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

> **Specific to `mixture`:** `rA` and `rX` have a type of [`sim::FullWord`]. `rI1-6`
> have a type of [`sim::HalfWord`]. `rJ` has a type of [`sim::PosHalfWord`]. All three types
> are aliases for different instantiations of [`sim::Word<N, P>`].

MIX has some more states that are not registers, and could only be manipulated through
specific instructions:

* An *overflow toggle* (flag for overflow);
* A *comparison indicator* (having three values: `LESS`, `EQUAL` or `GREATER`);
* A *memory* (4000 words of storage, each containing five bytes and a sign);
* *input-output devices* (cards, tapes, disks, etc.).

> **Specific to `mixture`:** There are several implementation-specific details of the above
> states.
>
> * The overflow toggle is a private field that is not exposed to users.
> * The comparison indicator could contain values in the enum [`sim::CompIndicator`].
> * The memory area is a struct named [`sim::Mem`].
> * I/O devices are only available while crate feature `io` is enabled, and are described by
>   dynamic trait [`sim::IODevice`].

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

> **Specific to `mixture`:** [`Word`][sim::Word<N, P>]s are able to be indexed by both [`usize`]
> scalars and [`core::ops::RangeInclusive<usize>`] ranges, so as to match the semantics proposed
> in the book.

The use of field specification varies among instructions. When encoded in an instruction,
field specification is packed into one-byte scalar equals to `8 * L + R`.

> **Specific to `mixture`:** There is a trait named [`sim::ToRangeInclusive<T>`] to simplify
> this conversion process. It is already implemented for [`u8`] by default. 

### Instruction format

Each instruction in MIX could fit in a single word. The encoding of instructions is as follows:

|   0   |   1   |   2   |   3   |   4   |   5   |
| :---: | :---: | :---: | :---: | :---: | :---: |
|   ±   |  `A`  |  `A`  |  `I`  |  `F`  |  `C`  |

* `C`: the *operation code* specifying what operation is to be performed. For example, `C = 8`
  specifies the operation [`LDA`][sim::Opcode::LdA], "load the register A".

> **Specific to `mixture`:** See [`sim::Opcode`] for a complete list of supported opcodes.

* `F`: the *modification* or *field* of an instruction. It is usually a packed field
  specification said above. For example, if `C = 8` and `F = 11`, the operation would be "load
  the register A with `(1:3)` field." Sometimes `F` is used to specify additional parameters,
  as it is on I/O instructions; in other cases it further refines the operation to carry out,
  as on [register modification][sim::Opcode::ModifyA] instruction series.
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

> **Specific to `mixture`:** If `I` is out of valid range, the VM will halt, and
> [`sim::ErrorCode::InvalidIndex`] is returned as error.

If the produced `M` does not fit in two bytes, the result is undefined.

> **Specific to `mixture`:** `mixture` uses [`u16`] for addresses, and will (depending on build
> flags) panic or silently wrap should any numerical overflow or underflow happens.

### Operators

For a brief sketch of the operators, see the inline documentation of variants in [`sim::Opcode`].

For a detailed explanation of instruction semantics and effects, please refer to
*The Art of Computer Programming (Volume 1, 3rd. ed.)* by D. E. Knuth, or visit
[Esolang/MIX_(Knuth)](https://esolangs.org/wiki/MIX_(Knuth)).

### Extensions to MIX

#### `x-ieee754`: IEEE 754-compatible floating-point arithmetic

WIP.

#### `x-binary`: Binary operations (TAOCP Section 4.5.2)

* `SLB`: Shift left `rAX` binary. `C = 6`; `F = 6`.
  The contents of `rA` and `rX` are shifted left `M` binary places.
  The signs of `rA` and `rX` are not affected.)
* `SRB`: Shift right `rAX` binary. `C = 6`; `F = 7`.
  The contents of `rA` and `rX` are shifted right `M` binary places.

* `JAE`: Jump `rA` even. `C = 40`; `F = 6`.
* `JAO`: Jump `rA` odd. `C = 40`; `F = 7`.
* `JXE`: Jump `rX` even. `C = 47`; `F = 6`.
* `JXO`: Jump `rX` odd. `C = 47`; `F = 7`.

> **Specific to `mixture`:** See [`sim::Opcode::Shift`], [`sim::Opcode::JA`] and
> [`sim::Opcode::JX`] for details.
