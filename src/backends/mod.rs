pub mod x86;

pub trait Backend {
	const registers: &'static [&'static str];
	const instruction: &'static [&'static str];
}
