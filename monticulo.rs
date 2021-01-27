fn suma(i: i32, j: i32){
     println!("Suma: {}", i + j);
}

fn imprimir_suma(x: i32, y: i32) {
    println!("la suma es: {}", x + y);
}

fn main() {
    let x = Box::new(5);
    println!("{}",x);
    suma(5, 19);
    imprimir_suma(5, 19);
}