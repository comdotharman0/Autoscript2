use std::{io::{stdin, Error},
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
  .expect("wrong number");
let num: Result<usize,_> = selected_option
.trim()
.parse()?;
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
match current_dir()?{
Ok(cwd)=> format!("{}",cwd.display()),
Err(e)=> format!("{}",e)
}

}


pub fn get_dir_items(&self, path:&String)->
Result<Vec<String>,Error>{
let  v_path: Vec<_>= read_dir(path)?.map(|j|{
String::from(j?.to_str()?)
 }).collect(); 
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
let mut v_path: Vec<_>= self.get_dir_items(i);
v.extend(self.get_files_paths(
&mut v_path
));
}

}
v
}


pub fn files_show(&self,files_list:&Vec<String>)->
Result<(),Error>{
for i in 0..files_list.len(){
let path: &Path = Path::new(
&files_list[i][..]);
println!("Press {} to  edit {}",i,
path.file_name()?.to_str()?);


 }
println!("Press {} or greater to exit" , files_list.len());
}

pub fn mainloop(&self,text:&str,path:&String)->
Result<(),Error>{
let paths: Vec<_> =self.get_files_paths(&mut self
.get_dir_items(path));
self.files_show(&paths);
let mut num : usize= inputnum!(text);
while num<=paths.len(){
 if num<paths.len(){
 Command::new("nano")
.arg(&paths[num][..]).status()?;
self.files_show(&paths);
num = inputnum!(text);
}else {
break;
}
 }
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

