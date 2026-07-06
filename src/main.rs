use std::io::{self, Write, stdout};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, MoveRight, MoveToColumn, MoveToNextLine},
    style::{self, Print, PrintStyledContent, Stylize},
    terminal,
};
use rustix::system::uname;

fn main() -> anyhow::Result<()> {
    let mut output = stdout();

    let dev_info = get_device_info()?;

    let (cols, t_col) = (48, 12);

    output.execute(terminal::Clear(terminal::ClearType::All))?;
    output.queue(cursor::MoveTo(0, 0))?;

    draw_header(&mut output, cols, t_col)?;
    draw_device_info(&mut output, t_col, cols, dev_info)?;
    draw_footer(&mut output, cols, t_col)?;

    output.flush()?;

    Ok(())
}

enum RowItem {
    KV(String, String),
    LineBreak,
}

fn get_device_info() -> io::Result<Box<[RowItem]>> {
    let mut res: Vec<RowItem> = Vec::new();

    let uname = uname();
    res.push(RowItem::KV(
        "Hostname".to_string(),
        uname.sysname().to_string_lossy().to_string(),
    ));

    Ok(res.into_boxed_slice())
}

fn draw_header(output: &mut dyn Write, width: u16, t_col: u16) -> io::Result<()> {
    output.queue(Print('┌'))?;
    for _i in 1..width - 1 {
        output.queue(Print('┬'))?;
    }
    output.queue(Print('┐'))?.queue(MoveToNextLine(1))?;

    output.queue(Print('├'))?;
    for _i in 1..width - 1 {
        output.queue(Print('┴'))?;
    }
    output.queue(Print('┤'))?.queue(MoveToNextLine(1))?;

    let header_title = "Machine Report";

    let center_loc: u16 = width / 2 - header_title.len() as u16 / 2;
    output
        .queue(Print('│'))?
        .queue(MoveToColumn(center_loc))?
        .queue(PrintStyledContent(header_title.bold()))?
        .queue(MoveToColumn(width - 1))?
        .queue(Print('│'))?
        .queue(MoveToNextLine(1))?;

    output.queue(Print('├'))?;
    for _i in 1..width - 1 {
        output.queue(Print('─'))?;
    }

    output
        .queue(Print('┤'))?
        .queue(MoveToColumn(t_col))?
        .queue(Print('┬'))?
        .queue(MoveToNextLine(1))?;

    Ok(())
}

fn draw_footer(output: &mut dyn Write, width: u16, t_col: u16) -> io::Result<()> {
    output.queue(Print('└'))?;
    for _i in 1..width - 1 {
        output.queue(Print('─'))?;
    }
    output.queue(Print('┘'))?;

    output.queue(MoveToColumn(t_col))?.queue(Print('┴'))?;

    Ok(())
}

fn draw_device_info(
    output: &mut dyn Write,
    t_col: u16,
    width: u16,
    device_info: Box<[RowItem]>,
) -> io::Result<()> {
    for row in device_info {
        match row {
            RowItem::KV(k, v) => {
                output
                    .queue(Print('│'))?
                    .queue(MoveRight(1))?
                    .queue(Print(k))?
                    .queue(MoveToColumn(t_col))?
                    .queue(Print('│'))?
                    .queue(MoveRight(1))?
                    .queue(PrintStyledContent(v.italic()))?
                    .queue(MoveToColumn(width - 1))?
                    .queue(Print('│'))?
                    .queue(MoveToNextLine(1))?;
            }
            RowItem::LineBreak => {
                output.queue(Print('├'))?;

                for _i in 1..width - 1 {
                    output.queue(Print('─'))?;
                }

                output.queue(Print('┤'))?;

                output
                    .queue(MoveToColumn(t_col))?
                    .queue(Print('┼'))?
                    .queue(MoveToNextLine(1))?;
            }
        }
    }

    Ok(())
}
