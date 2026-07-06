use std::io::{self, Cursor, Write, stdout};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor, queue,
    style::{self, Stylize},
    terminal,
};

fn main() -> anyhow::Result<()> {
    let mut output = stdout();

    output.execute(terminal::Clear(terminal::ClearType::All))?;

    let (rows, cols, t_col) = (24, 64, 12);

    let header_top = |x: u16, output: &mut dyn Write| -> io::Result<()> {
        print_divider(output, x, cols, '┌', '┬', '┐')
    };

    let header_bottom = |x: u16, output: &mut dyn Write| -> io::Result<()> {
        print_divider(output, x, cols, '├', '┴', '┤')
    };

    let bottom_div = |x: u16, output: &mut dyn Write| -> io::Result<()> {
        print_divider_with_t(output, x, cols, t_col, '└', '─', '┴', '┘')
    };

    let content_div = |x: u16, output: &mut dyn Write| -> io::Result<()> {
        print_divider_with_t(output, x, cols, t_col, '├', '─', '┼', '┤')
    };

    let content_div_top = |x: u16, output: &mut dyn Write| -> io::Result<()> {
        print_divider_with_t(output, x, cols, t_col, '├', '─', '┬', '┤')
    };

    let content_div_middle = |x: u16, output: &mut dyn Write| -> io::Result<()> {
        print_divider(output, x, cols, '├', '─', '┤')
    };

    let print_middle_text = |x: u16, output: &mut dyn Write, content: String| -> io::Result<()> {
        let before_whitespace = (cols as usize - content.chars().count()) / 2;
        if x == 0 || x == cols - 1 {
            output.queue(style::Print('│'))?;
        } else if x as usize == before_whitespace {
            output.queue(style::PrintStyledContent("test".bold()))?;
        }

        Ok(())
    };

    for y in 0..rows {
        for x in 0..cols {
            output.queue(cursor::MoveTo(x, y))?;
            match y {
                0 => {
                    header_top(x, &mut output)?;
                }
                1 => {
                    header_bottom(x, &mut output)?;
                }
                2 => {
                    print_middle_text(x, &mut output, "abc".to_string())?;
                }
                3 => {
                    content_div_top(x, &mut output)?;
                }
                bottom if bottom == rows - 1 => {
                    bottom_div(x, &mut output)?;
                }
                _ => {
                    if (y - 3) % 2 == 1 {
                        output
                            .queue(style::Print('│'))?
                            .queue(cursor::MoveTo(2, y))?
                            .queue(style::PrintStyledContent("asdf".bold()))?
                            .queue(cursor::MoveTo(t_col, y))?
                            .queue(style::Print('│'))?
                            .queue(cursor::MoveRight(1))?
                            .queue(style::PrintStyledContent("content".italic()))?
                            .queue(cursor::MoveTo(cols - 1, y))?
                            .queue(style::Print('│'))?;
                        break;
                    } else {
                        content_div(x, &mut output)?;
                    }
                }
            }
        }
    }

    output.flush()?;

    Ok(())
}

fn print_divider(
    output: &mut dyn Write,
    x: u16,
    cols: u16,
    l: char,
    m: char,
    r: char,
) -> io::Result<()> {
    Ok(match x {
        0 => {
            output.queue(style::Print(l))?;
        }
        right if right == cols - 1 => {
            output.queue(style::Print(r))?;
        }
        _ => {
            output.queue(style::Print(m))?;
        }
    })
}

fn print_divider_with_t(
    output: &mut dyn Write,
    x: u16,
    cols: u16,
    t_col: u16,
    l: char,
    m: char,
    t: char,
    r: char,
) -> io::Result<()> {
    if x == 0 {
        output.queue(style::Print(l))?;
    } else if x == t_col {
        output.queue(style::Print(t))?;
    } else if x == cols - 1 {
        output.queue(style::Print(r))?;
    } else {
        output.queue(style::Print(m))?;
    }
    Ok(())
}
