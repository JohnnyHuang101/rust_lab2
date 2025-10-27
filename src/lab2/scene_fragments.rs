//scene_fragments.rs. Declares the SceneFragment struct that holds a vec of players in a scene, with asscoiated functions for annoucning entrances/exists, and processing the config files and reciting the lines of each character stored in the SceneFragment as well as processing the config files. Johnny Huang, Hanson Li, Aman Verma

use super::player::Player;
use super::declarations::{WHINGE,GENERATION_FAILURE};
use std::sync::atomic::Ordering;
use super::script_gen::grab_trimmed_file_lines;
use std::collections::HashSet; //need hashset for checking duplicate lines
use std::io::{self, Write};

pub const TITLE_IDX: usize = 0;             //index of the line giving the title of the play
pub const PART_FILE_IDX: usize = 1; //index of the first line containing character info
pub const CHAR_NAME_POS: usize = 0; //index of the character's name in a line
pub const FILE_NAME_TOKEN_POS: usize = 1;      //index of the file containing the character's lines
pub const EXPECTED_TOKENS: usize = 2;       //expected number of tokens in a character line

pub type PlayConfig = Vec<(String, String)>;

pub struct SceneFragment{
    pub scene_title: String,
    pub chars_in_play: Vec<Player>,
}

impl SceneFragment{

    pub fn new(fragment_title: &String) -> Self {
         Self {
            scene_title: fragment_title.to_string(),
            chars_in_play: Vec::new(),
         }
    }

    // read each line in the config, calls Player's prepare function to parse the lines
    pub fn process_config(&mut self, play_cfg: &PlayConfig) -> Result<(), u8> {
        //note: iter yeilds immutable refs in rusts
        for a_cfg in play_cfg.iter() {
            //example from Expressions slide: match t {(x, y) => do_func(x,y);}
            match a_cfg {(char_name, speak_file) => {
              let mut new_player = Player::new(&char_name); //need mut since prepare take mut &self

              if let Err(e) = new_player.prepare(speak_file){ //TODO: confirm if this is the prepare function he wants us to call and if we should call this before or after push to vec?
                eprintln!("Error from process_config of SceneFragment: {}", e);
                return Err(GENERATION_FAILURE);
              }
              
              self.chars_in_play.push(new_player);
            }}
        }
        Ok (())
    }

    // add parsed config line to a vector (PlayConfig) holding the lines split by character name and the config file path
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
    
    // calls grab_trimmed_file_lines to populate a vector of strings holding the unsplit character and config file path, then call add_config on each of those lines to split and store into the PlayConfig 
    pub fn read_config(&mut self, cfg_fname: &String, play_cfg: &mut PlayConfig) -> Result<(), u8> {
        //play_title param is now a struct attribute self.scene_title

        let mut cfg_lines: Vec<String> = Vec::new();
        let mut stdout = io::stdout().lock(); // Lock stderr


        match grab_trimmed_file_lines(&cfg_fname, &mut cfg_lines) {
            Ok(_) => { //don't really need to read the ok code so use _
                
                // A config file can have 1 line, so we just check if it's empty
                if cfg_lines.is_empty() { 
                    writeln!(stdout,"Error: no lines from config file '{}' were read, exiting read_config with error code {}", cfg_fname, GENERATION_FAILURE);
                    return Err(GENERATION_FAILURE);
                }
            
                for a_cfg_line in cfg_lines.iter() {
                    //iter should already make a_cfg_line of &String type
                    self.add_config(a_cfg_line, play_cfg)
                }
            },
            Err(e_code) => {
                writeln!(stdout,"Error: in read_config, call to grab_trimmed_file_lines failed with error code {}", e_code);
                return Err(GENERATION_FAILURE);
            }

        }

        Ok(())
    }


    //calls the read_config and process_config in order
    pub fn prepare(&mut self, cfg_fname: &String) -> Result<(), u8> {
        //change the original script gen params: play_title: &mut String, play_vec: &mut SceneFragment to fields from SceneFragment struct
        let mut playcfg_var = PlayConfig::new();
        if let Err(e_code) = self.read_config(cfg_fname, &mut playcfg_var) {
            eprintln!("Error: in script_gen, read_config call failed with error code {}", e_code);
            return Err(GENERATION_FAILURE);
        }

        if let Err(e_code) = self.process_config(&playcfg_var) {
            eprintln!("Error: in script_gen, process_config call failed with error code {}", e_code);
            return Err(GENERATION_FAILURE);
        }

        self.chars_in_play.sort();
        Ok(())
    }

    //For each player stored in the vector of Player, we print their lines in order by extracting their line number (first pos in tuple) with the index of the Player struct in the vector and store in a vector of <usize, usize>, then we sort that vector by line number, which gives us the correct order of who should be speaking. 
    pub fn recite(&mut self) -> Result<(), u8> {

        let mut most_recent_speaker = String::new();
        //we can store the character's line number and the Player object's idx in a vector. Sort it by line number, and loop through this vector and call .speak
        let mut linenum_and_speaker_vec: Vec<(usize, usize)> = Vec::new();
        let mut linenum_set: HashSet<usize> = HashSet::new(); //use hashset to track dupe lines. If we insert dupe, it returns false so we use that to trigger whinge

        for (player_idx, a_player) in self.chars_in_play.iter().enumerate(){
            for (line_num, _)in a_player.char_lines.iter(){
                
                linenum_and_speaker_vec.push((*line_num, player_idx));
                
                //insert again into linenume_set to check for dupes
                let linenum_insert_status = linenum_set.insert(*line_num);
                if !linenum_insert_status{
                    if WHINGE.load(Ordering::SeqCst){
                        eprintln!("WHINGE Warning: duplicate line detected for line number: {}", line_num);
                    }
                }
            }
        }

        //sort by line_num
        linenum_and_speaker_vec.sort_by_key(|a_tuple| a_tuple.0);

        //Whinge if the first line doesn't start at 0 or there are duplicate lines
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

    //announces who enters the scene that also checks against a previous scene fragment to prevent announcing someone already in the scene
    pub fn enter(&self, prev_fragment: &SceneFragment) {
        let mut stdout = std::io::stdout().lock();

        if self.scene_title.split_whitespace().next().is_some() {
            writeln!(stdout,"{:?}", self.scene_title);
        }

        for plyr in &self.chars_in_play {
            //to check if prev player is already in the current list of players by their character name. If not, print the [Enter name] statement
            //followed this example using 'any' to check if elements in vec matches a condition: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
            if !prev_fragment.chars_in_play.iter().any(|prev_plyr| prev_plyr.char_name == plyr.char_name) {
                writeln!(stdout,"[Enter {:?}.]", plyr.char_name);
            }
        }
    }

    pub fn enter_all(&self) {
        let mut stdout = std::io::stdout().lock();

        if self.scene_title.split_whitespace().next().is_some() {
            writeln!(stdout,"{:?}!", self.scene_title);
        }

        for plyr in &self.chars_in_play {
            writeln!(stdout,"[Enter {:?}.]", plyr.char_name);
        }
    }

    //announces who exits by checking if they will be in the next fragment or not
    pub fn exit(&self, next_fragment: &SceneFragment) {

        let mut stdout = std::io::stdout().lock();

        for plyr in self.chars_in_play.iter().rev() { //using rev to reverse iterator so we print exit names in reverse order
            if !next_fragment.chars_in_play.iter().any(|next_plyr| next_plyr.char_name == plyr.char_name) {
                writeln!(stdout,"[Exit {:?}.]", plyr.char_name);
            }
        }
        writeln!(stdout); //new line to separate the next scene
    }

    pub fn exit_all(&self) {

        let mut stdout = std::io::stdout().lock();

        for plyr in self.chars_in_play.iter().rev() {
            writeln!(stdout,"[Exit {:?}.]", plyr.char_name);
        }
    }
}