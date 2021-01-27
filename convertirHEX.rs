mod bits{
    fn pos(bit: u32) -> u32{
        1 << bit
    }

    pub fn decimal(bit: u32){
        println!("Un decimal de bits {}", pos(bit));
    }

    pub fn hex(bit: u32){
        println!("Decimal de bits 0x{:x}", pos(bit));
    }

}


fn main(){
    bits::decimal(8);
    bits::hex(16);
}