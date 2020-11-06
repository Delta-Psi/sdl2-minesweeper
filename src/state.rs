use crate::field::{Field, RevealResult};
use std::time::{Instant, Duration};

const FIELD_WIDTH: u8 = 8;
const FIELD_HEIGHT: u8 = 8;
const MINE_COUNT: u16 = 10;

#[derive(Debug)]
pub struct State {
    field: Field,
    timer_started: Option<Instant>,
}

impl State {
    pub fn new() -> Self {
        Self {
            field: Field::new(FIELD_WIDTH, FIELD_HEIGHT, MINE_COUNT),
            timer_started: None,
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    fn start_timer(&mut self) {
        self.timer_started = Some(Instant::now());
    }

    pub fn timer(&self) -> Duration {
        match self.timer_started {
            Some(i) => i.elapsed(),
            None => Duration::new(0, 0),
        }
    }

    pub fn reveal(&mut self, x: u8, y: u8) -> RevealResult {
        if self.timer_started.is_none() {
            self.start_timer();
        }

        self.field.reveal(x, y)
    }

    pub fn toggle_flag(&mut self, x: u8, y: u8) {
        self.field.toggle_flag(x, y);
    }

    pub fn mines_remaining(&self) -> i32 {
        self.field.mine_count() as i32 - self.field.flagged_cells() as i32
    }
}
