use crate::backends::Backend;

pub struct X86;

impl Backend for X86 {
    const registers: &'static [&'static str] = &["al", "ah", "ax", "eax", "rax"];
    const instruction: &'static [&'static str] = &["add",];
}
