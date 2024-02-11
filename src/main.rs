use gtk::{gio::ActionEntry, glib, glib::clone, prelude::*};

const MARGIN: i32 = 20;
const APP_ID: &str = "nwegy.simple_gtk_login";

struct Credentials<'a> {
    id: &'a str,
    pass: &'a str,
}

fn build_ui(app: &gtk::Application) {
    let original_state = Credentials { id: "", pass: "" };
    let main_grid = gtk::Grid::builder().vexpand(true).hexpand(true).build();
    let input_grid = gtk::Grid::builder().vexpand(true).hexpand(true).build();
    let button_grid = gtk::Grid::builder().vexpand(true).hexpand(true).build();
    let user_label = gtk::Label::builder()
        .label("User:")
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN)
        .build();
    let pass_label = gtk::Label::builder()
        .label("Password:")
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN)
        .build();
    let user_text = gtk::Text::builder()
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN)
        .build();
    let pass_text = gtk::Text::builder()
        .input_purpose(gtk::InputPurpose::Password)
        .visibility(false)
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN)
        .build();
    let cancel_btn = gtk::Button::builder()
        .label("cancel")
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN)
        .hexpand(true)
        .build();
    let ok_btn = gtk::Button::builder()
        .label("ok")
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN)
        .hexpand(true)
        .build();
    input_grid.attach(&user_label, 0, 0, 1, 1);
    input_grid.attach(&pass_label, 0, 1, 1, 1);
    input_grid.attach(&user_text, 1, 0, 1, 1);
    input_grid.attach(&pass_text, 1, 1, 1, 1);
    button_grid.attach(&cancel_btn, 0, 2, 1, 1);
    button_grid.attach(&ok_btn, 1, 2, 1, 1);
    main_grid.attach(&input_grid, 0, 0, 1, 1);
    main_grid.attach(&button_grid, 0, 1, 1, 1);
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .child(&main_grid)
        .title("Login")
        .build();
    cancel_btn.connect_activate(clone! (@weak window => move |_| window.close()));
    // let action_set_cred = ActionEntry::builder("set_cred").build();
    window.present();
    // todo!("Setup ACtuion for adding changing ID and Password");
}

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}
