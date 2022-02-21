use gettextrs::gettext;
use log::{debug, info};

use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};

use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::window::PixseeApplicationWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct PixseeApplication {
        pub window: OnceCell<WeakRef<PixseeApplicationWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PixseeApplication {
        const NAME: &'static str = "PixseeApplication";
        type Type = super::PixseeApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for PixseeApplication {}

    impl ApplicationImpl for PixseeApplication {
        fn activate(&self, app: &Self::Type) {
            debug!("GtkApplication<PixseeApplication>::activate");
            self.parent_activate(app);

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = PixseeApplicationWindow::new(app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("GtkApplication<PixseeApplication>::startup");
            self.parent_startup(app);

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for PixseeApplication {}
}

glib::wrapper! {
    pub struct PixseeApplication(ObjectSubclass<imp::PixseeApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl PixseeApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(APP_ID)),
            ("flags", &gio::ApplicationFlags::empty()),
            ("resource-base-path", &Some("/com/blendermonkey/Pixsee/")),
        ])
        .expect("Application initialization failed...")
    }

    fn main_window(&self) -> PixseeApplicationWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak self as app => move |_, _| {
            // This is needed to trigger the delete event and saving the window state
            app.main_window().close();
            app.quit();
        }));
        self.add_action(&action_quit);

        // About
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about_dialog();
        }));
        self.add_action(&action_about);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/com/blendermonkey/Pixsee/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn show_about_dialog(&self) {
        let dialog = gtk::AboutDialog::builder()
            .logo_icon_name(APP_ID)
            .license_type(gtk::License::Gpl30)
            // Insert your website here
            // .website("https://gitlab.gnome.org/bilelmoussaoui/pixsee/")
            .version(VERSION)
            .transient_for(&self.main_window())
            .translator_credits(&gettext("translator-credits"))
            .modal(true)
            .authors(vec!["Cassaundra Smith".into(), "Kitt Zwovic".into()])
            .artists(vec!["Cassaundra Smith".into(), "Kitt Zwovic".into()])
            .build();

        dialog.present();
    }

    pub fn run(&self) {
        info!("Pixsee ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self);
    }
}
