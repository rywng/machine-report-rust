use std::io::stdout;

use crate::draw::draw_report;

mod device_info;
mod draw;

enum RowItem {
    KV(&'static str, String),
    LineBreak,
}

fn main() -> anyhow::Result<()> {
    let output = stdout();

    let dev_info = device_info::get_device_info()?;

    draw_report(output, dev_info)?;

    Ok(())
}
