use std::sync::atomic::Ordering;
use super::declarations::{WHINGE,GENERATION_FAILURE};
use super::script_gen::grab_trimmed_file_lines; //needed to impoirt this
pub type PlayLines = Vec<(usize, String)>;

pub struct Player{
    pub char_name: String,
    pub char_lines: PlayLines,
    pub cur_entry_idx: usize,
}

impl Player{

    pub fn new(char_name: &String) -> Self {
         Self {
            char_name: char_name.to_string(),
            char_lines: Vec::new(),
            cur_entry_idx : 0,
         }
    }


    fn add_script_line(&mut self, unparsed_line: &String){
        if unparsed_line.len() > 0 {
            if let Some((first_token, remain_token)) = unparsed_line.split_once(char::is_whitespace) {
                let line_extract = remain_token.trim(); //this will return &str, so we need to_string when pushing to Play
                
                //using if let for error handling. Base case is if parse returns anything other than Ok
                if let Ok(line_num) = first_token.parse::<usize>() {
                self.char_lines.push((line_num, line_extract.to_string()));
                } else {
                    if WHINGE.load(Ordering::SeqCst){
                        eprintln!("Error: the first token of the passed in line '{}' does not represent a valid usize value!", unparsed_line);
                    }

                }
            }
        }
    }

    pub fn prepare(&mut self, part_name: &String) -> Result<(), u8> {


            
        let mut cur_file_line_vec: Vec::<String> = Vec::new();
        if let Err(e_code) = grab_trimmed_file_lines(&part_name, &mut cur_file_line_vec) {
            println!("Error: process_config unsucessfully called grab_trimmed_file_lines with error code {}", e_code);
            return Err(GENERATION_FAILURE);
        } 

        for a_line in cur_file_line_vec.iter() {
            self.add_script_line(a_line)
        }

        Ok (())
    }

    pub fn speak(&mut self, most_recent_speaker: &mut String){
        if self.cur_entry_idx < self.char_lines.len(){
            //check if passed in name same as struct char name
            if *most_recent_speaker != self.char_name {
                *most_recent_speaker = self.char_name.to_string();
                println!();
                println!("Speaker: {}", most_recent_speaker);
            }
            
            //'either case should print out text and inc index'
            println!("{:?}", self.char_lines[self.cur_entry_idx]);
            self.cur_entry_idx += 1;


        } 
        //simply return if lines >= lines. So if the lines are already past the players lines, we do nothing
    }  

    //dd an associated public next_line method to the implementation block for the Player struct, 
    // which takes an immutable reference to itself and returns an Option<usize>.
    //  The method should check whether the Player struct's index is less than the number of elements in its PlayLines container: 
    // if it is, the method should return Some() with the line number at the current index; if it is not, 
    // the method should return None.

    pub fn next_line(&self) -> Option<usize> {
        if self.cur_entry_idx < self.char_lines.len(){
            return Some(self.char_lines[self.cur_entry_idx].0)
        }else{
            return None
        }
    } 


}