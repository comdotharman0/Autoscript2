use std::{io::stdin,process,env,fs,path};

macro_rules! inputnum{
($text:expr)=>{
{
println!("{}",$text);
//std::io::stdout().flush().expect("Stdout not flushe>
let mut selected_option=String::new();
stdin()
 .read_line(&mut selected_option)
  .expect("wrong number");
selected_option
.trim()
.parse().expect("It should be a number")

}

}

}

#[derive(Debug)]
pub struct FileManager;


impl FileManager{



pub fn new(&self)->Self{
FileManager
}



pub fn get_cwd(&self)->String{
match env::current_dir(){
Ok(cwd)=> format!("{}",cwd.display()),
Err(e)=> format!("{}",e)
}

}


pub fn get_dir_items(&self, path:&String)->
Vec<String>{
let  v_path: Vec<_>= fs::read_dir(path)
.expect("read_dir")
.map(|j|{
if let Ok(k)= j{

String::from(
k.path()
.to_str()
.expect("no str1"))
}else{
panic!("hlo");
}
 }).collect(); 
v_path

}


pub fn get_files_paths(&self,path:&mut Vec<String>)->
Vec<String>{
let mut v: Vec<_> = Vec::new();
for i in path{
let mypath : &path::Path = path::Path::new(&i[..]);
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
/*let v: Vec<_> = fs::read_dir(path)?.map(|i|{
if let Ok(j)= i{

format!("{} is_dir={}",String::from(
j.path()
.to_str()
.expect("no str")),j.path().is_dir())
}else{
panic!("hlo");
}
}).collect();
Ok(v)
//Ok(vec!["hlo".to_string()])

*/
}


pub fn files_show(&self,files_list:&Vec<String>){
for i in 0..files_list.len(){
let path: &path::Path = path::Path::new(
&files_list[i][..]);
println!("Press {} to  edit {}",i,
path.file_name().unwrap().to_str().unwrap());


 }
println!("Press {} to exit" , files_list.len());
}

pub fn mainloop(&self,text:&str,path:&String){
let paths: Vec<_> =self.get_files_paths(&mut self
.get_dir_items(path));
self.files_show(&paths);
let mut num : usize= inputnum!(text);

 if num<paths.len(){

process::Command::new("nano")
.arg(&paths[num][..]).spawn()
.expect("Command not working");
//self.files_show(&paths);
//println!("{}",&paths[num][..]);
self.mainloop(text,path);
}else if num==paths.len(){

}else{

println!("Please enter the number from the given list ");
self.mainloop(text,path);
//self.files_show(&paths);
}

 
//num=inputnum!(text);


}
}
