use std::{io::{stdin, Error},
process::Command,
env::current_dir,
fs::{read_dir,read_to_string},
path::Path};

/// The macro `inputnum` enables the program
/// to take the user input as a number and return a 
/// "usize" type. 
/// if a user types invalid number 
/// then "0" is selected by default

macro_rules! inputnum{
($text:expr)=>{
{
println!("{}",$text);
//std::io::stdout().flush().expect("Stdout not flushe>
let mut selected_option=String::new();
stdin()
 .read_line(&mut selected_option)
   .map_err(|e| e).unwrap_or_else(|_|{
println!("Some error occurred please restart");
0
});
let num : usize= selected_option
.trim()
.parse().unwrap_or_else(|_|{
println!("you chose invalid value so by default 0 is selected");
0
});
num

}

}

}


#[derive(Debug)]
pub struct FileManager;


impl FileManager{



pub fn new()->Self{
FileManager
}



pub fn get_cwd(&self)->Result<String,Error>{
let path=current_dir()?;
Ok(path.to_string_lossy().into_owned())

}


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




pub fn mainloop(&self,text:&str,path:&String,
ignore_list_file_path:&Option<Vec<String>>)->
Result<(),Error>{
let paths: Vec<_> =self.get_files_paths(&mut self
.get_dir_items(path)?,ignore_list_file_path)?;
self.files_show(&paths)?;
let mut num = inputnum!(text);
while num<=paths.len(){
 if num<paths.len(){
 Command::new("nano")
.arg(&paths[num][..]).status()?;
self.files_show(&paths)?;
 num = inputnum!(text);
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

