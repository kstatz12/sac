use device_query::MousePosition;

struct MouseHistory {
    history: Vec<Box<Point>>,
}

struct MouseHandler {
    1
}

impl MouseHandler
{
    fn new() -> Self {

    }
}

struct Point {
    x: u32,
    y: u32
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
       return Point { x, y };
    }

    fn adapt(pos: MousePosition) -> Box<Point> {
        let (x, y) = pos;
        return Box<Point>::new(Point::new(x, y));
    }
}

impl MouseHistory {
    async fn save(&mut self, pos: MousePosition) {
        self.history.push(Point::adapt(pos));
    }
}
