pub mod lab2;
use std::env;
use std::sync::atomic::Ordering;
use lab2::declarations::{
    ARGS_MAX,
    ARGS_MIN,
    ARG_SCRIPT_IDX,
    ARG_WHINGE_IDX,
    EXIT_BAD_CMDLINE,
    GENERATION_FAILURE,
    WHINGE,
};
// use lab2::script_gen::script_gen;
use lab2::play::Play;
use lab2::return_wrapper::ReturnWrapper;

fn usage(program_name: &str) -> String {
    format!(" wrong command line arguments. Correct usage: {program_name} <script_file_name> [whinge]\n")
}

fn parse_args(script: &mut String) -> Result<(), u8> {
    let mut args: Vec<String> = Vec::new();
    for arg in env::args(){
        args.push(arg);
    }

    if args.len() < ARGS_MIN || args.len() > ARGS_MAX || (args.len()==ARGS_MAX && args[ARG_WHINGE_IDX] != "whinge"){
        print!("{}", usage(&args[0]));
        return Err(EXIT_BAD_CMDLINE);
    }
    *script = args[ARG_SCRIPT_IDX].clone();
    if args.len() == ARGS_MAX && args[ARG_WHINGE_IDX] == "whinge" {
        WHINGE.store(true, Ordering::SeqCst);
    }
    Ok(())
}


// fn recite(play_title: &str, play_content: &Play) -> Result<(), u8> {
//     let mut cur_speaker = String::new(); //using this var to keep track of who the current speaker is. Initalized to "" so it works with first speaker in the file.
//     println!("Play: {}", play_title);

//     for play_entry in play_content.iter() {
//         //using _line_num since we won't be using the line num information
//         match play_entry {(_line_num, speaker, line) => print_play_line(&speaker, &line, &mut cur_speaker),};
//     }
//     Ok(())
// }

// //this is a helper function for formating play into blocks based on speakers. If we have a new speaker, also print out a new line.
// fn print_play_line(speaker: &String, line: &String, cur_speaker: &mut String){
//     if *cur_speaker != *speaker {
//         println!();
//         println!("{}:", speaker);
//         println!("{}", line);
//         *cur_speaker = speaker.clone();
//     }else {
//         println!("{}", line);
//     }
// }


fn main() -> ReturnWrapper {
    let mut script_fname = String::new();

    if let Err(e_code) = parse_args(&mut script_fname){
        println!("Error in main when calling parse_args with error code {}", e_code);
        return ReturnWrapper::new(EXIT_BAD_CMDLINE)
    }

    let mut play_title = String::new();
    let mut play_content = Play::new();

    if let Err(e_code) = play_content.prepare(&script_fname){

        eprintln!("Error: in main, {}", e_code);
        return ReturnWrapper::new(GENERATION_FAILURE)

    }else{
        play_content.recite();
    }

    // if let Err(e_code) = script_gen(&script_fname, &mut play_title, &mut play_content) {
    //     println!("call to script_gen in main.rs failed with code{}", e_code);
    //     return Err(GENERATION_FAILURE)
    // }

    //play_content.sort();

    // if let Err(e_code) = recite(&play_title, &play_content) {
    //     println!("call to recite in main.rs failed with code {}", e_code);
    //     return Err(GENERATION_FAILURE);
    // }

    return ReturnWrapper::new(0)
}


