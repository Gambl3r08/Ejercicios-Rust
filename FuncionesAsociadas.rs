#[derive(Debug)]
struct Circulo {
	x: f64,
	y: f64,
	radio: f64,
}

impl Circulo {
	fn new(x: f64, y: f64, radio: f64) -> Circulo{
		Circulo{
			x: x,
			y: y,
			radio: radio,
		}
	}

	fn mostrar(&self){
		println!("x: {}", self.x);
		println!("y: {}", self.y);
		println!("radio: {}", self.radio);
	}
}

fn main() {
	let c = Circulo::new(0.0, 0.0, 2.0);
	c.mostrar();
}