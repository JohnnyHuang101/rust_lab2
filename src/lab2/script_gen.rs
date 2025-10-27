//refactored script_gen.rs. Provides the grab_trimmed_file_lines used by other rs files to read in lines from a speak file. Aman Verma, Johnny Huang, Hanson Li

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use super::declarations::GENERATION_FAILURE;
use std::io::{self, Write};

pub fn grab_trimmed_file_lines(file_name: &String, file_line_vec: &mut Vec<String>) -> Result<(), u8>{
    //note: in config files you must provide the relative or full path to the speak files
    //using match since error code could be helpful here

    let mut stdout = std::io::stdout().lock();
    match File::open(file_name) {
        Ok(file_obj) => {
            let mut buf_reader = BufReader::new(file_obj);
            let mut cur_read_str = String::new();

            loop {
 
                cur_read_str.clear(); //clear first, then read_line
                match buf_reader.read_line(&mut cur_read_str) {
                    Ok(read_line_status) => {
                        
                        if read_line_status == 0 {
                            break
                        }

                        file_line_vec.push(cur_read_str.trim().to_string()); //trim will return &str so we need to_string
                    },
                    Err(e_code) => {
                        writeln!(stdout,"Error: in grab_trimmed_file_lines, BufReader failed with code {}", e_code);
                        return Err(GENERATION_FAILURE);
                    },
                    
                }

            }
            
            Ok(()) //only arrive here if we break from the loop

        }
        Err(e_code) => {
            writeln!(stdout,"Error: in grab_trimmed_file_lines, failed to open file with error code: {} and file name: {}", e_code, file_name);
            return Err(GENERATION_FAILURE);
        }
    }
}


