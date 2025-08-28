#![allow(dead_code)]
#![allow(unused)]

//use std::sync::mpsc::channel;
use std::thread::{self, JoinHandle};

use crate::data::loader::load_data_file;

//use crate::ThreadManager::thread_logic;

mod thread_manager;
mod data;

//use ThreadManager::thread_logic;


//#[allow(dead_code)]
//#[allow(unused)]

// https://stackoverflow.com/questions/72493266/how-to-loop-over-thread-handles-and-join-if-finished-within-another-loop

const NUM_VALUES : u64= 30;
const NUM_THREADS : usize = 10;


fn create_threads(numbers: &mut Vec::<u128>){
    
    //Número de factoriales calculados.
    let mut jobs_done: usize = 0;
    let mut jobs_sent: u64 = 0;
    //Número de valores enviados a los threads.
    let mut handlers_list = Vec::<Option<JoinHandle<thread_manager::thread_result::ThreadResult>>>::new();

    /*
        Para abordarlo vamos a intentar lo siguiente, 
        Tenemos un thread que se encargara de crear otros threads. En cuanto acabe un thread le notificará al thread padre que ha acabado mediante un channel

        RECUERDA: 
        en los channels, pueden hacer N emisores y un único receptor por lo que todos los threads pueden usar una copia de un mismo emisor.
     */
    //Creo el canal de comunicaciones de los threads.
    let (send_finished_thread, recv_finished_thread)  = std::sync::mpsc::channel::<u64>();
    //Creo los n primeros threads.
    for i in 0..NUM_THREADS{
        
        let value;
        match numbers.get(jobs_sent as usize)
        {
            None => panic!("ERROR: error was occurred while getting position {i}  of vector of numbers with len {}",numbers.len()),
            Some(_value) => value = *_value,
        }
        //Como estamos en un bucle, se pasa la propiedad del send al primer bucle  A NO SER QUE hagamos un clone en este scope.
        let send_finished_thread = send_finished_thread.clone();
        //Hago la llamada y hago push del handler.
        handlers_list.push(
            //Necesito un Some(JoinHandler<thread_result>) por ello , lo casteo a Some.
            Some(
                thread::spawn(move || thread_manager::thread_logic::factorial_function(
                    value as u64, 
                    send_finished_thread,
                    jobs_sent
                ))
            )
        );
        jobs_sent = jobs_sent + 1;
    }
    
    loop{
        //Si ya he calculado todos los numeros, salgo del bucle.
        //println!("JOBS: {jobs_done} num_values : {}", NUM_VALUES as usize);
        if jobs_done + 1 >= NUM_VALUES as usize{
            break;
        }
        //Me quedo escuchando que thread ha finalizado...
        match recv_finished_thread.recv(){
            Ok(_num_value) => {
                //Con Take, obtengo el valor del handler y lo seteo a None en el vector.
                match std::mem::take(&mut handlers_list[_num_value as usize]){
                    None => (),
                    Some(_thread_result) =>{
                        //Hago el join.
                        match _thread_result.join(){
                            Err(_e) => panic!("ERROR: error was occurred while joinning threads."),
                            Ok(th_res) => {
                                //Actualizo el valor de numeros calculados
                                jobs_done = jobs_done + 1;
                                //Print del resultado
                                println!("\t\t Nº Thread : {} resultado \t{}", th_res.num_thread, th_res.result);
                                //Reviso si he acabado, en caso de NO acabar, lanzo otro thread. 
                                if jobs_sent < NUM_VALUES{
                                    //Como estamos en un bucle, se pasa la propiedad del send al primer bucle  A NO SER QUE hagamos un clone en este scope.
                                    let send_finished_thread = send_finished_thread.clone();
                                    let value;
                                    match numbers.get(jobs_sent as usize)
                                    {
                                        None => panic!("ERROR: error was occurred while getting position {jobs_sent}  of vector of numbers with len {}",numbers.len()),
                                        Some(_value) => value = *_value,
                                    }
                                    //Hago la llamada y hago push del handler.
                                    handlers_list.push(
                                        //Necesito un Some(JoinHandler<thread_result>) por ello , lo casteo a Some.
                                        Some(
                                            thread::spawn(move || thread_manager::thread_logic::factorial_function(
                                                    value as u64, 
                                                    send_finished_thread,
                                                    jobs_sent
                                                )
                                            )
                                        ) 
                                    );
                                    jobs_sent = jobs_sent + 1;
                                }
                            },
                        }
                    }
                }
            },
            Err(e) =>  panic!("ERROR: Error while joinning threads {e:?}"),
        }
    }

}

fn main() {

    let mut numbers =  Vec::<u128>::new();
    let file_name = "gabriel.txt";
    match load_data_file("./data/numbers.txt", &mut numbers) {
        Ok(_) =>(),
        Err(_error) => { 
            panic!("ERROR loading data file {file_name} : {_error}");
        }
    }
    /* 
    println!("Hello, world!");
    data::loader::init_numbers(&mut numbers, NUM_VALUES);
    println!("\tVector inicial:\t{numbers:?}");
    create_threads(&mut numbers);
    */
    println!("\tVector final:\t{numbers:?}");
    println!("Bye, world!");
}