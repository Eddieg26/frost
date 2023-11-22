use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

use crate::ecs::Resource;

pub struct Timer {
    start: Instant,
    last: Instant,
    delta: Duration,
    fixed_delta: Duration,
}

impl Timer {
    pub(super) fn new(fixed_delta: Duration) -> Self {
        Self {
            start: Instant::now(),
            last: Instant::now(),
            delta: Duration::from_secs(0),
            fixed_delta,
        }
    }

    pub(super) fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last;
        self.last = now;
    }

    pub fn start(&self) -> Instant {
        self.start
    }

    pub fn last(&self) -> Instant {
        self.last
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn fixed_delta(&self) -> Duration {
        self.fixed_delta
    }
}

pub struct GameTime {
    timer: Rc<RefCell<Timer>>,
}

impl GameTime {
    pub(super) fn new(timer: Rc<RefCell<Timer>>) -> Self {
        Self { timer }
    }

    pub fn start(&self) -> Instant {
        self.timer.borrow().start
    }

    pub fn last(&self) -> Instant {
        self.timer.borrow().last
    }

    pub fn delta(&self) -> Duration {
        self.timer.borrow().delta
    }

    pub fn fixed_delta(&self) -> Duration {
        self.timer.borrow().fixed_delta
    }

    pub fn since_start(&self) -> Duration {
        Instant::now() - self.timer.borrow().start
    }
}

impl Resource for GameTime {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
