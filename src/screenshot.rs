use screenshots::{Screen, Image};
use crate::tick::Job;


pub struct ScreenShot {
    screens: Vec<Screen>,
    images: Vec<Image>
}


impl ScreenShot {
    pub fn new() -> Self {
        ScreenShot { screens: Screen::all().unwrap(), images: Vec::new() }
    }
}

impl Job for ScreenShot {
   fn exec(&mut self) {
       for s in &self.screens {
           let image = s.capture().unwrap();
           self.images.push(image);
       }
   }
}
