//Librerias
use gtk::prelude::*;
use gtk::{ Builder, ApplicationWindow };

use super::conex_db::log_in;


//Cargar Main Window
pub fn load_ui_main_window() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK");
        return;
    }
    let main_window_src = include_str!("../ui/main_window.glade");
    let builder = Builder::from_string(main_window_src);

    //Main Window
    let main_window: ApplicationWindow = builder
        .object("main_window")
        .expect("No se pudo encontrar el main_window");

    main_window.show_all();
    log_in("username", "jj");
    
}
