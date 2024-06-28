mod timer;
mod window;

use gtk::{gdk::Display, gio, glib, prelude::*, Application, CssProvider};

const APP_ID: &str = "org.chompa.thyme";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("thyme.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_resource("org/chompa/thyme/style.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = window::Window::new(app);
    // Present window
    window.present();
}
