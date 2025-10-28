//main.rs parses script files and delivers character lines structure by scenes and in order. Hanson Li, Aman Verma, Johnny Huang

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
    SUCCESS_CODE,
    ZERO_IDX
};
use lab2::play::Play;
use lab2::return_wrapper::ReturnWrapper;
use std::io::{self, Write};

fn usage(program_name: &str) -> String {
    format!("Wrong command line arguments. Correct usage: {program_name} <script_file_name> [whinge]\n")
}

fn parse_args(script: &mut String) -> Result<(), u8> {
    let mut args: Vec<String> = Vec::new();
    for arg in env::args(){
        args.push(arg);
    }

    if args.len() < ARGS_MIN || args.len() > ARGS_MAX || (args.len()==ARGS_MAX && args[ARG_WHINGE_IDX] != "whinge"){
        print!("{}", usage(&args[ZERO_IDX]));
        return Err(EXIT_BAD_CMDLINE);
    }
    *script = args[ARG_SCRIPT_IDX].clone();
    if args.len() == ARGS_MAX && args[ARG_WHINGE_IDX] == "whinge" {
        WHINGE.store(true, Ordering::SeqCst);
    }
    Ok(())
}

fn main() -> ReturnWrapper {
    let mut script_fname = String::new();
    let mut stderr = io::stderr().lock();

    if let Err(e_code) = parse_args(&mut script_fname){
        println!("Error in main when calling parse_args with error code {}", e_code);
        return ReturnWrapper::new(EXIT_BAD_CMDLINE)
    }

    let mut play_content = Play::new();

    if let Err(e_code) = play_content.prepare(&script_fname){

        let _ = writeln!(stderr,"Error: in main, {}", e_code);
        return ReturnWrapper::new(GENERATION_FAILURE)

    }else{
        let _ = play_content.recite();
    }

    return ReturnWrapper::new(SUCCESS_CODE)
}


