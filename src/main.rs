#[macro_use]

extern crate clap;

use std::fs;
use rand::Rng;
use std::env;
use clap::{Arg, App};
use regex::Regex;
use std::{thread, time};


static SHORT_LENGTH: usize = 160;


/*
    TODO:
        static bool	Found_one;		/* did we find a match? */
        static bool	Find_files = FALSE;	/* just find a list of proper fortune files */
        static bool	Fortunes_only = FALSE;	/* check only "fortunes" files */
        static bool	Match = FALSE;		/* dump fortunes matching a pattern */
        static bool	WriteToDisk = false;	/* use files on disk to save state */
        #ifdef DEBUG
        static int	Debug = 0;		/* print debug messages */
        #endif
*/


fn main() -> Result<(), Box<dyn std::error::Error + 'static>> 
{
    /* Order for locating the fortune file
        1. Argument passed in to the command line
        2. FORTUNE_FILE environment variable
        3. fortunes-utf8.dat in the same directory as the application
    */
    let default_fortune_file = env::var("FORTUNE_FILE")
        .unwrap_or_else(|_| "fortunes.dat".into());
    let matches = App::new("Fortune")
        .version("1.0")
        .about("Basic Rust implementation of the BSD fortune program")
        .author("Craig Jilbert")
        .arg(Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("Fortune file"))
        .arg(Arg::with_name("wait")
                .short("w")
                .long("wait")
                .takes_value(true)
                .help("Delay before displaying fortune"))
        .arg(Arg::with_name("short")
                .short("s")
                .long("short")
                .multiple(false)
                .takes_value(false)
                .help("Short fortunes only")
                .conflicts_with("all")
                .conflicts_with("long"))
        .arg(Arg::with_name("long")
                .short("l")
                .long("long")
                .multiple(false)
                .takes_value(false)
                .help("long fortunes only")
                .conflicts_with("all")
                .conflicts_with("short"))
        .get_matches();
    let fortune_file = matches.value_of("file").unwrap_or(&default_fortune_file);
    let wait = value_t!(matches, "wait", u64).unwrap_or(0); // Comment this line for clap-rs v3
    //let wait = matches.value_of_t("").unwrap_or(0) // Uncomment this line for clap-rs v3
    let short = matches.is_present("short");
    let long = matches.is_present("long");
    let fortunes = fs::read_to_string(fortune_file)?;
    let vec_fortunes = split_fortunes(&fortunes, short, long);
    let fortune = select_fortune(vec_fortunes);
    display_fortune(fortune, &wait);
    Ok(())
}


fn display_fortune(fortune: &str, wait: &u64)
{
    let sleep = time::Duration::from_secs(*wait);
    thread::sleep(sleep);
    print!("{}", &fortune);
}


fn split_fortunes(fortune: &str, short: bool, long: bool) -> Vec<&str> 
{
    let seperator = Regex::new(r"[\n\r]+%[\n\r]+").expect("Invalid regex");
    let fortunes = seperator.split(fortune).into_iter();
    if short
    {
        let wanted_fortunes = fortunes.filter(|x| x.len() <= SHORT_LENGTH).collect::<Vec<_>>();
        return wanted_fortunes;
    }
    if long
    {
        let wanted_fortunes = fortunes.filter(|x| x.len() > SHORT_LENGTH).collect::<Vec<_>>();
        return wanted_fortunes;
    }
    return fortunes.collect::<Vec<_>>();
}


fn select_fortune(fortunes: Vec<&str>) -> &str
{
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, fortunes.len());
    return fortunes[num];
}
