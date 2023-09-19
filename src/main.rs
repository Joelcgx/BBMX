//Mods
mod class;

use class::load_ui::load_ui_main_window;
//LIbrerias
use gtk::prelude::*;
use gtk::{ Builder, Dialog, Button };

//Structura de los widgets
struct MyGladeUI {
    dialog: gtk::Dialog,
    button_cancel: gtk::Button,
    button_ok: gtk::Button,
}
//fucion Main
fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK");
        return;
    }
    //Load ui
    let ui = load_ui();

    //Matar proceso al cerrar la ventana
    ui.dialog.connect_delete_event(|_, _| {
        gtk::main_quit();
        glib::Propagation::Stop
    });

    //Al cilck del boton Cancelar para cerrar el form
    let dialog = ui.dialog.clone();
    ui.button_cancel.connect_clicked(move |_| {
        dialog.close();
        gtk::main_quit();
    });
    //Boton Log in
    ui.button_ok.connect_clicked(move |_| {
        load_ui_main_window();
        
    });

    ui.dialog.show_all();

    gtk::main();
}
//Funcion para cargar la ui
fn load_ui() -> MyGladeUI {
    //Glade file
    let ui_src = include_str!("./ui/form_init.glade");
    let builder = Builder::from_string(ui_src);

    //Widgets id
    let dialog: Dialog = builder.object("dialog1").expect("No se pudo encontrar el dialog");
    let button_cancel: Button = builder
        .object("btn_cancelar")
        .expect("No se pudo encontrar el btn_cancelar");
    let button_ok: Button = builder
        .object("log_in_btn")
        .expect("No se pudo encontrar el log_in_btn");

    MyGladeUI {
        dialog,
        button_cancel,
        button_ok,
    }
}
