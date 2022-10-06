use crossterm::{
    // cursor::{Hide, MoveTo, MoveToColumn, MoveUp, RestorePosition, SavePosition},
    // event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    // style::{Print, Stylize},
    execute,
    queue,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, stdout, Write},
    time::{Duration, Instant},
};

fn main() -> io::Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    let now = Instant::now();
    
    for i in 0..5_000 {
        println!("{}", i);
        // execute!(stdout(), Clear(ClearType::FromCursorUp))?;
    }
    
    execute!(stdout(), Clear(ClearType::FromCursorUp))?;
    let elapsed_time = now.elapsed();

    println!("Hello World!\nfunction took {} seconds.", elapsed_time.as_secs());


    // execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}
