use crate::timer::Timer;

use gtk::{
    gdk, gio,
    glib::{
        self, clone,
        subclass::{prelude::*, InitializingObject},
        Object,
    },
    prelude::*,
    subclass::prelude::*,
    Application, ApplicationWindow, CompositeTemplate, EventControllerKey, TemplateChild,
};

mod imp {

    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/chompa/thyme/window.ui")]
    pub struct Window {
        pub timer: Timer,

        #[template_child]
        pub timer_label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "window";
        type Type = super::Window;
        type ParentType = ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        let window = Object::builder::<Self>()
            .property("application", app)
            .build();

        window.init();

        window
    }

    fn init(&self) {
        let imp = self.imp();

        imp.timer.start();

        // Setup keybinds
        let event_controller = EventControllerKey::new();

        event_controller.connect_key_pressed(clone!(@weak self as window =>
            @default-return glib::Propagation::Proceed,
            move |_, key, _, _| {
                match key {
                    gdk::Key::F1 => {
                        window.start_stopwatch();
                    }
                    gdk::Key::F2 => {
                        window.pause_stopwatch();
                    }
                    _ => (),
                }
                glib::Propagation::Proceed
            }
        ));

        self.add_controller(event_controller);

        // Timer/label updates
        let timer_label = &*imp.timer_label;

        timer_label.set_direction(gtk::TextDirection::Ltr);
        timer_label.set_label("00:00");

        imp.timer.connect_stopwatch_update(
            clone!(@weak self as window => move |_, hours, minutes| {
                window.update_stopwatch(hours, minutes);
            }),
        );
    }

    fn start_stopwatch(&self) {
        self.imp().timer.start();
    }

    fn pause_stopwatch(&self) {
        self.imp().timer.pause();
    }

    fn update_stopwatch(&self, minutes: u32, hours: u32) -> glib::ControlFlow {
        let imp = self.imp();
        let label = &*imp.timer_label;
        label.set_label(&format!("{minutes:>02}∶{hours:>02}"));
        glib::ControlFlow::Continue
    }
}
