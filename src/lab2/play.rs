//play.rs declares the Play struct that holds vector of SceneFragments. It contains associated functions for processing the script files and structuring the line delivery. Hanson Li, Johnny Haung, Aman Verma

use super::scene_fragments::SceneFragment;
use super::declarations::{WHINGE,GENERATION_FAILURE};
use std::sync::atomic::Ordering;
use super::script_gen::grab_trimmed_file_lines;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

pub const TITLE_IDX: usize = 0;             //index of the line giving the title of the play
pub const PART_FILE_IDX: usize = 1; //index of the first line containing character info
pub const CHAR_NAME_POS: usize = 0; //index of the character's name in a line
pub const FILE_NAME_TOKEN_POS: usize = 1;      //index of the file containing the character's lines
pub const EXPECTED_TOKENS: usize = 2;       //expected number of tokens in a character line


pub type ScriptConfig = Vec<(bool, String)>; 
pub type Fragments = Vec<Arc<Mutex<SceneFragment>>>; 


pub struct Play{
    fragments: Fragments
}

impl Play{
    pub fn new() -> Self {
        Self {
            fragments: Vec::new(),
        }
    }


    //the process_config function here reads in the script.txt file, iterate through the scene title and listed config file paths, and call SceneFragment's prepare function on the config file paths.
    pub fn process_config(&mut self, play_cfg: &ScriptConfig) -> Result<(), u8> {
        //variable to keep track of index of fragment in self.fragments after insertion
        let mut latest_frag_idx: usize;
        let mut title_str = String::new();
        let mut stderr = io::stderr().lock();

        //note: iter yeilds immutable refs in rusts
        for a_cfg in play_cfg.iter() {
            //example from Expressions slide: match t {(x, y) => do_func(x,y);}
            match a_cfg {(is_title, text_field) => { //both is_title and text_field are refs
                if *is_title{
                    title_str = text_field.to_string();
                }else{
                    //1: push title_str init SceneFrag to vec of fragments and update idx
                    self.fragments.push(Arc::new(Mutex::new(SceneFragment::new(&title_str))));
                    latest_frag_idx = self.fragments.len() - 1;
                    //2: reset title_str to empty
                    title_str = String::new();
                    //3: pass text_field to new fragment prepare method method and 4 match result on errors



                    match self.fragments[latest_frag_idx].lock() {
                        Ok(mut scene_ref) => {
                            //here `scene_ref` is an immutable reference to SceneFragment
                            match scene_ref.prepare(text_field){
                                Ok(_) => {
                                    //do nothing
                                },
                                Err(e_code) => {
                                    let _ = writeln!(stderr,"Error from process config of Play after calling prepare on Fragment: {}", e_code);
                                    return Err(GENERATION_FAILURE);
                                },
                            }
                        }
                        Err(_) => {
                            let _ = writeln!(stderr, "Error: Failed to acquire lock on first scene fragment");
                            return Err(GENERATION_FAILURE);
                        }
                    }
                    
                }
            }}
        }
        Ok (())
    }

    //for a line in the script file, determine if it is a scene line or a line with config file paths and add a bool val to the vector of lines
    pub fn add_config(&self, cfg_line: &String, play_cfg: &mut ScriptConfig){
        //split_whitespace gives an iterable, and collect turns that into a collection

        let cfg_items: Vec<&str> = cfg_line.split_whitespace().collect(); 
        let mut stderr = io::stderr().lock();

        if cfg_items.is_empty() {
            return;
        }

        if cfg_items[0] == "[scene]" {
            if cfg_items.len() == 1 {
                //[scene] alone, skip line and whinge
                if WHINGE.load(Ordering::SeqCst) {
                    let _ = writeln!(stderr,"Whinge Warning: [scene] directive missing title");
                }
            } else {
                //contains other tokens with [scene], concat from 1st element and up
                let scene_title = cfg_items[1..].join(" ");
                play_cfg.push((true, scene_title));
            }
        } else {
            //config file case
            //since using &str, need to do .to_string when inserting into play_cfg because it is of type <String, String>
            play_cfg.push((false, cfg_items[0].to_string()));
            
            if cfg_items.len() > 1 && WHINGE.load(Ordering::SeqCst) {
                let _ = writeln!(stderr,"Whinge Warning: there are additional tokens after config file name '{}'", cfg_items[0]);
            }
        }
    }
    
    //read in the script file with grab_trimmed_file_lines function 
    pub fn read_config(&self, cfg_fname: &String, play_cfg: &mut ScriptConfig) -> Result<(), u8> {
        //play_title param is now a struct attribute self.scene_title
        let mut stderr = io::stderr().lock();

        let mut cfg_lines: Vec<String> = Vec::new();
        match grab_trimmed_file_lines(&cfg_fname, &mut cfg_lines) {
            Ok(_) => {//don't really need to read the ok code so use _
                if cfg_lines.is_empty() {
                    let _ = writeln!(stderr,"Error: no lines read from script file '{}'", cfg_fname);
                    return Err(GENERATION_FAILURE);
                }
 
                for a_cfg_line in cfg_lines.iter() {
                    //iter should already make a_cfg_line of &String type

                    self.add_config(a_cfg_line, play_cfg)
                }
            },
            Err(_) => {
                let _ = writeln!(stderr,"Error: could not open or read script file '{}'", cfg_fname);
                return Err(GENERATION_FAILURE);
            }
        }
        Ok(())
    }

    //calls the read_config and process_config in order 
    pub fn prepare(&mut self, cfg_fname: &String) -> Result<(), u8> {
        //change the original script gen params: play_title: &mut String, play_vec: &mut Play to fields from Play struct
        let mut playcfg_var = ScriptConfig::new();
        let mut stderr = io::stderr().lock();

        if let Err(e_code) = self.read_config(cfg_fname, &mut playcfg_var) {
            // read_config now prints its own errors, but we still need to stop execution
            let _ = writeln!(stderr,"Error: in prepare, read_config call failed with error code {}", e_code);
            return Err(GENERATION_FAILURE);
        }
        if let Err(e_code) = self.process_config(&playcfg_var) {
            let _ = writeln!(stderr,"Error: in prepare, process_config call failed with error code {}", e_code);
            return Err(GENERATION_FAILURE);
        }

        //chheck if fragments exist and the first one is a title
        if self.fragments.is_empty(){
            let _ = writeln!(stderr,"Error: Script file must contain at least one [scene] directive at the start.");
            return Err(GENERATION_FAILURE);
        }
        

        match self.fragments[0].lock() {
            Ok(scene_ref) => {
                //here `scene_ref` is an immutable reference to SceneFragment
                if !scene_ref.scene_title.split_whitespace().next().is_some() {
                    let _ = writeln!(stderr, "Error: First scene must have a title.");
                    return Err(GENERATION_FAILURE);
                }
            }
            Err(_) => {
                let _ = writeln!(stderr, "Error: Failed to acquire lock on first scene fragment");
                return Err(GENERATION_FAILURE);
            }
        }

        Ok(())
    }






    //formats the character speech parts in scene-structure by calling entry then fragment's recite, and exit for each SceneFragment in vector
    pub fn recite(&mut self) -> Result<(), u8> {
        let mut stderr = io::stderr().lock();

        let num_fragments = self.fragments.len();

        //return early if fragments is empty
        if num_fragments == 0 {
            return Ok(());
        }
        
        for cur_fragment_idx in 0..num_fragments{

            // lock current fragment
            match self.fragments[cur_fragment_idx].lock() {
            Ok(mut scene_ref) => {
                //stuff goies here now

                if cur_fragment_idx == 0{ //first fragment, call enter all
                    scene_ref.enter_all();
                }else{ //save to do -1 to get prev index
                    let prev_fragment_idx = cur_fragment_idx - 1;

                    //NESTED FOR PREV SCENE
                    match self.fragments[prev_fragment_idx].lock() { //get reference to previous fragment
                    Ok(prev_scene_ref) => {
                        scene_ref.enter(&*prev_scene_ref); //do the enter call
                    }
                    Err(_) => {
                        let _ = writeln!(stderr, "Error: Failed to acquire lock on ith scene fragment");
                        return Err(GENERATION_FAILURE);
                    }
                }
                }

                //do the actual recite call on the SceneFragment
                if let Err(e_code) = scene_ref.recite(){
                    let _ = writeln!(stderr,"Error from recite in Play.rs: unsucessful fragment recite call with error code {}", e_code);
                    return Err(GENERATION_FAILURE);
                }

                //block for exit name calls
                if cur_fragment_idx == num_fragments-1 {
                    scene_ref.exit_all();
                }else{
                    //safe to do +1 here to the fragment index
                    let next_fragment_idx = cur_fragment_idx + 1;

                    //NESTED FOR NEXT SCENE
                    match self.fragments[next_fragment_idx].lock() { //get reference to next fragment
                    Ok(next_scene_ref) => {
                        scene_ref.exit(&*next_scene_ref); //do the exit call
                    }
                    Err(_) => {
                        let _ = writeln!(stderr, "Error: Failed to acquire lock on ith scene fragment");
                        return Err(GENERATION_FAILURE);
                    }
                }
                }
            }
            Err(_) => {
                let _ = writeln!(stderr, "Error: Failed to acquire lock on ith scene fragment");
                return Err(GENERATION_FAILURE);
            }
            }
        }
        Ok(())
    }

}