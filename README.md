# word-ram

This is an interactive Word-RAM. Hence the command JZ is not supported. But it is possible to copy and paste programs. The register machine can contain a maximum of 256 8-bit registers.

## Example

When the program started and w = 5 was selected, copy the following commands into the program:

```
R1 = 7
R2 = 3
LOAD R1
MUL R2
STORE R3
HALT

```

Result:

```
R1 = 7
|  R0 |  0 |  R1 |  7 |  R2 |  0 |  R3 |  0 |  R4 |  0 |  R5 |  0 |  R6 |  0 |  R7 |  0 |
|  R8 |  0 |  R9 |  0 | R10 |  0 | R11 |  0 | R12 |  0 | R13 |  0 | R14 |  0 | R15 |  0 |
| R16 |  0 | R17 |  0 | R18 |  0 | R19 |  0 | R20 |  0 | R21 |  0 | R22 |  0 | R23 |  0 |
| R24 |  0 | R25 |  0 | R26 |  0 | R27 |  0 | R28 |  0 | R29 |  0 | R30 |  0 | R31 |  0 |
R2 = 3
|  R0 |  0 |  R1 |  7 |  R2 |  3 |  R3 |  0 |  R4 |  0 |  R5 |  0 |  R6 |  0 |  R7 |  0 |
|  R8 |  0 |  R9 |  0 | R10 |  0 | R11 |  0 | R12 |  0 | R13 |  0 | R14 |  0 | R15 |  0 |
| R16 |  0 | R17 |  0 | R18 |  0 | R19 |  0 | R20 |  0 | R21 |  0 | R22 |  0 | R23 |  0 |
| R24 |  0 | R25 |  0 | R26 |  0 | R27 |  0 | R28 |  0 | R29 |  0 | R30 |  0 | R31 |  0 |
LOAD R1
|  R0 |  7 |  R1 |  7 |  R2 |  3 |  R3 |  0 |  R4 |  0 |  R5 |  0 |  R6 |  0 |  R7 |  0 |
|  R8 |  0 |  R9 |  0 | R10 |  0 | R11 |  0 | R12 |  0 | R13 |  0 | R14 |  0 | R15 |  0 |
| R16 |  0 | R17 |  0 | R18 |  0 | R19 |  0 | R20 |  0 | R21 |  0 | R22 |  0 | R23 |  0 |
| R24 |  0 | R25 |  0 | R26 |  0 | R27 |  0 | R28 |  0 | R29 |  0 | R30 |  0 | R31 |  0 |
MUL R2
|  R0 | 21 |  R1 |  7 |  R2 |  3 |  R3 |  0 |  R4 |  0 |  R5 |  0 |  R6 |  0 |  R7 |  0 |
|  R8 |  0 |  R9 |  0 | R10 |  0 | R11 |  0 | R12 |  0 | R13 |  0 | R14 |  0 | R15 |  0 |
| R16 |  0 | R17 |  0 | R18 |  0 | R19 |  0 | R20 |  0 | R21 |  0 | R22 |  0 | R23 |  0 |
| R24 |  0 | R25 |  0 | R26 |  0 | R27 |  0 | R28 |  0 | R29 |  0 | R30 |  0 | R31 |  0 |
STORE R3
|  R0 | 21 |  R1 |  7 |  R2 |  3 |  R3 | 21 |  R4 |  0 |  R5 |  0 |  R6 |  0 |  R7 |  0 |
|  R8 |  0 |  R9 |  0 | R10 |  0 | R11 |  0 | R12 |  0 | R13 |  0 | R14 |  0 | R15 |  0 |
| R16 |  0 | R17 |  0 | R18 |  0 | R19 |  0 | R20 |  0 | R21 |  0 | R22 |  0 | R23 |  0 |
| R24 |  0 | R25 |  0 | R26 |  0 | R27 |  0 | R28 |  0 | R29 |  0 | R30 |  0 | R31 |  0 |
HALT
```

## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
