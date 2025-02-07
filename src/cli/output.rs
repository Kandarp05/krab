use crossterm::{
    cursor::{MoveTo, position},
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, stdout, Write};

pub fn print_options(results: Vec<String>) -> io::Result<()> {
    let (start_x, start_y) = position()?;
    enable_raw_mode()?;
    let mut selected = 0;

    print_selections(&results, selected, start_x, start_y)?;

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
                    disable_raw_mode()?;
                    println!("You selected: {}", results[selected]);
                    return Ok(());
                }

                KeyCode::Esc => {
                    disable_raw_mode()?;
                    return Ok(());
                }

                _ => {}
            }
        }

        print_selections(&results, selected,  start_x,  start_y)?;
    }
}

fn print_selections(
    results: &Vec<String>,
    selected: usize,
    start_x: u16, 
    start_y: u16,
) -> io::Result<()> {
    let mut stdout = stdout();

    stdout.queue(MoveTo(start_x, start_y))?;
    stdout.queue(Clear(ClearType::FromCursorDown))?;

    for (i, option) in results.iter().enumerate() {
        if i == selected {
            print!("> {}\r", option);
        } else {
            print!("  {}\r", option);
        }
    }
    stdout.flush()?;
    Ok(())

}