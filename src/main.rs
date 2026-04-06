//JAI JAI SIYA RAM!
mod filemanager;
use filemanager::FileManager;
use std::error::Error;	
fn main() ->Result<(),Box<dyn Error>>{
	
let fm:FileManager = FileManager::new();
fm.mainloop("Enter your number : ",&fm.get_cwd()?)?;
println!("{:#?}",fm);
Ok(())
}
