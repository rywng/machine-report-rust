use chrono::TimeDelta;
use rustix::system::sysinfo;
use rustix::system::uname;

use crate::draw::format_loads;
use std::io;

use crate::draw::format_duration;

use crate::RowItem;

pub(crate) fn get_device_info() -> io::Result<Box<[RowItem]>> {
    let mut res: Vec<RowItem> = Vec::with_capacity(16);

    append_uname(&mut res);
    res.push(RowItem::LineBreak);
    append_sysinfo(&mut res);

    Ok(res.into_boxed_slice())
}

pub(crate) fn append_uname(res: &mut Vec<RowItem>) {
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

pub(crate) fn append_sysinfo(res: &mut Vec<RowItem>) {
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
