use failure::Error;
use gdk::ContextExt;
use gdk_pixbuf::Pixbuf;
use gtk::{
    self, ContainerExt, DrawingArea, GtkWindowExt, Inhibit, WidgetExt, Window as GtkWindow,
    WindowType,
};

pub fn select_area(pixbuf: Pixbuf) -> Result<Pixbuf, Error> {
    let window = GtkWindow::new(WindowType::Popup);
    let mut pixbuf2 = pixbuf.clone();
    let drawing_area = Box::new(DrawingArea::new)();
    drawing_area.connect_draw(move |_, cr| {
        cr.set_source_pixbuf(&pixbuf, 0.0, 0.0);
        cr.paint();

        Inhibit(false)
    });
    window.add(&drawing_area);

    window.connect_key_release_event(|_, key| {
        println!("{:?}", key.get_keyval());
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
    Ok(pixbuf2)
}
