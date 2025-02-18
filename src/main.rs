use std::{
    fs::{self, File, OpenOptions},
    io::{self,Write},
    path::Path, ptr::null,
};

struct CLI;

impl CLI {
    fn check_args_len(args_length:usize, length:usize, message:String) -> io::Result<()>{
        if args_length <= length{
            eprintln!("{}",message);
            return Ok(());
        }
        Ok(())
    }
    fn create_file(path:&str) -> io::Result<()>{
        if Path::new(path).exists(){
            return Ok(());
        }
        File::create(path)?;
        println!("Successfully created {}", path);
        Ok(())
    }

    fn read(path:&str)->io::Result<()>{
        if !Path::new(path).exists(){
            return Ok(());
        }
        let read: String = fs::read_to_string(path).expect("cant read file");
        println!("{}", read);
        Ok(())
    }
    
    fn write(path:&str, message:&str)->io::Result<()>{
        if !Path::new(path).exists(){
            return Ok(());
        }
        let mut  data_file = OpenOptions::new().append(true).open(path).expect("cant open file");
        data_file.write(&message.as_bytes()).expect("cant write to file");
        Ok(())
    }       
    fn delete(path:&str, message:Option<&str>)->io::Result<()>{
        if !Path::new(path).exists(){
            eprint!("file doesnt exist");
            return Ok(());
        }
        if message.is_none() {
            let _ = fs::remove_file(path);
        }else {
            
        }
        
        Ok(())
    }
}   

fn main() ->io::Result<()>{
    let args: Vec<String> = std::env::args().collect();
    let _ = CLI::check_args_len(args.len(), 1, "not a command".to_string());
    let command: &String = &args[1];
    match &command[..] {
        "create" => {
            CLI::create_file(&args[2])
        }
        "read" => {
            CLI::read(&args[2])
        }
        "write" => {
            let message = args[3..].join(" ");
            CLI::write(&args[2],&message)
        }
        _=>{
            Ok(())
        }
    }


}
