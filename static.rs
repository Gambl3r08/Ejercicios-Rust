fn main(){
    let x: &'static str = "Hello world!.";
    static FOO: i32 = 5;
    let y: &'static i32 = &FOO;
}