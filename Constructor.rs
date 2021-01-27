struct Circulo {
	x: f64,
	y: f64,
	radio: f64,
}


impl Circulo {
	fn area(&self) -> f64{
		std::f64::consts::PI*(self.radio*self.radio)
	}
}


struct ConstructorCirculo {
	x: f64,
	y: f64,
	radio: f64,
}

impl ConstructorCirculo {
	fn new() -> ConstructorCirculo{
		ConstructorCirculo{x: 0.0, y: 0.0, radio: 1.0,}
	}

	fn x(&mut self, coordenada: f64)-> &mut ConstructorCirculo{
		self.x = coordenada;
		self
	}

	fn y(&mut self, coordenada: f64)-> &mut ConstructorCirculo{
		self.y = coordenada;
		self
	}

	fn radio(&mut self, radio: f64) -> &mut ConstructorCirculo{
		self.radio = radio;
		self
	}

	fn finalizar(&self) -> Circulo{
		Circulo{x: self.x, y: self.y, radio: self.radio}
	}
}

fn main() {
	let c = ConstructorCirculo::new().x(1.0).y(2.0).radio(2.0).finalizar();

	println!("Area : {}", c.area());
	println!("x: {}", c.x);
	println!("y: {}", c.y);
}
