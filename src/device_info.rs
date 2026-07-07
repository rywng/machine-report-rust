use chrono::TimeDelta;
use rustix::system::sysinfo;
use rustix::system::uname;

use std::io;

use crate::RowItem;

pub fn get_device_info() -> io::Result<Box<[RowItem]>> {
    let mut res: Vec<RowItem> = Vec::with_capacity(16);

    append_uname(&mut res);
    res.push(RowItem::LineBreak);
    append_sysinfo(&mut res);

    Ok(res.into_boxed_slice())
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
    res.push(RowItem::KV(
        "Swap",
        format!(
            "{} / {} Mi (Free / Total)",
            sysinfo.freeswap / 1024 / 1024,
            sysinfo.totalswap / 1024 / 1024
        ),
    ));
    res.push(RowItem::KV("Procs", sysinfo.procs.to_string()));
}
