use std::{io::{self, Stdout, Write}, time};
use crossterm::{
    cursor, 
    event::{self, Event, KeyCode, KeyEvent}, 
    style::Print, terminal
};
use std::thread;

pub mod scene;
pub mod snake;

use snake::Direction;
use snake::Snake;
use scene::Scene;


fn init_game(stdout: &mut Stdout) -> io::Result<(Snake,Scene)>{
    terminal::enable_raw_mode()?; // <- enable raw mode
    // Enter alternate mode to avoid resize
    crossterm::execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    clear(stdout)?;

    let (term_width, term_height) = terminal::size()?;
    let width = std::cmp::min(150, term_width);
    let height = std::cmp::min(40, term_height);
    let border_width = 1;

    let snake = Snake::new(width / 2, height / 2, Direction::Down);
    let scene= Scene::new(stdout, width, height, border_width);
    Ok((snake, scene))

}

fn clear(stdout: &mut Stdout) -> io::Result<()>{
    crossterm::execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    crossterm::execute!(stdout, cursor::MoveTo(0,0))?;
    Ok(())

}

fn exit_game(stdout: &mut Stdout) -> io::Result<()> {
    terminal::disable_raw_mode()?; // <- cleanup
    // Revenir à l'écran normal et montrer le curseur
    crossterm::execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    clear(stdout)?;
    Ok(())
}

fn pause_game(stdout: &mut Stdout, scene: &Scene)  -> io::Result<()> {
    crossterm::queue!(stdout, cursor::MoveTo(scene.width()/2 as u16, scene.height()/2 as u16), Print("Press SPACE to continue"))?;
    stdout.flush()?;
    loop {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char(' ') => {
                    crossterm::queue!(stdout, 
                    cursor::MoveTo(scene.width()/2 as u16, scene.height()/2 as u16), 
                    Print("                       "))?;
                    stdout.flush()?;
                    break
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {

    let mut stdout = io::stdout();

    let (mut snake, mut scene) = init_game(&mut stdout)?;

    scene.draw_border()?;
    scene.draw_snake(&snake)?;

    scene.add_fruit(&snake);
    scene.draw_fruit()?;

    let mut end_of_game = false; 
    let mut add_segment = false;
    const DURATION_LOOP_MS: u64 = 100;

    loop {
        let mut latest_direction = None;

        // Drain all input events
        while event::poll(time::Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                let current_direction = snake.direction();

                match code {
                    KeyCode::Up if current_direction != Direction::Down => {
                        latest_direction = Some(Direction::Up);
                    }
                    KeyCode::Down if current_direction != Direction::Up => {
                        latest_direction = Some(Direction::Down);
                    }
                    KeyCode::Left if current_direction != Direction::Right => {
                        latest_direction = Some(Direction::Left);
                    }
                    KeyCode::Right if current_direction != Direction::Left => {
                        latest_direction = Some(Direction::Right);
                    }
                    KeyCode::Enter if end_of_game == true => {
                        //restart
                        (snake, scene) = init_game(&mut stdout)?;
                        scene.draw_border()?;
                        scene.draw_snake(&snake)?;
                        scene.add_fruit(&snake);
                        scene.draw_fruit()?;
                        end_of_game = false;

                    }
                    KeyCode::Char(' ') => {
                        pause_game(&mut io::stdout(),&scene)?;
                    }
                    KeyCode::Esc => return exit_game(&mut io::stdout()), // quit the game
                    _ => {}
                }
            }
        }

        if scene.is_snake_out_of_bound(&snake) || scene.is_snake_touch_himself(&snake) {
            crossterm::queue!(&mut io::stdout(), cursor::MoveTo(scene.width()/3 as u16, scene.height()/2 as u16), Print("LOST Again/Exit: ENTER/ECHAP"))?;
            io::stdout().flush()?;
            end_of_game = true;
            continue;
        }        

        // Apply only the latest direction change if any
        if let Some(new_dir) = latest_direction {
            snake.update_direction(new_dir);
        }

        scene.clear_snake(&snake)?;        
        snake.move_snake(add_segment);
        scene.draw_snake(&snake)?;
        scene.draw_fruit()?;

        if scene.snake_ate_food(&snake) {
            add_segment = true;
            scene.add_fruit(&snake);
            scene.draw_fruit()?;
        }
        else {
            add_segment = false;
        }

        thread::sleep(time::Duration::from_millis(DURATION_LOOP_MS));        
    }
}