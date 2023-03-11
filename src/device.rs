use device_query::{MousePosition, DeviceState, DeviceEvents};

struct MouseHandler {
    history: Vec<Point>,
}

impl MouseHandler
{
    fn new() -> Self {
        MouseHandler { history: Vec::new() }
    }

    pub fn start(&self) {
        let ds = DeviceState::new();
        let h = self.history.clone();
        ds.on_mouse_move(move |p| {
            self.history.push(Point::adapt(p));
        });
    }
}

struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
       return Point { x, y };
    }

    fn adapt(pos: &MousePosition) -> Point {
        let (x, y) = *pos;
        return Point::new(x, y);
    }
}
