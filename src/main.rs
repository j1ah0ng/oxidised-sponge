mod types;
use log::{info, warn};
use argparse::{
    ArgumentParser,
    StoreTrue,
    Store, 
    Collect
};

static LOGGER: types::Logger = types::Logger;

fn main() {
    
    // init logger
    let _: Result<(), log::SetLoggerError> = log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info));

    let mut verbose = false;
    let mut file = String::new();
    let mut sponge_v: Vec<String> = Vec::new();
    let mut failed = false;
    let mut err: i32 = 0;

    { // limit argparse scope
        let mut ap = ArgumentParser::new();
        ap.set_description("Generates sPoNgEbOB text.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Log output");
        ap.refer(&mut file)
            .add_option(&["-f", "--file"], Store, "Sponge an entire file");
        ap.refer(&mut sponge_v)
            .add_argument("string", Collect, "String to be sponged (ignored if file provided");
        match ap.parse_args() {
            Err(x) => {
                failed = true;
                err = x;
            }
            _ => { }
        }
    }

    // collect posargs
    let mut sponge: String = String::new();
    for s in sponge_v.into_iter() {
        sponge.push_str(&s);
        sponge.push(' ');
    }
    let sponge = String::from(sponge.trim());

    // check if failed
    if failed {
        warn!("Argument parser failed with");
        warn!("verbose: {}", verbose);
        warn!("file:    {}", file);
        warn!("sponge:  {}", sponge);
        std::process::exit(err);
    } else if sponge.len() == 0 {
        warn!("Please provide the text to be sponged.");
        std::process::exit(3);
    }

    if verbose {
        info!("verbose: {}", verbose);
        info!("file:    {}", file);
        info!("sponge:  {}", sponge);
    }

    // try to open file
    let content = match std::fs::read_to_string(&file).ok() {
        None => { sponge }
        Some(x) => { x }
    };

    // begin sponging
    let mut sm = types::StateMachine::new();
    let mut sponged: String = String::new();
    for c in content.chars() {
        eprintln!("{}", c);
        // allow symbols and numbers
        if !c.is_alphabetic() {
            sponged.push(c);
        } else {
            let mut was_except: bool = false;
            let mut s: usize = 0;
            loop {
                s += 1;
                if s >= types::LEN {
                    break;
                }
                if types::EXCEPTIONS[s] == c {
                    was_except = true;
                    break;
                }
            }
            if !was_except {
                if sm.next_is_upper() {
                    sm.consec_up += 1;
                    sponged.push(c
                        .to_uppercase()
                        .collect::<Vec<_>>()[0]);
                } else {
                    sm.consec_down += 1;
                    sponged.push(c
                        .to_lowercase()
                        .collect::<Vec<_>>()[0]);
                }
            }
        }
    }

    eprintln!("{}", sponged);
}
