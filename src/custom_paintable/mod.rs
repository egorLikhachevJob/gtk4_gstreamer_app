mod imp;

use gtk4::{gdk, glib};

glib::wrapper! {
    pub struct CustomPaintable(ObjectSubclass<imp::CustomPaintable>) @implements gdk::Paintable;
}

impl Default for CustomPaintable {
    fn default() -> Self {
        glib::Object::new()
    }
}
