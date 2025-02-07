use crossterm::{
    cursor::{MoveTo, Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    execute,
};
use std::io::{self, stdout, Write};

pub fn print_options(results: Vec<String>) -> io::Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), Hide)?;
    
    let mut selected = 0;

    print_selections(&results, selected)?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < results.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0), Show)?;
                    disable_raw_mode()?;
                    println!("Selected: {}", results[selected]);
                    return Ok(());
                }
                KeyCode::Esc => {
                    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0), Show)?;
                    disable_raw_mode()?;
                    return Ok(());
                }
                _ => {}
            }
            print_selections(&results, selected)?;
        }
    }
}

fn print_selections(
    results: &Vec<String>,
    selected: usize,
) -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    for (i, option) in results.iter().enumerate() {
        execute!(stdout, MoveTo(0, i as u16))?;
        if i == selected {
            write!(stdout, "> {}", option)?;
        } else {
            write!(stdout, "  {}", option)?;
        }
    }
    stdout.flush()?;
    Ok(())
}