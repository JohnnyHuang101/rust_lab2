//player.rs declares a Player struct that holds information about the character's name, their current spoken line number, and a vector of PlayLines that contains a line number and the text per line. Also contains associated functions for Player that parse and stores lines from the speak files and deliver them. Hanson Li, Aman Verma, Johnny Huang

use std::sync::atomic;
use std::cmp::Ordering;
use super::declarations::{WHINGE,GENERATION_FAILURE, ZERO_IDX};
use super::script_gen::grab_trimmed_file_lines;
pub type PlayLines = Vec<(usize, String)>; //per line, holds information about the line number and the text.

#[derive(Debug)]
pub struct Player{
    pub char_name: String, //character name
    pub char_lines: PlayLines, //vector of tuple of (line number, line text)
    pub cur_entry_idx: usize, //current line number spoken by character
}

impl Player{

    pub fn new(char_name: &String) -> Self {
         Self {
            char_name: char_name.to_string(),
            char_lines: Vec::new(),
            cur_entry_idx : ZERO_IDX,
         }
    }

    //adds a line parsed from self.prepare to our chars_lines vector
    fn add_script_line(&mut self, unparsed_line: &String){
        if unparsed_line.len() > 0 {
            if let Some((first_token, remain_token)) = unparsed_line.split_once(char::is_whitespace) {
                let line_extract = remain_token.trim(); //this will return &str, so we need to_string when pushing to Play
                
                //using if let for error handling. Base case is if parse returns anything other than Ok
                if let Ok(line_num) = first_token.parse::<usize>() {
                self.char_lines.push((line_num, line_extract.to_string()));
                } else {
                    if WHINGE.load(atomic::Ordering::SeqCst){
                        eprintln!("Whinge Warning: the first token of the passed in line '{}' does not represent a valid usize value!", unparsed_line);
                    }

                }
            }
        }
    }

    //read lines and their line number from the speak files
    pub fn prepare(&mut self, part_name: &String) -> Result<(), u8> {


        let mut cur_file_line_vec: Vec::<String> = Vec::new();
        if let Err(e_code) = grab_trimmed_file_lines(&part_name, &mut cur_file_line_vec) {
            println!("Error: process_script unsucessfully called grab_trimmed_file_lines with error code {}", e_code);
            return Err(GENERATION_FAILURE);
        } 

        for a_line in cur_file_line_vec.iter() {
            self.add_script_line(a_line)
        }
        self.char_lines.sort_by_key(|a_tuple| a_tuple.0); //need to use sort by key on the line nume (1st tuple pos) to correctly sort out of order lines
        Ok (())
    }

    //delivers the lines using self.char_lines
    pub fn speak(&mut self, most_recent_speaker: &mut String){
        if self.cur_entry_idx < self.char_lines.len(){
            //check if passed in name same as struct char name
            if *most_recent_speaker != self.char_name {
                *most_recent_speaker = self.char_name.to_string();
                println!();
                println!("Speaker: {}", most_recent_speaker);
            }
            
            //'either case should print out text and inc index'
            println!("{:?}", self.char_lines[self.cur_entry_idx].1);
            self.cur_entry_idx += 1


        } 
    }  
    
    
    //checks if the current character still has a next line, return line num if yes None if doesn't
    pub fn next_line(&self) -> Option<usize> {
        if self.cur_entry_idx < self.char_lines.len(){
            return Some(self.char_lines[self.cur_entry_idx].0)
        }else{
            return None
        }
    } 

}

//partial eq fn sig: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
impl PartialEq for Player{
    fn eq(&self, other: &Self) -> bool{
        if self.char_lines.len() == 0 && other.char_lines.len() == 0{
            return true
        }else if self.char_lines.len() != 0 && other.char_lines.len() != 0{
            return self.char_lines[0].0 == other.char_lines[0].0
        }
        
        return false
    }
}



//how to implement Eq: https://doc.rust-lang.org/std/cmp/trait.Eq.html
impl Eq for Player{}

//PartialOrd fn sig: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
impl PartialOrd for Player{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        if self.char_lines.len() == 0 && other.char_lines.len() != 0 {
            return Some(Ordering::Less)
        }
        if self.char_lines.len() != 0 && other.char_lines.len() == 0 {
            return Some(Ordering::Greater)
        }
        //first line number comparison
        if self.char_lines[0].0 < other.char_lines[0].0{
            return Some(Ordering::Less)
        }
        if self.char_lines[0].0 > other.char_lines[0].0{
            return Some(Ordering::Greater)
        }
        return Some(Ordering::Equal)

    }
}

//implementation for Ord trait should be same as Partial Ord but return without Some
impl Ord for Player{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.char_lines.len() == 0 && other.char_lines.len() != 0 {
            return Ordering::Less
        }
        if self.char_lines.len() != 0 && other.char_lines.len() == 0 {
            return Ordering::Greater
        }
        //first line number comparison
        if self.char_lines[0].0 < other.char_lines[0].0{
            return Ordering::Less
        }
        if self.char_lines[0].0 > other.char_lines[0].0{
            return Ordering::Greater
        }
        return Ordering::Equal
    }
}