use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::application::PixseeApplication;
use crate::config::{APP_ID, PROFILE};

mod imp {
    use super::*;

    use gtk::CompositeTemplate;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/com/blendermonkey/Pixsee/ui/window.ui")]
    pub struct PixseeApplicationWindow {
        #[template_child]
        pub headerbar: TemplateChild<gtk::HeaderBar>,
        pub settings: gio::Settings,
    }

    impl Default for PixseeApplicationWindow {
        fn default() -> Self {
            Self {
                headerbar: TemplateChild::default(),
                settings: gio::Settings::new(APP_ID),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PixseeApplicationWindow {
        const NAME: &'static str = "PixseeApplicationWindow";
        type Type = super::PixseeApplicationWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        // you must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PixseeApplicationWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            // devel profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            // load latest window state
            obj.load_window_size();
        }
    }

    impl WidgetImpl for PixseeApplicationWindow {}
    impl WindowImpl for PixseeApplicationWindow {
        // save window state on delete event
        fn close_request(&self, window: &Self::Type) -> gtk::Inhibit {
            if let Err(err) = window.save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }

            // pass close request on to the parent
            self.parent_close_request(window)
        }
    }

    impl ApplicationWindowImpl for PixseeApplicationWindow {}
}

glib::wrapper! {
    pub struct PixseeApplicationWindow(ObjectSubclass<imp::PixseeApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

impl PixseeApplicationWindow {
    pub fn new(app: &PixseeApplication) -> Self {
        glib::Object::new(&[("application", app)])
            .expect("Failed to create PixseeApplicationWindow")
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();

        let (width, height) = self.default_size();

        imp.settings.set_int("window-width", width)?;
        imp.settings.set_int("window-height", height)?;

        imp.settings
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let imp = self.imp();

        let width = imp.settings.int("window-width");
        let height = imp.settings.int("window-height");
        let is_maximized = imp.settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}
