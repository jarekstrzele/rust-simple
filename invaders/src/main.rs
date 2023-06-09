use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;



fn main() -> Result<(), Box<dyn Error>>{
    let mut audio = Audio::new() ;
    audio.add("explode", "explode.wav") ;
    audio.add("lose", "lose.wav") ;
    audio.add("move", "move.wav") ;
    audio.add("pew", "pew.wav") ;
    audio.add("startup", "startup.wav") ;
    audio.add("win", "win.wav") ;
    audio.play("startup") ;


    //terminal
    let mut stdout = io::stdout() ;
    terminal::enable_raw_mode()? ;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;


    // rander loop in a separate thread
    


    //game loop
    'gameloop: loop {
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event::read()?{
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') =>{
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ =>{}
                }
            }
        }
    }


    // CLEANUP
    audio.wait() ;
    stdout.execute(Show)? ;
    stdout.execute(LeaveAlternateScreen)? ;
    terminal::disable_raw_mode()? ;
    Ok(())
}
