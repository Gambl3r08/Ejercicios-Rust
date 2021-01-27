fn main(){
    let a = 1;
    let ramp = &a as *const i32;

    unsafe{
        println!("ramp es {}", *ramp);
    }
}