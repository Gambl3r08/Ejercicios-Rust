use std::thread;
use std::time::Duration;

fn my_thread(){
    println!("El hilo {:?} se esta ejecutando", std::thread::current().id());
    thread::sleep(Duration::from_millis(1));
}

fn main(){
    let mut v = vec![];

    for _i in 1..10{
        v.push(thread::spawn(||{my_thread();}));
    }

    println!("main() en espera.");

    for child in v{
        match child.join(){
            Ok(_) => (),
            Err(why) => print!("Fallo conjunto {:?}", why),
        };
    }
}