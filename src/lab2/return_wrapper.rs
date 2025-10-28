//return_wrapper.rs: provides a ReturnWrapper for the main function implenting the Termination Trait. Johnny Huang, Aman Verma, Hanson Li

use std::process::{ExitCode, Termination};
use std::io::{self, Write};

pub struct ReturnWrapper {
    pub field_type: u8,
}

impl ReturnWrapper {
    pub fn new(value: u8) -> Self {
        Self { field_type: value }
    }
}

impl Termination for ReturnWrapper {
    fn report(self) -> ExitCode {
        if self.field_type != 0 {
            // Lock stderr at runtime inside the function
            let mut stderr = io::stderr().lock();
            let _ = writeln!(stderr, "Error: {}", self.field_type);
        }
        ExitCode::from(self.field_type)
    }
}
