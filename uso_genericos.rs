fn main() {
	#[derive(Debug)]
	enum Option<T> {
		Some(T),
		None,
	}


	let x: Option<i32> = Some(5); // Puede almacenar depende de su valor
	let y: Option<f64> = Some(5.0f64); 

  //let z: Option<f64> = Some(4) esto daria error pues 4 no es un f64


    //Funciones genericas

    fn recibe_loquesea<T>(x: T){
		//foo    	
    }

}