use std::{fs::File, io::Read}; 

pub fn parse_command<'a>(cmd: &'a str, value: &'a str, path: Option<&'a str>) -> Option<String> {
    match value {
        "echo" => Some(value.to_owned()), 
        "find" => find(value), 
        "grep" => grep(path, value), 
        _ => None
    }
}

pub fn find<'a>(value: &'a str) -> Option<String> {
    Some(value.to_owned())
}

pub fn grep<'a>(path: Option<&'a str>, value: &'a str) -> Option<String> {
    let file_content = open_file(path).ok();
    if let Some(file_content) = file_content {
        let results = file_content.find(value);
        let match_index = results.clone()?;
       return Some(format!("Match found at {match_index}"));
    } else {
        return None;
    }
}

fn open_file<'a>(path: Option<&'a str>) -> std::io::Result<String> {
    let mut file = File::open(path.unwrap())?;
    let mut file_contents = String::new(); 
    file.read_to_string(&mut file_contents); 
    
    Ok(file_contents)
}