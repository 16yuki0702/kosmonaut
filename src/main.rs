pub mod dom;
pub mod frame;
#[allow(unused_imports)]
pub mod layout;
pub mod style;

#[macro_use]
extern crate matches;
#[macro_use]
extern crate html5ever;
#[macro_use]
extern crate cssparser;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Entry, Orientation};
use std::fs::File;
use std::io::Read;

use crate::dom::parser::parse_html;
use crate::dom::traits::TendrilSink;
use crate::dom::traits::*;

/// Algorithm:
///  1. Upon enter button of URL textbox, make request to URL (or local FS file)
///  2. Construct render tree with HTML received from response - https://developers.google.com/web/fundamentals/performance/critical-rendering-path/render-tree-construction?hl=en
///  3. Perform layout step using render tree.  Turn all the things into boxes!
///
/// Useful resources:
///     * https://developer.mozilla.org/en-US/docs/Learn/CSS/Introduction_to_CSS/Cascade_and_inheritance
///     * https://html.spec.whatwg.org/#introduction
///     * https://dom.spec.whatwg.org/#goals

fn main() {
    let application = Application::new(Some("com.kosmonaut.main"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Kosmonaut");
        window.set_default_size(800, 800);

        let url_entry_container = Box::new(Orientation::Vertical, 32);
        url_entry_container.add(&Entry::new());
        window.add(&url_entry_container);
        window.show_all();
    });

    let dom = parse_html()
        .from_utf8()
        .read_from(&mut File::open("web/basic.html").unwrap())
        .unwrap();

    dbg!(dom);
    style::parse_stylesheet_to_rules("web/browser.css");

    application.run(&[]);
}
