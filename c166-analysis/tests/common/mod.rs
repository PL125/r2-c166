use std::process::Command;
use std::borrow::Cow;
use std::env;

pub fn run_radare<'a>(args: &[&str]) -> Cow<'a, str> {
    
    let output = Command::new("radare2")
            .arg("-l")
            .arg(env::var("R2_ASM_PLUGIN").unwrap())
            .arg("-l")
            .arg(env::var("R2_ANALYSIS_PLUGIN").unwrap())
            .args(args)
            .output()
            .expect("failed to execute process");
    let stdout : Cow<'a, str> = String::from_utf8(output.stdout).unwrap().into();
    stdout
}

pub fn r2_version<'a>() -> Cow<'a, str> {
    run_radare(&["-v"])
}

pub fn r2_eval_asm_op<'a>(op: &str, cmd: &str) -> String {
    r2_eval_asm_op_with_init(op, cmd, "")
}

pub fn r2_eval_asm_op_with_init<'a>(op: &str, cmd: &str, init: &str) -> String {
    let r2_cmd : String = format!("s 0; wx {op}; {init}; ds 1; s 0; {cmd}", op=op, cmd=cmd, init=init);
    let ret : Cow<'a, str> = run_radare(&["-2", "-0", "-a", "c166", "-b", "16", "-Q", "-c", r2_cmd.as_str(), "-"]);
    let substr = ( ret[0..(ret.len() - 1)] ).to_string();
    eprintln!("R2: >> {}\nR2: << {}", r2_cmd, substr);
    substr
}

pub fn r2_eval_asm_op_reg(op: &str, reg: &str) -> String {
    r2_eval_asm_op_reg_with_init(op, reg, "")
}

pub fn r2_eval_asm_op_reg_with_init(op: &str, reg: &str, init: &str) -> String {
    let reg_cmd = format!("ar {reg}", reg=reg);
    r2_eval_asm_op_with_init(op, reg_cmd.as_str(), init)
}

pub fn r2_eval_asm_op_indirect_with_init(op: &str, reg: &str, init: &str) -> String {
    let reg_cmd = format!("ae {reg},[]", reg=reg);
    r2_eval_asm_op_with_init(op, reg_cmd.as_str(), init)
}
