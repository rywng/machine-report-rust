use std::io::{self, Write, stdout};

use chrono::{NaiveDateTime, TimeDelta};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, MoveRight, MoveToColumn, MoveToNextLine},
    style::{self, Print, PrintStyledContent, Stylize},
    terminal,
};
use rustix::system::{sysinfo, uname};

fn main() -> anyhow::Result<()> {
    let mut output = stdout();

    let dev_info = get_device_info()?;

    let (cols, t_col) = get_width(&dev_info);

    output.execute(terminal::Clear(terminal::ClearType::All))?;
    output.queue(cursor::MoveTo(0, 0))?;

    draw_header(&mut output, cols, t_col)?;
    draw_device_info(&mut output, t_col, cols, dev_info)?;
    draw_footer(&mut output, cols, t_col)?;

    output.flush()?;

    Ok(())
}

enum RowItem {
    KV(&'static str, String),
    LineBreak,
}

fn get_width(device_info: &Box<[RowItem]>) -> (u16, u16) {
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

fn get_device_info() -> io::Result<Box<[RowItem]>> {
    let mut res: Vec<RowItem> = Vec::new();

    append_uname(&mut res);
    res.push(RowItem::LineBreak);
    append_sysinfo(&mut res);

    Ok(res.into_boxed_slice())
}

fn append_uname(res: &mut Vec<RowItem>) {
    let uname = uname();
    res.push(RowItem::KV(
        "Hostname",
        uname.nodename().to_string_lossy().to_string(),
    ));
    res.push(RowItem::KV(
        "Kernel",
        uname.release().to_string_lossy().to_string(),
    ));
    res.push(RowItem::KV(
        "Kernel Ver.",
        uname.version().to_string_lossy().to_string(),
    ));
}

fn append_sysinfo(res: &mut Vec<RowItem>) {
    let sysinfo = sysinfo();
    let uptime = TimeDelta::seconds(sysinfo.uptime);
    res.push(RowItem::KV("Uptime", format_duration(uptime)));
    res.push(RowItem::KV("Load Avg.", format_loads(&sysinfo.loads)));
    res.push(RowItem::KV(
        "RAM",
        format!(
            "{} / {} Mi (Free / Total)",
            sysinfo.freeram / 1024 / 1024,
            sysinfo.totalram / 1024 / 1024
        ),
    ));
    res.push(RowItem::KV("Swap", format!("{} / {} Mi (Free / Total)", sysinfo.freeswap / 1024 / 1024, sysinfo.totalswap / 1024 / 1024)));
    res.push(RowItem::KV("Procs", sysinfo.procs.to_string()));
}

fn format_loads(loads: &[u64; 3]) -> String {
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

fn format_duration(input: TimeDelta) -> String {
    let day_part = if input.num_days() != 0 {
        format!("{} days, ", input.num_days())
    } else {
        "".to_string()
    };

    let time_part = format!("{}:{:02}", input.num_hours() % 24, input.num_minutes() % 60);

    format!("{}{}", day_part, time_part)
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
