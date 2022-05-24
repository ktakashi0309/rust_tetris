use crate::eventloop::Input;
use anyhow::Result;
use crossterm::event::KeyEvent;
use crossterm::event::{poll, read};
use crossterm::event::{Event, KeyCode};
use std::time::{Duration, Instant};

pub struct CliInput {}

impl Input for CliInput {
    fn read_dead(&self) -> Result<crate::eventloop::DeadInput> {
        loop {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => return Ok(crate::eventloop::DeadInput::End),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('k'),
                    ..
                }) => return Ok(crate::eventloop::DeadInput::Restart),
                _ => continue,
            }
        }
    }
    fn read_play(&self, timeout: std::time::Duration) -> Result<crate::eventloop::PlayInput> {
        let now = Instant::now();
        loop {
            if !poll(timeout.checked_sub(now.elapsed()).unwrap_or(Duration::ZERO))? {
                return Ok(crate::eventloop::PlayInput::Timeout);
            }
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => return Ok(crate::eventloop::PlayInput::End),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    ..
                }) => return Ok(crate::eventloop::PlayInput::Down),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    ..
                }) => return Ok(crate::eventloop::PlayInput::Right),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('h'),
                    ..
                }) => return Ok(crate::eventloop::PlayInput::Left),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('k'),
                    ..
                }) => return Ok(crate::eventloop::PlayInput::Routation),
                _ => continue,
            }
        }
    }
}
