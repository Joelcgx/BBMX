//Mods
mod class;
// funciones de otros archivos
use class::conex_db::{ verificar_user };
//LIbrerias
use gtk::prelude::*;
use gtk::{ Builder, Dialog, Button, MessageDialog, MessageType, DialogFlags, ButtonsType, Entry };
use gdk_pixbuf::Pixbuf;

//Structura de los widgets
struct MyGladeUI {
    dialog: gtk::Dialog,
    button_cancel: gtk::Button,
    button_ok: gtk::Button,
    user: gtk::Entry,
    pass: gtk::Entry,
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
        let waring = MessageDialog::new(
            Some(&dialog),
            DialogFlags::MODAL,
            MessageType::Warning,
            ButtonsType::OkCancel,
            "Esta seguro que desea salir?"
        );
        waring.set_title("Desea Salir?");
        let pixbuf = Pixbuf::from_file("./src/ui/icons/iconApp.jpg").unwrap();
        waring.set_icon(Some(&pixbuf));

        //Mostrar dialogo y esperar una respuesta
        let response = waring.run();
        match response {
            gtk::ResponseType::Ok => {
                gtk::main_quit();
            }
            gtk::ResponseType::Cancel => {
                waring.close();
            }
            _ => {}
        }

        unsafe { waring.destroy() }
    });
    //Boton Log in
    ui.button_ok.connect_clicked(
        move |_| {
            //Verificar
            let u = ui.user.text();
            let p = ui.pass.text();
            verificar_user(&u, &p).unwrap();
        }
    );
    //Icono ala dialog
    let icon_app = Pixbuf::from_file("./src/ui/icons/iconApp.ico").unwrap();
    ui.dialog.set_icon(Some(&icon_app));

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
    let user: Entry = builder.object("user_entry1").expect("No se pudo encontrar el user_entr");
    let pass: Entry = builder.object("passw_entry").expect("No se pudo encontrar el pass_entr");
    
    MyGladeUI {
        dialog,
        button_cancel,
        button_ok,
        user,
        pass,
    }
}
