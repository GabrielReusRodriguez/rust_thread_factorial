

pub fn load_data_file( file: &str, data: &mut  Vec::<u128> ){

}

pub fn write_data_file( file: &str, data: &Vec::<u128>){

}

pub fn init_numbers(num: &mut Vec::<u128>, size: u64){

    for i in 0..size{
        num.push((i + 1) as u128);
    }
}
