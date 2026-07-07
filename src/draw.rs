use chrono::Local;

use crossterm::{
    QueueableCommand,
    cursor::{self, MoveRight, MoveToColumn, MoveToNextLine, MoveUp},
    style::{Print, StyledContent, Stylize},
    terminal::{self, ScrollUp},
};

use iana_time_zone::get_timezone;

use std::{
    fmt::Display,
    io::{self, Write},
};

use super::RowItem;

pub fn draw_report(
    mut output: std::io::Stdout,
    dev_info: Box<[RowItem]>,
) -> Result<(), anyhow::Error> {
    let (cols, t_col) = get_width(&dev_info);

    let lines_needed: u16 = dev_info.len() as u16 + 7; // 2x content top / bottom divider, 4x header, 1x newline
    let remaining_lines: u16 = terminal::size()?.1 - cursor::position()?.1;

    if remaining_lines < lines_needed {
        output
            .queue(ScrollUp(lines_needed - remaining_lines))?
            .queue(MoveUp(lines_needed - remaining_lines))?;
    }

    output.queue(MoveToColumn(0))?;
    draw_header(&mut output, cols, t_col)?;
    draw_device_info(&mut output, t_col, cols, dev_info)?;
    draw_footer(&mut output, cols, t_col)?;
    output.flush()?;
    Ok(())
}

fn get_width(device_info: &[RowItem]) -> (u16, u16) {
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

fn round_up(i: usize) -> usize {
    (i / 4 + 2) * 4
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

    print_centered(output, width, header_title.bold())?;
    let now = Local::now();
    let timezone = get_timezone();
    let time_string = if let Ok(zone) = timezone {
        format!("{} ({})", now.format("%c"), zone)
    } else {
        now.format("%c").to_string()
    };

    print_centered(output, width, time_string.italic())?;

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

fn print_centered<D: Display>(
    output: &mut dyn Write,
    width: u16,
    header_title: StyledContent<D>,
) -> Result<(), io::Error> {
    let center_loc: u16 = width / 2 - header_title.content().to_string().len() as u16 / 2;

    output
        .queue(Print('│'))?
        .queue(MoveToColumn(center_loc))?
        .queue(Print(header_title))?
        .queue(MoveToColumn(width - 1))?
        .queue(Print('│'))?
        .queue(MoveToNextLine(1))?;
    Ok(())
}

fn draw_footer(output: &mut dyn Write, width: u16, t_col: u16) -> io::Result<()> {
    output.queue(Print('└'))?;
    for _i in 1..width - 1 {
        output.queue(Print('─'))?;
    }
    output.queue(Print('┘'))?;

    output
        .queue(MoveToColumn(t_col))?
        .queue(Print('┴'))?
        .queue(MoveToNextLine(1))?;

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
