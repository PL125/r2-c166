test_requires!();

#[test]
fn c166_op_0c() {
    test_disasm_op!([0x0C, 0x15], "rol r1, r5");
}

#[test]
fn c166_op_1c() {
    test_disasm_op!([0x1C, 0x51], "rol r1, #05h");
}
