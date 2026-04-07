//! # Autoscript File Manager
//! 
//! `Autoscript` provides a robust, recursive file management system designed to 
//! handle file discovery and batch editing with safety and configurability.
//! 
//! This module contains the [`FileManager`] struct which handles directory traversal,
//! path filtering via ignore lists, and a terminal-based interface for file manipulation.


use std::{io::{stdin, Error},
process::Command,
env::current_dir,
fs::{read_dir,read_to_string},
path::Path};


/// Prompts the user for numeric input via standard input.
/// 
/// # Arguments
/// * `$text` - The string slice message to display to the user.
/// * `$default` - The fallback `usize` value if the input is invalid or an error occurs.
/// 
/// # Returns
/// Returns a `usize`. If the user provides a non-numeric string or an I/O error occurs,
/// it prints an error message and returns the `$default` value.
///macro_rules! inputnum { ... }


macro_rules! inputnum{
($text:expr,$default:expr)=>{
{
println!("{}",$text);
//std::io::stdout().flush().expect("Stdout not flushe>
let mut selected_option=String::new();
stdin()
 .read_line(&mut selected_option)
   .map_err(|e| e).unwrap_or_else(|_|{
println!("Some error occurred please restart");
$default
});
let num : usize= selected_option
.trim()
.parse().unwrap_or_else(|_|{
println!(
"you chose invalid value so program exits now. Please restart and use correct value");
$default
});
num

}

}

}


/// A stateless manager for handling recursive file operations and directory traversal.

#[derive(Debug)]
pub struct FileManager;


impl FileManager{



pub fn new()->Self{
FileManager
}

/// Returns the current working directory as a String.
    /// 
    /// # Errors
    /// Returns an [`std::io::Error`] if the current directory does not exist 
    /// or if the process lacks permissions to access it.
 

pub fn get_cwd(&self)->Result<String,Error>{
let path=current_dir()?;
Ok(path.to_string_lossy().into_owned())

}

/// Retrieves a list of all items (files and folders) within a specified directory.
    /// 
    /// # Arguments
    /// * `path` - A reference to a String representing the directory to scan.
    /// 
    /// # Errors
    /// Returns `InvalidData` if a filename contains invalid Unicode, or a standard 
    /// I/O error if the path is not a directory.

pub fn get_dir_items(&self, path:&String)->
Result<Vec<String>,Error>{

let v_path: Result<Vec<String>, Error> =
 read_dir(path)?
        .map(|entry| {
            let entry = entry?;
  let full_path = Path::new(path)
.join(entry.file_name());
    full_path.to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| Error::new(
std::io::ErrorKind::InvalidData,
 "Invalid Unicode"))
})

        .collect();

        v_path

}

/// Loads a list of patterns to ignore from a plain text file.
    /// 
    /// Each line in the file is treated as a separate ignore pattern. Whitespace 
    /// is trimmed, and empty lines are discarded.
    /// 
    /// # Errors
    /// Returns an error if the file cannot be read or found.


pub fn load_ignore_list(&self, file_path: &str) ->
 Result<Vec<String>, Error> {
    let content = read_to_string(file_path)?;
    // Split by lines, remove
// whitespace, and ignore empty lines
    let list: Vec<String> = content
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(list)
}

/// Recursively finds all file paths within a list of directories, respecting 
    /// an optional ignore list.
    /// 
    /// # Arguments
    /// * `path` - A mutable vector of starting directory/file paths.
    /// * `paths_to_ignore` - An optional vector of strings. If a path contains 
    /// any of these strings, it will be skipped.
    /// 
    /// # Errors
    /// Returns an error if the traversal encounters a directory it cannot read.


pub fn get_files_paths(&self,
path:&mut Vec<String>,
paths_to_ignore: &Option<Vec<String>>)->
Result<Vec<String>,Error>{
let mut v: Vec<_> = Vec::new();

for i in path{
let mypath : &Path = Path::new(&i[..]);
 let mut to_ignore = false;
if let Some(path_to_ignore)= paths_to_ignore{ 
for ignore in path_to_ignore{
if i.contains(ignore) { 
to_ignore=true;


 }
}

}


if !to_ignore{

 
if mypath.is_file(){
v.push(i.to_string());
}
if mypath.is_dir(){
let mut v_path= self.get_dir_items(i)?;
v.extend(self.get_files_paths(
&mut v_path, &None
)?);
}
}

}
Ok(v)
}


/// Displays a numbered list of files to the standard output.
    /// 
    /// # Arguments
    /// * `files_list` - A reference to a vector of file paths to display.


pub fn files_show(&self,files_list:&Vec<String>)->
Result<(),Error>{
for (i, file_path) in files_list.iter().enumerate() {
        let path = Path::new(file_path);
        
        
        let file_name = path.file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| Error::new(
std::io::ErrorKind::InvalidData,
 "Invalid file name"))?;

        println!("Press {} to edit {}", i, file_name);
    }
    
    println!("Press {} or greater to exit", files_list.len());
Ok(())
}

/// The primary execution loop for the file manager.
    /// 
    /// This function scans the directory, presents the files to the user, and 
    /// opens the selected file in the `nano` editor. It repeats until the 
    /// user chooses to exit.
    /// 
    /// # Errors
    /// Returns an error if the external editor (`nano`) fails to start or if 
    /// I/O operations fail.


pub fn mainloop(&self,text:&str,path:&String,
ignore_list_file_path:&Option<Vec<String>>)->
Result<(),Error>{
let paths: Vec<_> =self.get_files_paths(&mut self
.get_dir_items(path)?,ignore_list_file_path)?;
self.files_show(&paths)?;
let mut num = inputnum!(text,paths.len());
while num<=paths.len(){
 if num<paths.len(){
 Command::new("nano")
.arg(&paths[num][..]).status()?;
self.files_show(&paths)?;
 num = inputnum!(text,paths.len());
}else {
break;
}
 }
Ok(())
}
}
	

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cwd_not_empty() {
        let fm = FileManager::new();
        assert!(!fm.get_cwd().is_empty());
    }
}

