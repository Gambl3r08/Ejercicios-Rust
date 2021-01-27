fn recibe_texto(texto: &str){
	println!("recibi: {}", texto);
}
fn main() {
	let text = "xxxxxxx".to_string();
	recibe_texto(&text);
}