use super::player::Player;
use super::declarations::{WHINGE,GENERATION_FAILURE};
use std::sync::atomic::Ordering;
use super::script_gen::grab_trimmed_file_lines;

pub const TITLE_IDX: usize = 0;             // Index of the line giving the title of the play
pub const PART_FILE_IDX: usize = 1; // Index of the first line containing character info
pub const CHAR_NAME_POS: usize = 0; // Index of the character's name in a line
pub const FILE_NAME_TOKEN_POS: usize = 1;      // Index of the file containing the character's lines
pub const EXPECTED_TOKENS: usize = 2;       // Expected number of tokens in a character line

pub type PlayConfig = Vec<(String, String)>;

pub struct Play{
    scene_title: String,
    chars_in_play: Vec<Player>,
}

impl Play{

    pub fn new() -> Self {
         Self {
            scene_title: String::new(),
            chars_in_play: Vec::new(),
         }
    }

    // Move the remainder of the process_config function that wasn't moved into the implementation of the Player struct, 
    // out of the script_gen.rs file and into the implementation block for the Play struct. 
    // Modify its signature so that it is a method for the Play struct. For each of the PlayConfig tuples, 
    // the process_config method should create a new instance of the Player struct using the tuple's character name field, 
    // push the new Player into the Play struct's vector, and pass the tuple's part file name into a call to the new Player 
    // struct's prepare method - if the call to the prepare method fails, the process_config method should return an appropriate error.


    pub fn process_config(&mut self, play_cfg: &PlayConfig) -> Result<(), u8> {
        //note: iter yeilds immutable refs in rusts
        for a_cfg in play_cfg.iter() {
            //example from Expressions slide: match t {(x, y) => do_func(x,y);}
            match a_cfg {(char_name, speak_file) => {
              let mut new_player = Player::new(&char_name); //need mut since prepare take mut &self

              if let Err(e) = new_player.prepare(speak_file){ //TODO: confirm if this is the prepare function he wants us to call and if we should call this before or after push to vec?
                eprintln!("Error from process_config of Play: {}", e);
                return Err(e); //TODO: change error code?
              }
              
              self.chars_in_play.push(new_player);
            }}
        }
        Ok (())
    }

    pub fn add_config(&self, cfg_line: &String, play_cfg: &mut PlayConfig){
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

    pub fn read_config(&mut self, cfg_fname: &String, play_cfg: &mut PlayConfig) -> Result<(), u8> {
        //play_title param is now a struct attribute self.scene_title

        let mut cfg_lines: Vec<String> = Vec::new();

        match grab_trimmed_file_lines(&cfg_fname, &mut cfg_lines) {
            Ok(_) => { //don't really need to read the ok code so use _
                if cfg_lines.len() < 2 {
                    println!("Error: less than 2 lines from config were read, exiting read_config with error code {}", GENERATION_FAILURE);
                    return Err(GENERATION_FAILURE);
                }
                
                self.scene_title = cfg_lines[TITLE_IDX].to_string();
                // can skip the 1st elem of cfg_lines as it contains the title, using PART_FILE_IDX for this
                for a_cfg_line in cfg_lines.iter().skip(PART_FILE_IDX) {
                    //iter should already make a_cfg_line of &String type
                    self.add_config(a_cfg_line, play_cfg)
                }
            },
            Err(e_code) => {
                println!("Error: in read_config, call to grab_trimmed_file_lines failed with error code {}", e_code);
                return Err(GENERATION_FAILURE);
            }

        }

        Ok(())
    }


    pub fn prepare(&mut self, cfg_fname: &String) -> Result<(), u8> {
        //change the original script gen params: play_title: &mut String, play_vec: &mut Play to fields from Play struct
        let mut playcfg_var = PlayConfig::new();
        if let Err(e_code) = self.read_config(cfg_fname, &mut playcfg_var) {
            eprintln!("Error: in script_gen, read_config call failed with error code {}", e_code);
            return Err(GENERATION_FAILURE);
        }

        if let Err(e_code) = self.process_config(&playcfg_var) {
            eprintln!("Error: in script_gen, process_config call failed with error code {}", e_code);
            return Err(GENERATION_FAILURE);
        }

        Ok(())
    }


    pub fn recite(&mut self) -> Result<(), u8> {
        let mut most_recent_speaker = String::new();
        // let mut prev_line_num = 0; //keep track of duplicated lines
        //we can store the character's line number and the Player object's idx in a vector. Sort it by line number, and loop through this vector and call .speak
        let mut linenum_and_speaker_vec: Vec<(usize, usize)> = Vec::new();

        for (player_idx, a_player) in self.chars_in_play.iter().enumerate(){
            for (line_num, _)in a_player.char_lines.iter(){
                
                linenum_and_speaker_vec.push((*line_num, player_idx))

            }
        }

        //sort by line_num
        linenum_and_speaker_vec.sort_by_key(|a_tuple| a_tuple.0);
        println!("{:?}", linenum_and_speaker_vec);
        //Whinge if the first line doesn't start at 0
        if WHINGE.load(Ordering::SeqCst) {
            if linenum_and_speaker_vec[0].0 != 0{
                eprintln!("WHINGE Warning: line number should start at 0!");
            }
        }
        //loop through vector to get player idx and call speak
        for (line_num_speak, player_idx) in linenum_and_speaker_vec.iter(){ //line_num_speak are the line numbers a character is suppoed to speak according to our sorting. Use this with next_line to prevent character from speaking all their lines.
            while let Some(line_num) = self.chars_in_play[*player_idx].next_line(){  //.iter.enumerate gives reference
                if line_num <= *line_num_speak{ 
                    self.chars_in_play[*player_idx].speak(&mut most_recent_speaker);

                }else{
                    break
                }
            }
        }
        Ok(())

    }

}