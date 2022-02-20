use crossterm::{
    cursor::MoveTo,
    cursor::{Hide, Show},
    event::KeyEvent,
    event::{poll, read, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::{
    io::{stderr, Write},
    time::{Duration, Instant},
};

mod creater;
mod datas;
mod field;
mod points;
mod view;

use creater::random_create;
use datas::Datas;
use field::Field;
use view::create_view;

fn wait_start<W>(write: &mut W, datas: &Datas) -> Result<bool>
where
    W: Write,
{
    queue!(
        write,
        MoveTo(1, datas.get_depth() as u16 + 2),
        Print("please k keys to start. esc key is end")
    )?;
    let mut ans = true;
    loop {
        poll(Duration::MAX)?;
        match read()? {
            Event::Key(KeyEvent { code: d, .. }) => match d {
                KeyCode::Esc => {
                    ans = false;
                    break;
                }
                KeyCode::Char('k') => break,
                _ => continue,
            },
            _ => continue,
        }
    }
    queue!(
        write,
        MoveTo(1, datas.get_depth() as u16 + 2),
        Print("                                          ")
    )?;
    Ok(ans)
}

fn update_terminal<W>(write: &mut W, datas: &Datas) -> Result<()>
where
    W: Write,
{
    for (i, line) in create_view(datas).into_iter().enumerate() {
        queue!(
            write,
            MoveTo(1, i as u16 + 1),
            Print(line.into_iter().collect::<String>())
        )?;
    }
    write.flush()?;
    Ok(())
}

fn app_loop<W>(write: &mut W, datas: &mut Datas) -> Result<()>
where
    W: Write,
{
    const DEFAULT_SPEED: Duration = Duration::from_millis(1_000);
    let mut last_down_time = Instant::now();
    let mut speed: Duration = DEFAULT_SPEED;
    loop {
        if !datas.exist_float() {
            speed = speed
                .checked_div(datas.get_deletable_lines().len() as u32 + 1)
                .unwrap_or(speed);
            datas.delete_lines().create_float(random_create());
        }
        update_terminal(write, datas)?;
        if datas.is_dead() {
            if !wait_start(write, datas)? {
                return Ok(());
            }
            datas.clear();
            last_down_time = Instant::now();
            speed = DEFAULT_SPEED;
            continue;
        };
        if !poll(speed.saturating_sub(last_down_time.elapsed()))? {
            datas.down_action();
            last_down_time = Instant::now();
            continue;
        };
        let k = if let Event::Key(KeyEvent { code: k, .. }) = read()? {
            k
        } else {
            continue;
        };
        match k {
            KeyCode::Esc => {
                return Ok(());
            }
            KeyCode::Char('j') => {
                datas.down_action();
                last_down_time = Instant::now();
            }
            KeyCode::Char('h') => {
                datas.left_action();
            }
            KeyCode::Char('l') => {
                datas.right_action();
            }
            KeyCode::Char('k') => {
                datas.do_routing();
            }
            _ => (),
        }
    }
}

fn main() -> Result<()> {
    let mut write = stderr();
    terminal::enable_raw_mode()?;
    execute!(write, Hide, EnterAlternateScreen)?;
    app_loop(&mut write, &mut Datas::new(Field::new(10, 10)))?;
    execute!(write, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
