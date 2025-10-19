use std::process::{ExitCode, Termination};

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
            eprintln!("Error FROM THE RETURN WRAPPER: {}", self.field_type);
        }
        ExitCode::from(self.field_type)
    }
}
