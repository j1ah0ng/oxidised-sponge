mod types;
use log::{info, warn};
use argparse::{
    ArgumentParser,
    StoreTrue,
    Store, 
    Collect
};

static LOGGER: types::Logger = types::Logger;

struct Args {
    verbose: bool,
    file: String,
    sponge: Vec<String>,
}

fn main() {
    
    // init logger
    let _: Result<(), log::SetLoggerError> = log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info));

    let mut args: Args = Args {
        verbose: false,
        file: String::new(),
        sponge: Vec::new(),
    };
    let mut failed: bool = false;
    let mut from_file: bool = true;
    let mut err: i32 = 0;

    // limit argparse scope
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Generates sPoNgEbOB text.");
        ap.refer(&mut args.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Log output");
        ap.refer(&mut args.file)
            .add_option(&["-f", "--file"], Store, "Sponge an entire file");
        ap.refer(&mut args.sponge)
            .add_argument("string", Collect, "String to be sponged (ignored if file provided");
        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                failed = true;
                err = x;
            }
        }
    }

    // collect posargs
    let mut sponge: String = String::new();
    for s in args.sponge.into_iter() {
        sponge.push_str(&s);
        sponge.push(' ');
    }

    // check if failed
    if failed {
        warn!("argument parser failed with");
        warn!("verbose: {}", args.verbose);
        warn!("file:    {}", args.file);
        warn!("sponge:  {}", sponge);
        std::process::exit(err);
    }

    // log
    if args.verbose {
        info!("verbose: {}", args.verbose);
        info!("file:    {}", args.file);
        info!("sponge:  {}", sponge);
    }

    // try to open file
    let mut content: String = std::fs::read_to_string(&args.file)
        .map_or_else(
            |_| {
                from_file = false;
                String::from("unavailable")
            },
            |t| { t }
        );
    if !from_file {
        content = sponge;
    }

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
