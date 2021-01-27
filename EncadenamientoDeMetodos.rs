#[derive(Debug)]
struct Circulo {
	x: f64,
	y: f64,
	radio: f64,
}

impl Circulo {
	fn area(&self) -> f64{
		std::f64::consts::PI * (self.radio * self.radio)
	}

	fn agrandar(&self, incremento: f64) -> Circulo{
		Circulo{x: self.x, y: self.y, radio: self.radio + incremento}
	}
}


fn main() {
	let c = Circulo{x: 0.0, y: 0.0, radio: 2.0};
	println!("{}", c.area());

	let d = c.agrandar(2.0).area();
	println!("{}", d);
}