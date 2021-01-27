fn foo(_vector1: &Vec<i32>, _vector2: &Vec<i32>) -> i32 {
    // hacer algo con v1 y v2

    // retornando la respuesta
    42
}

fn main(){
    let _v1 = vec![1, 2, 3];
    let _v2 = vec![1, 2, 3];

    let _respuesta = foo(&_v1, &_v2);

// podemos usar a v1 y v2 aqui

// referencias mutables

    let mut x = 5;
        {
            let y = &mut x;
            *y += 1;
        }
    println!("{}", x);

}

