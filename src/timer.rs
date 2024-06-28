use gtk::glib::{
    self, clone, closure_local,
    prelude::*,
    subclass::{prelude::*, Signal},
};

use std::{
    cell::Cell,
    sync::OnceLock,
    time::{Duration, Instant},
};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Timer {
        pub running: Cell<bool>,
        pub instant: Cell<Option<Instant>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Timer {
        const NAME: &'static str = "timer";
        type Type = super::Timer;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for Timer {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![Signal::builder("stopwatch-update")
                    .param_types([u32::static_type(), u32::static_type()])
                    .build()]
            })
        }
    }
}

glib::wrapper! {
    pub struct Timer(ObjectSubclass<imp::Timer>);
}

impl Timer {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn connect_stopwatch_update<F: Fn(&Self, u32, u32) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_closure(
            "stopwatch-update",
            true,
            closure_local!(|ref timer, minutes, seconds| {
                f(timer, minutes, seconds);
            }),
        )
    }

    pub fn start(&self) {
        let imp = self.imp();

        imp.running.set(true);
        imp.instant.set(Some(Instant::now()));

        glib::timeout_add_local(
            std::time::Duration::from_millis(100),
            clone!(@weak self as timer => @default-return glib::ControlFlow::Break, move || {
                let imp = timer.imp();

                if !imp.running.get() {
                    return glib::ControlFlow::Break;
                }

                let instant = imp
                    .instant
                    .get()
                    .expect("timer is running, but no instant is set");
                let (hours, minutes) = instant.elapsed().as_hours_and_minutes();
                timer.emit_by_name::<()>("stopwatch-update", &[&hours, &minutes]);

                glib::ControlFlow::Continue
            }),
        );
    }

    pub fn pause(&self) {
        let imp = self.imp();
        imp.running.set(false);
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer::new()
    }
}

trait DurationExt {
    fn as_hours_and_minutes(&self) -> (u32, u32);
}

impl DurationExt for Duration {
    fn as_hours_and_minutes(&self) -> (u32, u32) {
        let seconds = self.as_secs();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;

        (hours.try_into().unwrap(), minutes.try_into().unwrap())
    }
}
