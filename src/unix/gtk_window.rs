extern crate gtk;
extern crate gdk;
use self::gtk::prelude::*;
use self::gdk::WindowExt;

use std::error::Error;

use std::rc::Rc;
use std::cell::RefCell;

use *;

pub struct Window {
    app: Rc<RefCell<App>>,
    window: gtk::Window
}

impl Window {
    pub fn new<A: App + 'static, F: FnOnce(&mut RenderContext)->A>(title: &str, width: usize, height: usize, appf: F) -> Result<Self, Box<Error>> {
        gtk::init().map_err(|_| "failed to init GTK")?; //probably there is a better place for this
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        let drawing_area = DrawingArea::new();

        let dc = window.begin_draw_frame(&cairo::Region::create());
        let rx = RenderContext { cx: dc.get_cairo_context() };
        let app: Rc<RefCell<App>> = Rc::new(RefCell::new(appf(&mut rx)));
        window.end_draw_frame(&dc);

        drawing_area.connect_draw(|a, c| {
        });

        window.set_default_size(width as i32, height as i32);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        Ok(Window { app, window })
    }

    pub fn show(&mut self) {
        self.window.show_all();
        gtk::main();
    }
}
