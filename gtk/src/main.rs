extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType, Label, Entry, Box as GtkBox, Orientation};

fn main() {
    if gtk::init().is_err(){
        panic!("No se pudo iniciar GTK");
    }

    let window = Window::new(WindowType::Toplevel);

    window.connect_delete_event(|_,_|{gtk::main_quit(); Inhibit(false) });
    window.set_title("Beta");
    window.set_default_size(300, 300);
    let label = Label::new(Some("Texto"));
    
    // Creando VBox con 10px de espacio
    let bx = GtkBox::new(Orientation::Vertical, 10);
    let entry = Entry::new();

    //Conectando la señal de activación a la funcion anonima
    entry.connect_activate(|x| println!("{}",x.get_text().unwrap() ));

    //Añadiendo al label
    bx.pack_start(&label, false, false, 0);
    bx.pack_start(&entry, false, false, 0);


    window.add(&bx);
    window.show_all();
    gtk::main();
}
