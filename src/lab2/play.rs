use super::player::Player;

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


    pub fn process_config(&mut self, play_vec: &mut Play, play_cfg: &PlayConfig) -> Result<(), u8> {
        //note: iter yeilds immutable refs in rusts
        for a_cfg in play_cfg.iter() {
            //example from Expressions slide: match t {(x, y) => do_func(x,y);}
            match a_cfg {(char_name, speak_file) => {
              let new_player = Player::new(&char_name);

              //call prepare method of vec now stroed in the Play struct
              if let Err(e) = self.chars_in_play[self.chars_in_play.len() - 1]{
                eprintln!("Error from process_config of Play: {}", e);
                return Err(e); //TODO: change error code?
              }
              
              self.chars_in_play.push(new_player);
            }}
        }
        Ok (())
    }

}
