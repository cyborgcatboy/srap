/*
 * The main.rs file of srap
 *
 * Copyright 2024 © max 74.25 <maximillian[at]disroot[dot]org>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by 
 * the Free Software Foundation, either version 3 of the License, or 
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but 
 * WITHOUT ANY WARRANTY; without even the implied warranty of 
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the 
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License 
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use std::{ env, fs, path::Path };

// a simple struct to hold the config vars
struct SrapConfig {
    all: bool,
    dryrun: bool,
    file: String,
    nocolor: bool,
    verbose: bool
}

impl SrapConfig {
    // init the config with some optional default values
    fn new_default() -> SrapConfig {
        SrapConfig { all: false, dryrun: false, file: String::new(), nocolor: false, verbose: false }
    }
}

// parse the command-line arguments from a String vector and return the config struct
fn parse_args(args: &mut Vec<String>) -> SrapConfig {
    let mut config = SrapConfig::new_default();

    if args.contains(&"-a".to_owned()) || args.contains(&"--all".to_owned()) {
        config.all = true;
    }

    if args.contains(&"-d".to_owned()) || args.contains(&"--dry-run".to_owned()) {
        config.dryrun = true;
    }
    
    if args.contains(&"-v".to_owned()) || args.contains(&"--verbose".to_owned()) {
        config.verbose = true;
    }
    
    if args.contains(&"-f".to_owned()) || args.contains(&"--file".to_owned()) {
        // find the nect argument, this is the filename.
        let file_arg_index = match args.iter().position( |a| a.contains("-f") ) {
            None => { panic!("You must provide a filename!"); },
            Some(index) => index
        } + 1;

        let filename = args[file_arg_index].clone();

        args.remove(file_arg_index); //gotta remove the file argument, otherwise itll end up
                                     //messing up the line
        config.file = filename;
        
        if config.verbose { println!("index: {file_arg_index}; filename: {}, args {:?}", &config.file, &args); }
    }

    if args.contains(&"-n".to_owned()) || args.contains(&"--no-color".to_owned()) {
        config.nocolor = true;
    }

    config
}

fn print_help() {
    println!("srap - the Shell Rc APpender

Usage: srap [options] <line to append>
Options:
-a / --all             : append line to all POSIX-compliant shells
-d / --dry-run         : do a dry run of the program
-f / --file <filename> : specify a file
-h / --help            : show this help
-n / --no-color        : no colored output
-v / --verbose         : verbose output

Supports bash, ksh, nsh, zsh as POSIX-compliant, and fish, tcsh, and ion shells 
(stable for both bash and zsh, everything else experimental)

Made w love by max <3");

}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // discard the first arg, as we dont care where the binary is
    
    // show help if there isn't a line, or the arguments to show it are passed, and exit the
    // program
    if args.len() == 0 || args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned()) {
        print_help();
        return;
    } 

    let config = parse_args(&mut args); // get the config
    
    if config.verbose { println!("{:?}", &args); }

    // notify the user if we are doing a dry run
    if config.dryrun { println!("{}", { if config.nocolor { "Doing a dry run..." } else { "\x1b[31;1mDoing a dry run...\x1b[0m" }}) };

    // get the line index
    let line = match args.iter().position( |a| !a.starts_with("-") ) {
        None => { print_help(); return; },
        Some(index) => index
    };

    // get the line to append from the index and the length
    let mut line_to_append: String = if args.len() > 0 { 
        format!("\n{}", &args[line..args.len()].join(" "))
    } else {
        println!("Please enter a line!"); /* getting here shouldn't be possible */"".to_owned()
    };

    // just a helper thing to add in the correct "" if alias is in the line
    if line_to_append.contains("alias") && !line_to_append.contains("\"") {
        let alias_start = match line_to_append.find("=") { Some(val) => val, _ => 1992 } + 1;
        line_to_append.insert(alias_start, '\"');
        line_to_append.insert(line_to_append.len(), '\"');
    }

    if config.verbose { println!("appending line: `{}`", line_to_append); }

    if config.all {
        // do all the posix compliant shells
        let mut config_files: Vec<String> = vec!["~/.bashrc".to_string(), 
                                             "~/.zshrc".to_string(),
                                             "~/.nshrc".to_string(),
                                             "~/.kshrc".to_string()];
        
        if !config.file.is_empty() { config_files.push(config.file) }

        // get the home directory
        let home_dir = match env::var("HOME") {
            Ok(val) => val,
            Err(e) => { println!("Couln't find HOME env var! {e}, continuing..."); "".to_string() },
        };

        for mut config_file in config_files{
            config_file = config_file.replace("~", home_dir.as_str()); // expand the ~ to the full
                                                                       // path

            if config.verbose { println!("Using presumed config file path: {}", config_file); }

            // check the file exists, continue through the files if it doesn't
            if config.nocolor {
                if !Path::new(&config_file).is_file() { println!("{} not found", config_file); continue; } 
            } else { 
                if !Path::new(&config_file).is_file() { println!("\x1b[36m{}\x1b[0m \x1b[31;1mnot found\x1b[0m", config_file); continue; } 
            }
            
            // read the existing config
            let existing_config = match fs::read_to_string(config_file.clone()) {
                Ok(val) => val,
                Err(e) => panic!("couldnt read file: {}, {e}", &config_file)
            };

            if config.nocolor {
                println!("Appending \"{:}\" to {}", match line_to_append.strip_prefix("\n") { None => "", Some(s) => s } , &config_file);
            } else {
                println!("\x1b[35;1mAppending\x1b[0m \"{:}\" \x1b[35;1mto\x1b[0m \x1b[36m{}\x1b[0m", match line_to_append.strip_prefix("\n") { None => "", Some(s) => s } , &config_file);
            }
            
            // append the new line
            let new_config = existing_config + &line_to_append;

            // if it's not a dry run, write to the file the new config
            if !config.dryrun {
                match fs::write(&config_file, new_config) {
                    Ok(result) => result,
                    Err(e) => panic!("writing file failed! {config_file}: {e}")
                }
            }
        }
    } else {
        // get which shell we're in
        let shell = match env::var("SHELL") {
            Ok(val) => val,
            Err(e) => panic!("couldn't interpret SHELL: {e}"),
        };

        // get the home directory
        let home_dir = match env::var("HOME") {
            Ok(val) => val,
            Err(e) => { println!("Couln't find HOME env var! {e}, continuing..."); "".to_string() },
        };

        if config.verbose { println!("SHELL: {shell}"); }

        // find the config file we're using
        let config_file_path = {
            if !config.file.is_empty() {
                config.file.as_str()
            } else if shell.as_str().contains("zsh") {
                "~/.zshrc"
            } else if shell.as_str() .contains("bash"){
                "~/.bashrc"
            } else if shell.as_str().contains("nsh") {
                "~/.nshrc"
            } else if shell.as_str().contains("ksh") {
                "~/.kshrc"
            } else if shell.as_str().contains("fish") {
                "~/.config/fish/config.fish"
            } else if shell.as_str().contains("ion") {
                ".config/ion/initrc"
            } else if shell.as_str().contains("tcsh") {
                "~/.cshrc"
            } else {
                panic!("Unsupported shell!: {shell}")
            }
        }.replace("~", home_dir.as_str());

        if config.verbose { println!("Using presumed config file path: {}", config_file_path); }

        let existing_config = match fs::read_to_string(config_file_path.clone()) {
            Ok(val) => val,
            Err(e) => panic!("couldnt read file: {config_file_path}, {e}")
        };

        if config.nocolor {
            println!("Appending \"{:}\" to {}", match line_to_append.strip_prefix("\n") { None => "", Some(s) => s } , config_file_path);
        } else {
            println!("\x1b[35;1mAppending\x1b[0m \"{:}\" \x1b[35;1mto\x1b[0m \x1b[36m{}\x1b[0m", match line_to_append.strip_prefix("\n") { None => "", Some(s) => s } , config_file_path);
        }

        let new_config = existing_config + &line_to_append;

        if !config.dryrun {
            match fs::write(config_file_path, new_config) {
                Ok(result) => result,
                Err(e) => panic!("writing file failed! {e}")
            }
        }
    }
    println!("{}", if config.nocolor {"Now source the config file and you're all ready to go! :3"} else {"\x1b[32mNow source the config file and you're all ready to go! :3\x1b[0m"});
} 
