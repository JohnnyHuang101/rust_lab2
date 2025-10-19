use super::scene_fragments::SceneFragment; // Changed Player to SceneFragment
use super::declarations::{WHINGE,GENERATION_FAILURE};
use std::sync::atomic::Ordering;
use super::script_gen::grab_trimmed_file_lines;

pub const TITLE_IDX: usize = 0;             // Index of the line giving the title of the play
pub const PART_FILE_IDX: usize = 1; // Index of the first line containing character info
pub const CHAR_NAME_POS: usize = 0; // Index of the character's name in a line
pub const FILE_NAME_TOKEN_POS: usize = 1;      // Index of the file containing the character's lines
pub const EXPECTED_TOKENS: usize = 2;       // Expected number of tokens in a character line

// pub type PlayConfig = Vec<(String, String)>;

pub type ScriptConfig = Vec<(bool, String)>; 
pub type Fragments = Vec<SceneFragment>; 

pub struct Play{
    fragments: Fragments
}

impl Play{
    pub fn new() -> Self {
        Self {
            fragments: Vec::new(),
        }
    }
    // Modify the implementation of the Play struct's associated process_config method so that 
    // it maintains a title string with which to initialize each new SceneFragment it creates. 
    // The method should destructure each tuple in the ScriptConfig argument that was passed to it, 
    // into a boolean variable indicating whether the text field represents the title of a new scene (true) 
    // or the name of a configuration file (false). If the text is a new title the method should use it to update 
    // its title string; otherwise it should (1) use the title string to push a new SceneFragment into the 
    // Play struct's vector of fragments, (2) set the title string to be an empty string (so a new title is 
    // only announced by the first fragment of the scene), (3) pass the text from the current configuration tuple 
    // into a call to the new fragment's prepare method, and (4) match on the result of that call. If that call failed,
    //  the process_config method should return an error indicating that script generation failed - otherwise 
    // if the call succeeded the process_config method should continue to the next tuple.
    
    pub fn process_config(&mut self, play_cfg: &ScriptConfig) -> Result<(), u8> {
        //variable to keep track of index of fragment in self.fragments after insertion
        let mut latest_frag_idx: usize;
        let mut title_str = String::new();

        //note: iter yeilds immutable refs in rusts
        for a_cfg in play_cfg.iter() {
            //example from Expressions slide: match t {(x, y) => do_func(x,y);}
            match a_cfg {(is_title, text_field) => { //both is_title and text_field are refs
                if *is_title{
                    title_str = text_field.to_string();
                }else{
                    //1: push title_str init SceneFrag to vec of fragments and update idx
                    self.fragments.push(SceneFragment::new(&title_str));
                    latest_frag_idx = self.fragments.len() - 1;
                    //2: reset title_str to empty
                    title_str = String::new();
                    //3: pass text_field to new fragment prepare method method and 4 match result on errors
                    match self.fragments[latest_frag_idx].prepare(text_field){
                        Ok(_) => {
                            //do nothing
                        },
                        Err(e_code) => {
                            eprintln!("Error from process config of Play after calling prepare on Fragment: {}", e_code);
                            return Err(GENERATION_FAILURE);
                        },
                    }
                    
                }
            }}
        }
        Ok (())
    }

    pub fn add_config(&self, cfg_line: &String, play_cfg: &mut ScriptConfig){
        //split_whitespace gives an iterable, and collect turns that into a collection
        //since using &str, need to do .to_string when inserting into play_cfg because it is of type <String, String>

        let cfg_items: Vec<&str> = cfg_line.split_whitespace().collect(); 

        if cfg_items.is_empty() {
            return; // ignore blank lines
        }

        if cfg_items[0] == "[scene]" {
            if cfg_items.len() == 1 {
                //scene] alone, skip line
                if WHINGE.load(Ordering::SeqCst) {
                    eprintln!("Error: [scene] directive missing title");
                }
            } else {
                //with tiele ok
                let scene_title = cfg_items[1..].join(" ");
                play_cfg.push((true, scene_title));
            }
        } else {
            //config file case
            //since using &str, need to do .to_string when inserting into play_cfg because it is of type <String, String>
            play_cfg.push((false, cfg_items[0].to_string()));
            
            if cfg_items.len() > 1 && WHINGE.load(Ordering::SeqCst) {
                eprintln!("Warning: additional tokens after config file name '{}'", cfg_items[0]);
            }
        }
    }
      
    pub fn read_config(&self, cfg_fname: &String, play_cfg: &mut ScriptConfig) -> Result<(), u8> {
        //play_title param is now a struct attribute self.scene_title

        let mut cfg_lines: Vec<String> = Vec::new();
        match grab_trimmed_file_lines(&cfg_fname, &mut cfg_lines) {
            Ok(_) => {//don't really need to read the ok code so use _
                if cfg_lines.is_empty() {
                    eprintln!("Error: no lines read from script file '{}'", cfg_fname);
                    return Err(GENERATION_FAILURE);
                }
                // self.scene_title = cfg_lines[TITLE_IDX].to_string();
                // can skip the 1st elem of cfg_lines as it contains the title, using PART_FILE_IDX for this
 
                for a_cfg_line in cfg_lines.iter() {
                    //iter should already make a_cfg_line of &String type

                    self.add_config(a_cfg_line, play_cfg)
                }
            },
            Err(_) => {
                eprintln!("Error: could not open or read script file '{}'", cfg_fname);
                return Err(GENERATION_FAILURE);
            }
        }
        Ok(())
    }

    pub fn prepare(&mut self, cfg_fname: &String) -> Result<(), u8> {
        //change the original script gen params: play_title: &mut String, play_vec: &mut Play to fields from Play struct
        let mut playcfg_var = ScriptConfig::new();
        if let Err(e_code) = self.read_config(cfg_fname, &mut playcfg_var) {
            // read_config now prints its own errors, but we still need to stop execution
            eprintln!("Error: in prepare, read_config call failed with error code {}", e_code);
            return Err(GENERATION_FAILURE);
        }
        if let Err(e_code) = self.process_config(&playcfg_var) {
            eprintln!("Error: in prepare, process_config call failed with error code {}", e_code);
            return Err(GENERATION_FAILURE);
        }

        //chheck if fragments exist and the first one is a title
        if self.fragments.is_empty() || !self.fragments[0].scene_title.split_whitespace().next().is_some(){
            eprintln!("Error: Script file must contain at least one [scene] directive at the start.");
            return Err(GENERATION_FAILURE);
        }
        
        Ok(())
    }

    pub fn recite(&mut self) -> Result<(), u8> {

         // let mut prev_line_num = 0; //keep track of duplicated lines
        //we can store the character's line number and the Player object's idx in a vector. Sort it by line number, and loop through this vector and call .speak
        let num_fragments = self.fragments.len();
        if num_fragments == 0 {
            return Ok(()); // Nothing to recite
        }

        for idx in 0..num_fragments {
            
            //handle enter logic
            if idx == 0 {
                self.fragments[idx].enter_all();
            } else {
                //not first, pass reference to previous
                let (prev_slice, current_and_rest) = self.fragments.split_at_mut(idx);
                let prev_fragment = &prev_slice[idx - 1];
                let current_fragment = &mut current_and_rest[0];
                current_fragment.enter(prev_fragment);
            }
            //loop through vector to get player idx and call speak
            let _ = self.fragments[idx].recite();

            //Whinge if the first line doesn't start at 0
            if idx == num_fragments - 1 {
                
                self.fragments[idx].exit_all();
            } else {
                // not the last, pass reference to next
                let (current_slice, next_slice) = self.fragments.split_at_mut(idx + 1);
                let current_fragment = &mut current_slice[idx];
                let next_fragment = &next_slice[0];
                current_fragment.exit(next_fragment);
                
            }
        }

        Ok(())
    }
}