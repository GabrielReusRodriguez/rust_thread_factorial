use std::sync::mpsc::Sender;

use crate::thread_manager::thread_result;
 
pub fn factorial_function(
        value : u64,
        channel_sender: Sender<u64>,
        n_thread: u64
    )-> thread_result::ThreadResult
{
    let mut factorial_result:u128 = 1;
    for i in 0..value{
        factorial_result = factorial_result * ((i + 1) as u128);
    }
    //Comunico que he acabado...
    match  channel_sender.send(n_thread){
        Ok(_) => (),
        Err(e) => panic!("ERROR: error was ocurred while comunicating between threads {e:?} thread : {n_thread}"),
    }
    //println!("\tProcesando el thread numero {n_thread}");
    //Lleno la estructura de retorno
    thread_result::ThreadResult { num_thread: n_thread, result: factorial_result }
}