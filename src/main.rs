use clap::{ Args, Command, CommandFactory, Parser, Subcommand }; 

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLIUtils {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Echo(EchoArgs), 
    Grep(GrepArgs), 
    Find(FindArgs),
    Ls(LsArgs),
    Cat(CatArgs)
}

#[derive(Args)]
struct EchoArgs {
    output: String
}

#[derive(Args)]
struct GrepArgs {
    needle: String, 
    filename: String, 
}

#[derive(Args)]
struct FindArgs {
    path: String
}

#[derive(Args)]
struct LsArgs {
    dir_path: String
}

#[derive(Args)]
struct CatArgs {
    file1: String, 
    file2: String 
}

mod operations {
    use std::path::Path; 
    use std::fs; 

    type StdResult<T> = std::io::Result<T>; 

    fn is_file(path: &String) -> bool {
        let path: &Path = Path::new(path); 
        
        path.is_file()
    }

    fn read_file(file_path: &String) -> String {
        if !is_file(file_path) {
            return String::new(); 
        }

        let file_contents: String = fs::read_to_string(&file_path).unwrap(); 

        file_contents
    }

    pub fn list_dir(dir_path: String) -> StdResult<()> {
        println!("Currently In: {}", dir_path); 
        for dir_entry in std::fs::read_dir(dir_path)? {
            let entry_name = dir_entry.unwrap().file_name().into_string().unwrap();
            let path: &Path = Path::new(&entry_name); 

            if path.is_dir() {
                println!("<DIR> {entry_name}"); 
            } else {
                println!("{entry_name}"); 
            }
        }   

        Ok(())
    } 

    pub fn grep_impl(file_path: String, needle: String) -> StdResult<()> {
        if is_file(&file_path) {
            // Opening file here 
            let file_content: String = fs::read_to_string(file_path)?; 

            match file_content.find(needle.as_str()) {
                Some(index) => {
                    println!("Match found at {}", index); 
                }, 
                None => {
                    println!("Match not found"); 
                }
            }
        } else {
            println!("The path entered is not a file"); 
        }

        Ok(())
    }

    pub fn find_file_or_dir(path: String) -> StdResult<()> {
        let _path: &Path = Path::new(&path); 

        if _path.exists() {
            println!("Path found at {}", _path.to_string_lossy()); 
        } else {
            println!("Path `{path}` does not exist");
        }

        Ok(())
    }

    pub fn concatenate_files(file_path1: String, file_path2: String) -> StdResult<()> {
        if is_file(&file_path1) && is_file(&file_path2) {
            let (file_contents1, file_contents2) = (read_file(&file_path1), read_file(&file_path1)) ;

            let concatenated_file_content: String = format!("{file_contents1}\n{file_contents2}");

            println!("Concatenated {file_path1} and {file_path2} \n{concatenated_file_content}"); 
        } else {
            println!("An error occured!"); 
            println!("Either of the paths may not exist, or is not a file"); 
        }

        Ok(())
    }

    // pub fn 
}


fn main() {
    let app = CLIUtils::parse(); 

    match app.command {
        Commands::Echo(args) => {
            println!("{}", args.output);
        }, 
        Commands::Ls(args) => {
            let _ = operations::list_dir(args.dir_path); 
        }, 
        Commands::Grep(args) => {
            let _ = operations::grep_impl(args.filename, args.needle);
        }, 
        Commands::Find(args) => {
            let _ = operations::find_file_or_dir(args.path); 
        }, 
        Commands::Cat(args) => {
            let _ = operations::concatenate_files(args.file1, args.file2); 
        }
    }
}