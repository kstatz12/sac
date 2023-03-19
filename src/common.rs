pub trait Startable {
    fn start(&mut self);
}

pub trait Stoppable {
    fn stop(&mut self, on_stop: fn(c: &mut Self));
}

pub trait Ping {
    fn ping(&mut self);
}
