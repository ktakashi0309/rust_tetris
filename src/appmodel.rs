use super::actions::Actions;
use super::datas::Datas;
use super::points::Points;
use crossterm::event::KeyEvent;
use crossterm::event::{Event, KeyCode};
use std::time::{Duration, Instant};

pub struct Appmodel {
    pub datas: Datas,
    pub last_down_time: Instant,
    pub next_action: Actions,
    pub speed: Duration,
    pub message: String,
}

use super::field::Field;

impl Appmodel {
    pub fn new(last_down_time: Instant) -> Self {
        Appmodel {
            speed: Duration::from_millis(1_000),
            datas: Datas::new(Field::new(10, 10)),
            last_down_time,
            next_action: Actions::Create,
            message: "".to_string(),
        }
    }
    pub fn down(&mut self, now: Instant) -> &mut Self {
        self.last_down_time = now;
        self.datas.down_action();
        if !self.datas.exist_float() {
            self.datas.delete_lines();
            self.speed = self
                .speed
                .checked_div(self.datas.get_deletable_lines().len() as u32 + 1)
                .unwrap_or(self.speed);
            self.next_action = Actions::Create;
        } else {
            self.next_action = Actions::Update;
        }
        self
    }
    pub fn create(&mut self, first: Points) -> &mut Self {
        self.datas.create_float(first);
        self.next_action = Actions::Update;
        self
    }
    pub fn setaction(&mut self, action: Actions) -> &mut Self {
        self.next_action = action;
        self
    }
    pub fn dead_of_alive(&mut self) -> &mut Self {
        if self.datas.is_dead() {
            self.setaction(Actions::WaitStart);
            self.message = "please k keys to start. esc key is end".to_string();
        } else {
            self.setaction(Actions::Poll);
            self.message = "                                      ".to_string();
        }
        self
    }
    pub fn next_timeout(&self, elasp: Duration) -> Duration {
        self.speed.saturating_sub(elasp)
    }
    pub fn event_action(&mut self, event: Event) -> &mut Self {
        if let Event::Key(KeyEvent { code: k, .. }) = event {
            match k {
                KeyCode::Esc => {
                    self.next_action = Actions::Escape;
                }
                KeyCode::Char('j') => {
                    self.next_action = Actions::DownAction;
                }
                KeyCode::Char('h') => {
                    self.datas.left_action();
                    self.next_action = Actions::Update;
                }
                KeyCode::Char('l') => {
                    self.datas.right_action();
                    self.next_action = Actions::Update;
                }
                KeyCode::Char('k') => {
                    self.datas.do_routing();
                    self.next_action = Actions::Update;
                }
                _ => {
                    self.next_action = Actions::Poll;
                }
            }
        } else {
            self.next_action = Actions::Poll;
        }
        self
    }
    pub fn init(&mut self, start: Instant) -> &mut Self {
        self.datas.clear();
        self.last_down_time = start;
        self.next_action = Actions::Create;
        self
    }
    pub fn dead_event_action(&mut self, event: Event) -> &mut Self {
        if let Event::Key(KeyEvent { code: k, .. }) = event {
            match k {
                KeyCode::Esc => self.setaction(Actions::Escape),
                KeyCode::Char('k') => self.setaction(Actions::Init),
                _ => self,
            }
        } else {
            self
        }
    }
}
