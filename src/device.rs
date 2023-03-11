use device_query::MousePosition;

struct MouseHandler {
    history: Vec<Box<Point>>,
}

impl MouseHandler
{
    fn new() -> Self {
        MouseHandler { history: Vec::new() }
    }

    pub fn start(&mut self) {

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

    fn adapt(pos: MousePosition) -> Box<Point> {
        let (x, y) = pos;
        return Box::<Point>::new(Point::new(x, y));
    }
}
