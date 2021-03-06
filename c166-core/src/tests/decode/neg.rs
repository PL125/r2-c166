test_requires_decode!();

#[test]
fn op_81_1() {
    test_disasm_op!([0x81, 0x10], "neg r1");
}

#[test]
#[ignore]
#[should_panic(expected = "Instruction was invalid")]
fn op_81_2() {
    test_disasm_op!([0x81, 0x11], "invalid");
}
