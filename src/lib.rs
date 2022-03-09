

extern crate regex;
use regex::{Captures, Regex};


pub fn title_case(input:String)->String{
    let  words: Regex = Regex::new(
        r"(?x)
            (_*)
            ([\w'’.:/@\[\]/()]+)
            (_*)",
    )
    .expect("unable to compile regex");
    
    let result = input.trim().to_string();
    let result = words.replace_all(&result, |captures: &Captures| {
        let mut result = captures
            .get(1)
            .map(|cap| cap.as_str())
            .unwrap_or("")
            .to_owned();
        let word = &captures[2];
        result.push_str(&capitalize(word.to_string()));
        result.push_str(captures.get(3).map(|cap| cap.as_str()).unwrap_or(""));
        result
    }).to_string();
    return result;
}

pub fn capitalize(value:String) -> String {
    let mut chars = value.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
pub fn uppercase(value:String) -> String {
    return value.to_uppercase();
}
pub fn lowercase(value:String) -> String {
    return value.to_lowercase();
}

pub fn slugify(input:String) -> String {
    return input.trim().chars()
    .map(|x| match x { 
        ' ' => '-', 
        '_' => '-', 
        _ => x
    }).collect::<String>().to_string().to_lowercase();
}

pub fn snake_case(input:String) -> String {
    let result =  input.trim().chars()
    .map(|x| match x { 
        ' ' => '_', 
        '-' => '_', 
        _ => x
    }).collect::<String>().to_string().to_lowercase();
    return  result;
}



pub fn camel_case(input:String)->String{
    let  words: Regex = Regex::new(
        r"(?x)
            (_*)
            ([\w'’.:/@\[\]/()]+)
            (_*)",
    )
    .expect("unable to compile regex");
    
    let result = input.trim().to_string();
    let mut result = words.replace_all(&result, |captures: &Captures| {
        let mut result = captures
            .get(1)
            .map(|cap| cap.as_str())
            .unwrap_or("")
            .to_owned();
        let word = &captures[2];
        result.push_str(&capitalize(word.to_string()));
        result.push_str(captures.get(3).map(|cap| cap.as_str()).unwrap_or(""));
        result
    }).to_string();

    // result =  match result.chars().next() {
    //     None => String::new(),
    //     Some(f) => f.to_lowercase().collect::<String>() + result.chars().as_str(),
    // };
    result = result.trim().chars()
    .filter(|c| !c.is_whitespace()).collect::<String>();
    return result;
}
