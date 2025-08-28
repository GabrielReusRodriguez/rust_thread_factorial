//use std::fs;

#[allow(dead_code)]

pub fn load_data_file( file: &str, data: &mut  Vec::<u128> ) -> Result<&'static str, String>{

    let file_reader  = std::fs::read_to_string(file);
    //println!("ERROR: {file_reader:?}");
    match file_reader
    {
        Ok(_content) => {
            match parse_data(_content, data){
                Ok(_) => return Ok("Ok"),
                Err(_error) => return Err(_error),
            }
        },
        Err(_error) =>{
            return Err(_error.to_string());
        },
    }
}

fn parse_data(file_text: String, data: &mut Vec<u128>) -> Result<&str, String>{

    let numbers = file_text.split("\n");
    for number in numbers{
        let unsigned = number.trim().parse::<u128>();
        match unsigned{
            Ok(_num) =>{
                data.push(_num);
            },
            Err(_error) => {
                let  _msg = "Error parsing number".to_owned() + " '" + number + "'";
                println!("ERROR Parsing {_msg}");
                return Err(_msg)
            }
        }
    }
    Ok("Ok")
}

/* 
pub fn write_data_file( file: &str, data: &Vec::<u128>){

}
*/

pub fn init_numbers(num: &mut Vec::<u128>, size: u64){

    for i in 0..size{
        num.push((i + 1) as u128);
    }
}
