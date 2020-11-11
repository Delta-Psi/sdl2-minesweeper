use crate::field::{Field, RevealResult};
use std::time::{Duration, Instant};

const FIELD_WIDTH: u8 = 8;
const FIELD_HEIGHT: u8 = 8;
const MINE_COUNT: u16 = 10;

#[derive(Debug)]
enum Timer {
    NotStarted,
    Started(Instant),
    Stopped(Duration),
}

#[derive(Debug)]
pub struct State {
    field: Field,
    timer: Timer,
}

impl State {
    pub fn new() -> Self {
        Self {
            field: Field::new(FIELD_WIDTH, FIELD_HEIGHT, MINE_COUNT),
            timer: Timer::NotStarted,
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    fn start_timer(&mut self) {
        self.timer = Timer::Started(Instant::now());
    }

    fn stop_timer(&mut self) {
        self.timer = Timer::Stopped(self.timer());
    }

    pub fn timer(&self) -> Duration {
        match self.timer {
            Timer::NotStarted => Duration::new(0, 0),
            Timer::Started(i) => i.elapsed(),
            Timer::Stopped(d) => d,
        }
    }

    pub fn game_over(&self) -> bool {
        match self.timer {
            Timer::Stopped(_) => true,
            _ => false,
        }
    }

    pub fn reveal(&mut self, x: u8, y: u8) -> RevealResult {
        if let Timer::NotStarted = self.timer {
            self.start_timer();
        }

        let result = self.field.reveal(x, y);
        match result {
            RevealResult::Mine => self.stop_timer(),
            _ => (),
        }

        result
    }

    pub fn toggle_flag(&mut self, x: u8, y: u8) {
        self.field.toggle_flag(x, y);
    }

    pub fn mines_remaining(&self) -> i32 {
        self.field.mine_count() as i32 - self.field.flagged_cells() as i32
    }
}
