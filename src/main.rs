//JAI JAI SIYA RAM!
mod filemanager;
use filemanager::FileManager;
use std::error::Error;	
fn main() ->Result<(),Box<dyn Error>>{
	
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
