use std::fs;

const INPUTS_DIR: &str = "./inputs";

pub fn read(day: i32) -> String
{
    let file_path = format!("{}/day{}.txt", INPUTS_DIR, day);
    fs::read_to_string(file_path).unwrap()
}
