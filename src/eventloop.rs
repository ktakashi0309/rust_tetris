use anyhow::Result;
use std::time::{Duration, Instant};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait View {
    fn update(&mut self, appdata: &Appmodel2) -> Result<()>;
}
#[cfg_attr(test, automock)]
pub trait Input {
    fn read_dead(&self) -> Result<DeadInput>;
    fn read_play(&self, timeout: Duration) -> Result<PlayInput>;
}

#[cfg_attr(test, automock)]
pub trait Creater {
    fn create(&self) -> Points;
}

use crate::points::Points;

use super::datas::Datas;

pub struct Appmodel2 {
    pub datas: Datas,
    pub mode: Mode,
    pub speedlevel: Speedlevel,
}

pub struct Speedlevel(pub u8);

impl Speedlevel {
    pub fn get_duration(&self) -> Duration {
        match self {
            Self(1) => Duration::new(2, 0),
            _ => Duration::new(0, 1),
        }
    }
}

pub enum Mode {
    Dead,
    Play,
    End,
}
pub enum DeadInput {
    Restart,
    End,
}

pub enum PlayInput {
    Down,
    Left,
    Right,
    Routation,
    End,
    Timeout,
}

pub fn app_loop<V, I, C>(
    view: &mut V,
    input: &mut I,
    appmodel: &mut Appmodel2,
    creater: &C,
) -> Result<()>
where
    V: View,
    I: Input,
    C: Creater,
{
    loop {
        match appmodel.mode {
            Mode::Dead => {
                dead_loop(view, input, appmodel)?;
            }
            Mode::Play => {
                play_loop(view, input, appmodel, creater)?;
            }
            Mode::End => return Ok(()),
        }
    }
}

fn play_loop<V, I, C>(
    view: &mut V,
    input: &mut I,
    appmodel: &mut Appmodel2,
    creater: &C,
) -> Result<()>
where
    V: View,
    I: Input,
    C: Creater,
{
    let mut last_down = Instant::now();
    if !appmodel.datas.exist_float() {
        appmodel.datas.create_float(creater.create());
    }
    view.update(appmodel)?;
    loop {
        match input.read_play(
            appmodel
                .speedlevel
                .get_duration()
                .checked_sub(last_down.elapsed())
                .unwrap_or(Duration::ZERO),
        )? {
            PlayInput::Down | PlayInput::Timeout => {
                appmodel.datas.down_action();
                if !appmodel.datas.exist_float() {
                    appmodel.datas.delete_lines();
                    appmodel.datas.create_float(creater.create());
                }
                if appmodel.datas.is_dead() {
                    appmodel.mode = Mode::Dead;
                    return Ok(());
                }
                last_down = Instant::now();
            }
            PlayInput::End => {
                appmodel.mode = Mode::End;
                return Ok(());
            }
            PlayInput::Left => {
                appmodel.datas.left_action();
            }
            PlayInput::Right => {
                appmodel.datas.right_action();
            }
            PlayInput::Routation => {
                appmodel.datas.do_routing();
            }
        }
        view.update(appmodel)?;
    }
}

fn dead_loop<V, I>(view: &mut V, input: &mut I, appmodel: &mut Appmodel2) -> Result<()>
where
    V: View,
    I: Input,
{
    loop {
        match input.read_dead()? {
            DeadInput::Restart => {
                appmodel.mode = Mode::Play;
                appmodel.datas.clear();
                view.update(appmodel)?;
                return Ok(());
            }
            DeadInput::End => {
                appmodel.mode = Mode::End;
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::creater::RandomCreater;

    use super::super::field::Field;
    use super::*;

    #[test]
    fn pause_escape() {
        let mut view = MockView::new();
        let mut input = MockInput::new();
        let mut creater = MockCreater::new();
        let mut count = 0;
        input.expect_read_play().times(10).returning(move |_| {
            count += 1;
            if count < 5 {
                return Ok(PlayInput::Timeout);
            } else if count < 10 {
                return Ok(PlayInput::Down);
            } else {
                return Ok(PlayInput::End);
            }
        });
        input.expect_read_dead().returning(|| {
            return Ok(DeadInput::End);
        });
        view.expect_update().returning(|_| Ok(()));
        creater
            .expect_create()
            .returning(|| RandomCreater::random_create());
        app_loop(
            &mut view,
            &mut input,
            &mut Appmodel2 {
                datas: super::Datas::new(Field::new(3, 3)),
                mode: Mode::Play,
                speedlevel: Speedlevel(1),
            },
            &creater,
        )
        .unwrap();
    }
    #[test]
    fn restart_escape() {
        let mut view = MockView::new();
        let mut input = MockInput::new();
        let mut creater = MockCreater::new();
        input.expect_read_play().times(1).returning(|_| {
            return Ok(PlayInput::End);
        });
        input.expect_read_dead().times(1).returning(|| {
            return Ok(DeadInput::Restart);
        });
        view.expect_update().returning(|_| Ok(()));
        creater
            .expect_create()
            .returning(|| RandomCreater::random_create());
        app_loop(
            &mut view,
            &mut input,
            &mut Appmodel2 {
                datas: super::Datas::new(Field::new(3, 3)),
                mode: Mode::Dead,
                speedlevel: Speedlevel(1),
            },
            &creater,
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "input error")]
    fn input_error() {
        let mut view = MockView::new();
        let mut input = MockInput::new();
        let mut creater = MockCreater::new();
        input
            .expect_read_dead()
            .times(1)
            .returning(|| anyhow::bail!("input error"));
        view.expect_update().returning(|_| Ok(()));
        creater
            .expect_create()
            .returning(|| RandomCreater::random_create());
        app_loop(
            &mut view,
            &mut input,
            &mut Appmodel2 {
                datas: super::Datas::new(Field::new(3, 3)),
                mode: Mode::Dead,
                speedlevel: Speedlevel(1),
            },
            &creater,
        )
        .unwrap();
    }
}
