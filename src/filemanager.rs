use std::{io::{stdin, Error,ErrorKind},
process::Command,
env::current_dir,
fs::read_dir,
path::Path};

macro_rules! inputnum{
($text:expr)=>{
{
println!("{}",$text);
//std::io::stdout().flush().expect("Stdout not flushe>
let mut selected_option=String::new();
stdin()
 .read_line(&mut selected_option)
   .map_err(|e| e)?;

            // Convert ParseIntError into a generic io::Error
let num: Result<usize, _> = selected_option
          .trim()
          .parse()
          .map_err(|_| Error::new(
ErrorKind::InvalidInput,
 "Not a number"));
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
Ok(format!("{}",path.display()))

}


pub fn get_dir_items(&self, path:&String)->
Result<Vec<String>,Error>{

let v_path: Result<Vec<String>, Error> =
 read_dir(path)?
        .map(|entry| {
            let entry = entry?; // Exit the CLOSURE with Err if an entry fails.
  let full_path = Path::new(path)
.join(entry.file_name());
    full_path.to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| Error::new(
std::io::ErrorKind::InvalidData,
 "Invalid Unicode"))
})

        .collect();

    // 3. Return the Result directly (No semicolon!).
    v_path

}



pub fn get_files_paths(&self,path:&mut Vec<String>)->
Result<Vec<String>,Error>{
let mut v: Vec<_> = Vec::new();
for i in path{
let mypath : &Path = Path::new(&i[..]);
if mypath.is_file(){
v.push(i.to_string());
}
if mypath.is_dir(){
let mut v_path= self.get_dir_items(i)?;
v.extend(self.get_files_paths(
&mut v_path
)?);
}

}
Ok(v)
}


pub fn files_show(&self,files_list:&Vec<String>)->
Result<(),Error>{
for (i, file_path) in files_list.iter().enumerate() {
        let path = Path::new(file_path);
        
        // Convert Option to Result so we can use '?' properly
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




pub fn mainloop(&self,text:&str,path:&String)->
Result<(),Error>{
let paths: Vec<_> =self.get_files_paths(&mut self
.get_dir_items(path)?)?;
self.files_show(&paths);
let mut num = if let Ok(n)=inputnum!(text){
n
}else{
paths.len()
};
while num<=paths.len(){
 if num<paths.len(){
 Command::new("nano")
.arg(&paths[num][..]).status()?;
self.files_show(&paths);
 num = if let Ok(n)=inputnum!(text){
n
}else{
paths.len()
};
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

