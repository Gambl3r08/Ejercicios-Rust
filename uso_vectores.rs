fn main(){
    let mut vector = Vec::new();
    vector.push(20);
    vector.insert(0, 10);
    let mut cont = 0;
    for i in &vector {

        println!("pos: {} valor: {:?}", cont, i);
        cont = cont + 1;
    }
}