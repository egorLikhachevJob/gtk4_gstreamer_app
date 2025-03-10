mod custom_paintable;

use custom_paintable::CustomPaintable;
use gtk4::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk4::Application::builder()
        .application_id("com.github.gtk4-rs.examples.paintable")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    window.set_title(Some("Custom Paintable"));
    window.set_default_size(500, 500);

    let paintable = CustomPaintable::default();

    let picture = gtk4::Picture::new();
    picture.set_halign(gtk4::Align::Center);
    picture.set_size_request(200, 200);
    picture.set_paintable(Some(&paintable));

    window.set_child(Some(&picture));
    window.present();
}
