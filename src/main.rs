use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk4 as gtk;
const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let horisontal_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .build();
    let horizontal_bar = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .build();

    let vertical_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    let entry = gtk4::Entry::builder()
        .margin_bottom(4)
        .margin_start(4)
        .margin_end(4)
        .margin_top(4)
        .build();

    let entry_ole = gtk4::Entry::builder()
        .margin_bottom(4)
        .margin_start(4)
        .margin_end(4)
        .margin_top(4)
        .build();

    let button = gtk4::Button::builder()
        .label("Berechnen")
        .margin_start(4)
        .margin_end(4)
        .margin_top(4)
        .margin_bottom(4)
        .build();

    let combo_box = gtk4::ComboBoxText::builder()
        .margin_start(4)
        .margin_end(4)
        .margin_top(4)
        .margin_bottom(4)
        .build();

    combo_box.append(Some("0"), "1 : 25");
    combo_box.append(Some("1"), "1 : 30");
    combo_box.append(Some("2"), "1 : 50");

    combo_box.set_active(Some(2));
    entry.set_text("5");
    horisontal_box.append(&entry);
    horisontal_box.append(&button);
    horizontal_bar.append(&entry_ole);
    horizontal_bar.append(&combo_box);
    vertical_box.append(&horisontal_box);
    vertical_box.append(&horizontal_bar);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&vertical_box)
        .build();

    button.connect_clicked(move |_| {
        let mut mix = 0.0;
        if combo_box.active() == Some(0) {
            mix = 0.025;
        } else if combo_box.active() == Some(1) {
            mix = 0.030;
        } else if combo_box.active() == Some(2) {
            mix = 0.050;
        }
        let mut benzin: f64 = entry.text().parse().unwrap();
        benzin = benzin / mix;
        let s = format!("Mix benzin ({})", benzin);
        entry_ole.set_text(&s);
    });
    // Present window
    window.present();
}
