test_requires_decode!();

#[test]
fn op_4c() {
    test_disasm_op!([0x4C, 0x45], "shl r4, r5");
}

#[test]
fn op_5c() {
    test_disasm_op!([0x5C, 0x45 ], "shl r5, #04h");
}
