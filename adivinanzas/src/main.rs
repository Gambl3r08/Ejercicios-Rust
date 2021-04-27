extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el numero!");

    let num_secreto = rand::thread_rng().gen_range(1..101);


    loop {
        println!("Por favor introduce un numero:");
        let mut entrada_num = String::new();
        io::stdin().read_line(&mut entrada_num).ok().expect("Falló al leer la linea");
        let num: u32 = match entrada_num.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match num.cmp(&num_secreto) {
            Ordering::Less => println!("Muy pequeño"),
            Ordering::Greater => println!("Muy grande"),
            Ordering::Equal => {
                println!("Haz ganado");
                break;
            }
        }
    }
}