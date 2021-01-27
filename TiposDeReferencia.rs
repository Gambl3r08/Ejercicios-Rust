#[derive(Debug)]
struct Circulo {
	x: f64,
	y: f64,
	radio: f64,
}

impl Circulo {
	fn referencia(&self){
		println!("Recibiendo a self como referencia");
	}

	fn referencia_mutable(&mut self){
		println!("Recibiendo a self como referencia mutable");
	}

	fn toma_pertenencia(self){
		println!("Tomando pertenencia de self");
	}
}

fn main() {
	referencia();
	referencia_mutable();
	toma_pertenencia();
}