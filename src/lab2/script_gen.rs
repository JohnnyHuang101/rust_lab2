// use super::declarations::Line;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::atomic::Ordering;
use super::declarations::{WHINGE,Play,GENERATION_FAILURE};



// pub type PlayConfig = Vec<(String, String)>;

const TITLE_IDX: usize = 0;             // Index of the line giving the title of the play
const PART_FILE_IDX: usize = 1; // Index of the first line containing character info

const CHAR_NAME_POS: usize = 0; // Index of the character's name in a line
const FILE_NAME_TOKEN_POS: usize = 1;      // Index of the file containing the character's lines
const EXPECTED_TOKENS: usize = 2;       // Expected number of tokens in a character line

// pub fn add_script_line(play_vec: &mut Play, unparsed_line: &String, part_name: &String){
//     if unparsed_line.len() > 0 {
//         if let Some((first_token, remain_token)) = unparsed_line.split_once(char::is_whitespace) {
//             let line_extract = remain_token.trim(); //this will return &str, so we need to_string when pushing to Play
            
//             //using if let for error handling. Base case is if parse returns anything other than Ok
//             if let Ok(line_num) = first_token.parse::<usize>() {
//                play_vec.push((line_num, part_name.to_string(), line_extract.to_string()));
//             } else {
//                  if WHINGE.load(Ordering::SeqCst){
//                     eprintln!("Error: the first token of the passed in line '{}' does not represent a valid usize value!", unparsed_line);
//                 }

//             }
//         }
//     }
// }


pub fn grab_trimmed_file_lines(file_name: &String, file_line_vec: &mut Vec<String>) -> Result<(), u8>{
    //we had to prepend the "./data/" string to the file_name
    //using match since error code could be helpful here
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
                        println!("Error: in grab_trimmed_file_lines, BufReader failed with code {}", e_code);
                        return Err(GENERATION_FAILURE);
                    },
                    
                }

            }
            
            Ok(()) //only arrive here if we break from the loop

        }
        Err(e_code) => {
            println!("Error: in grab_trimmed_file_lines, failed to open file with error code: {}", e_code);
            return Err(GENERATION_FAILURE);
        }
    }
}

// pub fn process_config(play_vec: &mut Play, play_cfg: &PlayConfig) -> Result<(), u8> {
//     //note: iter yeilds immutable refs in rusts
//     for a_cfg in play_cfg.iter() {
//         //example from Expressions slide: match t {(x, y) => do_func(x,y);}
//         match a_cfg {(part_name, speak_file) => {
//             let mut cur_file_line_vec: Vec::<String> = Vec::new();
//             if let Err(e_code) = grab_trimmed_file_lines(&speak_file, &mut cur_file_line_vec) {
//                 println!("Error: process_config unsucessfully called grab_trimmed_file_lines with error code {}", e_code);
//                 return Err(GENERATION_FAILURE);
//             } 

//             for a_line in cur_file_line_vec.iter() {
//                 add_script_line(play_vec, a_line, part_name)
//             }

//         }}
//     }
//     Ok (())
// }

pub fn add_config(cfg_line: &String, play_cfg: &mut PlayConfig){
    //split_whitespace gives an iterable, and collect turns that into a collection
    //since using &str, need to do .to_string when inserting into play_cfg because it is of type <String, String>
    let cfg_items: Vec<&str> = cfg_line.split_whitespace().collect(); 
    

    if cfg_items.len() > EXPECTED_TOKENS {
        if WHINGE.load(Ordering::SeqCst) {
            eprintln!("Error: expecting config line to have 2 items but got more than 2 items, pushing first 2 elements");
        }
        play_cfg.push((cfg_items[CHAR_NAME_POS].to_string(), cfg_items[FILE_NAME_TOKEN_POS].to_string()))
    } else if cfg_items.len() < EXPECTED_TOKENS {
        if WHINGE.load(Ordering::SeqCst) {
            eprintln!("Error: expecting config line to have 2 items but got less than 2 items. Not pushing anything");
        }
    } else {
        play_cfg.push((cfg_items[CHAR_NAME_POS].to_string(), cfg_items[FILE_NAME_TOKEN_POS].to_string()))
    }

}

pub fn read_config(cfg_fname: &String, play_title: &mut String, play_cfg: &mut PlayConfig) -> Result<(), u8> {
    let mut cfg_lines: Vec<String> = Vec::new();

    match grab_trimmed_file_lines(&cfg_fname, &mut cfg_lines) {
        Ok(_) => { //don't really need to read the ok code so use _
            if cfg_lines.len() < 2 {
                println!("Error: less than 2 lines from config were read, exiting read_config with error code {}", GENERATION_FAILURE);
                return Err(GENERATION_FAILURE);
            }
            
            *play_title = cfg_lines[TITLE_IDX].to_string();
            // can skip the 1st elem of cfg_lines as it contains the title, using PART_FILE_IDX for this
            for a_cfg_line in cfg_lines.iter().skip(PART_FILE_IDX) {
                //iter should already make a_cfg_line of &String type
                add_config(a_cfg_line, play_cfg)
            }
        },
        Err(e_code) => {
            println!("Error: in read_config, call to grab_trimmed_file_lines failed with error code {}", e_code);
            return Err(GENERATION_FAILURE);
        }

    }

    Ok(())
}


pub fn script_gen(cfg_fname: &String, play_title: &mut String, play_vec: &mut Play) -> Result<(), u8> {
    let mut playcfg_var = PlayConfig::new();
    if let Err(e_code) = read_config(cfg_fname, play_title, &mut playcfg_var) {
        println!("Error: in script_gen, read_config call failed with error code {}", e_code);
        return Err(GENERATION_FAILURE);
    }

    if let Err(e_code) = process_config(play_vec, &playcfg_var) {
        println!("Error: in script_gen, process_config call failed with error code {}", e_code);
        return Err(GENERATION_FAILURE);
    }

    Ok(())
}

