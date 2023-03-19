use screenshots::{Screen, Image};

use crate::common::{Job, Startable, Stoppable};

pub struct ScreenShotService {
    screens: Vec<Screen>,
    images: Vec<Image>
}

impl ScreenShotService {
    pub fn new() -> Self {
        ScreenShotService { screens: Screen::all().expect("Could Not Initialize Screens"), images: Vec::new() }
    }

    fn capture(&mut self) {
        for s in &self.screens {
           let image = s.capture().expect("Could Not Capture Screen");
           self.images.push(image);
       }
    }
}

impl Startable for ScreenShotService {
    fn start(&mut self) {
        self.capture();
    }
}

impl Stoppable for ScreenShotService {
    fn stop(&mut self, on_stop: fn(c: &mut Self)) {
        on_stop(self);
    }
}
