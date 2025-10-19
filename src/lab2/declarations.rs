use std::sync::atomic::AtomicBool;

pub type Line = (usize, String, String);

//command-line arity & positions
pub const ARGS_MIN: usize = 2;
pub const ARGS_MAX: usize = 3;
pub const ARG_PROGRAM_IDX: usize = 0;
pub const ARG_SCRIPT_IDX: usize = 1;
pub const ARG_WHINGE_IDX: usize = 2;

//exit codes
pub const EXIT_BAD_CMDLINE: u8 = 1;
pub const GENERATION_FAILURE: u8 = 2;

//whinge, default to false
pub static WHINGE: AtomicBool = AtomicBool::new(false);
