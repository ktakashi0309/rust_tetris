use std::io::Write;

use crate::eventloop::View;
use ::anyhow::Result;

use crossterm::{
    cursor::MoveTo,
    cursor::{Hide, Show},
    execute, queue,
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

pub(crate) struct CliView {
    stderr: std::io::Stderr,
}

impl CliView {
    pub fn init() -> Result<Self> {
        let mut value = Self {
            stderr: std::io::stderr(),
        };
        terminal::enable_raw_mode()?;
        execute!(value.stderr, Hide, EnterAlternateScreen)?;
        Ok(value)
    }
    pub fn finalaize(&mut self) {
        execute!(self.stderr, Show, LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

impl View for CliView {
    fn update(&mut self, appdata: &crate::eventloop::Appmodel2) -> Result<()> {
        let width = appdata.datas.get_width();
        let depth = appdata.datas.get_depth();
        let mut ans = vec![vec!['　'; width + 2]; depth + 2];
        appdata
            .datas
            .get_frame()
            .iter()
            .for_each(|x| ans[x.0][x.1] = '🔲');
        appdata
            .datas
            .get_float()
            .iter()
            .for_each(|x| ans[x.0][x.1] = '🟥');
        appdata
            .datas
            .get_fixed()
            .iter()
            .for_each(|x| ans[x.0][x.1] = '🟦');
        for (i, line) in ans.into_iter().enumerate() {
            queue!(
                self.stderr,
                MoveTo(1, i as u16 + 1),
                Print(line.into_iter().collect::<String>())
            )?;
        }
        self.stderr.flush()?;
        Ok(())
    }
}
