use std::sync::mpsc::channel;
use std::thread::{self, JoinHandle};


// https://stackoverflow.com/questions/72493266/how-to-loop-over-thread-handles-and-join-if-finished-within-another-loop

const NUM_VALUES : usize = 30;
const NUM_THREADS : usize = 10;

#[derive(Default)]
struct ThreadResult{
    num_thread:usize,
    result: u128
}


fn init_numbers(num: &mut Vec::<u128>){

    for i in 0..NUM_VALUES{
        num.push((i + 1) as u128);
    }
}

fn create_threads(numbers: &mut Vec::<u128>){
    
    //Número de factoriales calculados.
    let mut num_values_calculated: usize = 0;
    //Número de valores enviados a los threads.
    let mut num_values_sent : usize = 0;
    //Número de threads ejecutandose.
//    let mut num_threads_running: usize = 0;
    //La lista de handlers de los vectores.
//    let mut handlers_list = Vec::<JoinHandle<Result>>::new();
//    let mut handlers_list = Vec::new();
    let mut handlers_list = Vec::<Option<JoinHandle<ThreadResult>>>::new();

    /*
        Para abordarlo vamos a intentar lo siguiente, 
        Tenemos un thread que se encargara de crear otros threads. En cuanto acabe un thread le notificará al thread padre que ha acabado mediante un channel

        RECUERDA: 
        en los channels, pueden hacer N emisores y un único receptor por lo que todos los threads pueden usar una copia de un mismo emisor.
     */
    //Creo el canal de comunicaciones de los threads.
    let (send_finished_thread, recv_finished_thread)  = std::sync::mpsc::channel::<usize>();
    //Creo los n primeros threads.
    for i in 0..NUM_THREADS{
        
        let value;
        match numbers.get(num_values_sent)
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
                thread::spawn(move || {
                    let mut factorial_result:u128 = 1;
                    for i in 0..value{
                        factorial_result = factorial_result * ((i + 1) as u128);
                    }
                    //Comunico que he acabado...
                    match  send_finished_thread.send(num_values_sent){
                        Ok(_) => (),
                        Err(e) => panic!("ERROR: error was ocurred while comunicating between threads {e:?}"),
                    }
                    //Lleno la estructura de retorno
                    ThreadResult{ num_thread : num_values_sent, result : factorial_result }
                })
            )
        );
        num_values_sent = num_values_sent + 1;
    }
    
    loop{
        //Si ya he calculado todos los numeros, salgo del bucle.
        if num_values_calculated >= NUM_VALUES{
            break;
        }
        //Me quedo escuchando que thread ha finalizado...
        match recv_finished_thread.recv(){
            Ok(_num_value) => {
                /*  
                //Obtengo el valor, lo guardo en el vector y creo otro thread.
                match handlers_list.get(_num_value){
                    Some(_handler) =>{
                        match (*_handler).join(){
                            Ok(_thread_result) => {

                            },
                            Err(e) => panic!("ERROR: error was ocurred while joinning threads"),
                        }
                    },
                    None => panic!("ERROR: We got a thread value that does NOT exist"),
                }
                */ 
                //Con Take, obtengo el valor del handler y lo seteo a None en el vector.
                match std::mem::take(&mut handlers_list[_num_value]){
                    None => (),
                    Some(_thread_result) =>{
                        //Hago el join.
                        match _thread_result.join(){
                            Err(e) => panic!("ERROR: error was occurred while joinning threads."),
                            Ok(th_res) => {
                                //Actualizo el valor de numeros calculados
                                num_values_calculated = num_values_calculated + 1;
                                //Print del resultado
                                println!("\t\t Nº Thread : {} resultado \t{}", th_res.num_thread, th_res.result);
                                //Reviso si he acabado, en caso de NO acabar, lanzo otro thread. 
                                if num_values_sent < NUM_VALUES{
                                    //Como estamos en un bucle, se pasa la propiedad del send al primer bucle  A NO SER QUE hagamos un clone en este scope.
                                    let send_finished_thread = send_finished_thread.clone();
                                    let value;
                                    match numbers.get(num_values_sent)
                                    {
                                        None => panic!("ERROR: error was occurred while getting position {num_values_sent}  of vector of numbers with len {}",numbers.len()),
                                        Some(_value) => value = *_value,
                                    }
                                    //Hago la llamada y hago push del handler.
                                    handlers_list.push(
                                    //Necesito un Some(JoinHandler<thread_result>) por ello , lo casteo a Some.
                                    Some(
                                            thread::spawn(move || {
                                                let mut factorial_result:u128 = 1;
                                                for i in 0..value{
                                                    factorial_result = factorial_result * ((i + 1) as u128);
                                                }
                                                //Comunico que he acabado...
                                                match  send_finished_thread.send(num_values_sent){
                                                    Ok(_) => (),
                                                    Err(e) => panic!("ERROR: error was ocurred while comunicating between threads {e:?}"),
                                                }
                                                //Lleno la estructura de retorno
                                                ThreadResult{ num_thread : num_values_sent, result : factorial_result }
                                            })
                                        )
                                    );
                                    num_values_sent = num_values_sent + 1;  
                                }
                            },
                        }
                    }
                }
            },
            Err(e) =>  panic!("ERROR: Error while joinning threads {e:?}"),
        }
    }

    while num_values_calculated < NUM_THREADS{
        for i in 0..NUM_THREADS{

        }
        num_values_calculated = num_values_calculated + 1;
    }
}

fn main() {

    let mut numbers =  Vec::<u128>::new();
    
    println!("Hello, world!");
    init_numbers(&mut numbers);
    println!("\tVector inicial:\t{numbers:?}");
    create_threads(&mut numbers);
    println!("\tVector final:\t{numbers:?}");
    println!("Bye, world!");
}
