use ::anyhow::Result;
mod cli_input;
mod cli_view;
mod creater;
mod datas;
mod eventloop;
mod field;
mod points;

use eventloop::app_loop;

use cli_input::CliInput;
use cli_view::CliView;
use creater::RandomCreater;
use datas::Datas;
use eventloop::Appmodel2;
use eventloop::Mode;
use eventloop::Speedlevel;
use field::Field;

fn main() -> Result<()> {
    let mut view = CliView::init()?;
    let mut input = CliInput {};
    let mut appmodel = Appmodel2 {
        datas: Datas::new(Field::new(20, 20)),
        mode: Mode::Play,
        speedlevel: Speedlevel(1),
    };
    let creater = RandomCreater {};
    app_loop(&mut view, &mut input, &mut appmodel, &creater)?;
    view.finalaize();
    Ok(())
}
