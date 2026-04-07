//JAI JAI SIYA RAM!
mod filemanager;
use filemanager::FileManager;
use std::error::Error;	
fn main() ->Result<(),Box<dyn Error>>{
//! # Autoscript Entry Point
//! 
//! This is the main executable for the `Autoscript` tool. It initializes the 
//! [`FileManager`], processes command-line arguments for ignore lists, and 
//! starts the primary execution loop.

/// The entry point of the application.
/// 
/// It performs the following steps:
/// 1. Initializes a new [`FileManager`] instance.
/// 2. Checks for a command-line argument to load a custom ignore list.
/// 3. Prints the current working directory to the console.
/// 4. Enters the `mainloop` to allow user interaction with files.
/// 
/// # Errors
/// Returns a boxed dynamic error if:
/// - The ignore list file cannot be read.
/// - The current working directory cannot be accessed.
/// - Any critical I/O failure occurs during the `mainloop`.

	
let fm:FileManager = FileManager::new();
let args: Vec<String> = std::env::args().collect();
let ignore_list = if args.len() > 1 {
        Some(fm.load_ignore_list(&args[1])?)
    } else {
        None
    };
println!(" the cwd is {:#?}",&fm.get_cwd()?);
fm.mainloop("Enter your number : ",&fm.get_cwd()?,
&ignore_list)?;
Ok(())
}
