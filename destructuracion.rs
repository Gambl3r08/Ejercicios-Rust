fn main(){
    struct Punto{
        x: i32,
        y: i32,
    }

    let origen = Punto{x: 0, y: 0};

    match origen{
        Punto{x, y} => println!("({},{})", x, y),
    }
}