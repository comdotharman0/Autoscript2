//JAI JAI SIYA RAM!
mod filemanager;
use filemanager::FileManager;

fn main() {
	
let fm:FileManager = FileManager.new();
fm.mainloop("Enter your number : ",&fm.get_cwd());
println!("{:#?}",fm);
}
