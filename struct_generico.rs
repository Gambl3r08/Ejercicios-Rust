fn main() {
	#[derive(Debug)]
	struct Punto<T> {
		x: T,
		y: T,
	}


	let Origen_int = Punto{x: 0, y: 0};
	let Origen_float = Punto{x: 0.0, y: 0.0};

	println!("{:?}", Origen_int);
	println!("{:?}", Origen_float);
}