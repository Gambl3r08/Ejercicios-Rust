use std::fs::File;

fn main(){
    let _f = File::open("file.txt");
    
    let _f = match _f{
        Ok(file) => file,
        Err(why) => panic!("Error al abrir el archivo {:?}", why),
    };
   
}

