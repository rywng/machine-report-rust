use chrono::TimeDelta;
use crossterm::QueueableCommand;
use crossterm::cursor;
use crossterm::cursor::MoveRight;

use crossterm::cursor::MoveToColumn;
use crossterm::cursor::MoveToNextLine;
use crossterm::style::PrintStyledContent;

use crossterm::style::Print;
use crossterm::style::Stylize;
use crossterm::terminal;

use std::io;
use std::io::Write;

use crate::draw;

use super::RowItem;

pub fn draw_report(
    mut output: std::io::Stdout,
    dev_info: Box<[RowItem]>,
) -> Result<(), anyhow::Error> {
    let (cols, t_col) = get_width(&dev_info);
    output.queue(terminal::Clear(terminal::ClearType::All))?;
    output.queue(cursor::MoveTo(0, 0))?;
    draw::draw_header(&mut output, cols, t_col)?;
    draw::draw_device_info(&mut output, t_col, cols, dev_info)?;
    draw::draw_footer(&mut output, cols, t_col)?;
    output.flush()?;
    Ok(())
}

pub(crate) fn get_width(device_info: &[RowItem]) -> (u16, u16) {
    let t_col = device_info
        .iter()
        .map(|i| match i {
            RowItem::KV(k, _) => round_up(k.len()),
            RowItem::LineBreak => 0,
        })
        .max()
        .unwrap_or_default();

    let width = device_info
        .iter()
        .map(|i| match i {
            RowItem::KV(_, v) => t_col + round_up(v.len()),
            RowItem::LineBreak => 0,
        })
        .max()
        .unwrap_or_default();

    (width as u16, t_col as u16)
}

pub(crate) fn round_up(i: usize) -> usize {
    (i / 4 + 2) * 4
}

pub(crate) fn format_loads(loads: &[u64; 3]) -> String {
    if cfg!(target_os = "linux") {
        let loads_linux = [
            loads[0] as f64 / 65536.,
            loads[1] as f64 / 65536.,
            loads[2] as f64 / 65536.,
        ];

        format!(
            "{:.2}, {:.2}, {:.2}",
            loads_linux[0], loads_linux[1], loads_linux[2]
        )
    } else {
        format!("{}, {}, {}", loads[0], loads[1], loads[2])
    }
}

pub(crate) fn format_duration(input: TimeDelta) -> String {
    let day_part = if input.num_days() > 0 {
        format!("{} days, ", input.num_days())
    } else if input.num_days() == 1 {
        "1 day, ".to_string()
    } else {
        "".to_string()
    };

    let time_part = format!("{}:{:02}", input.num_hours() % 24, input.num_minutes() % 60);

    format!("{}{}", day_part, time_part)
}

pub(crate) fn draw_header(output: &mut dyn Write, width: u16, t_col: u16) -> io::Result<()> {
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

pub(crate) fn draw_footer(output: &mut dyn Write, width: u16, t_col: u16) -> io::Result<()> {
    output.queue(Print('└'))?;
    for _i in 1..width - 1 {
        output.queue(Print('─'))?;
    }
    output.queue(Print('┘'))?;

    output.queue(MoveToColumn(t_col))?.queue(Print('┴'))?;

    Ok(())
}

pub(crate) fn draw_device_info(
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
                    .queue(Print(v))?
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
