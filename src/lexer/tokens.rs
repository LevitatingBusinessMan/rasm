// https://www.nasm.us/doc/nasmdoc3.html#section-3.4
pub enum TokenType {
	INSTRUCTION, REGISTER,IDENTIFIER,STRING,CHAR,NUMBER,
}

pub struct Token {
	pub ttype: TokenType,
	pub start: usize,
	pub length: u32,
	pub line: u32,
	pub column: u32,
}
