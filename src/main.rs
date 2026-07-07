use std::io::stdout;

use crate::{device_info::get_device_info, draw::draw_report};

mod device_info;
mod draw;

enum RowItem {
    KV(&'static str, String),
    LineBreak,
}

fn main() -> anyhow::Result<()> {
    let output = stdout();

    let dev_info = get_device_info()?;

    draw_report(output, dev_info)?;

    Ok(())
}
