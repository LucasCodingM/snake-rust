use std::{io::{self, Stdout, Write}, time};
use crossterm::{
    cursor, 
    event::{self, Event, KeyCode, KeyEvent}, 
    style::{self, Print, Stylize}, terminal, ExecutableCommand, QueueableCommand
};
use std::thread;

pub struct Scene<'a> {
    pub width: u16,
    pub height: u16,
    pub buffer: &'a mut Stdout,
}

impl<'a> Scene<'a> {
    pub fn new(buffer: &'a mut Stdout, width: u16, height: u16) -> Self {
        Scene{
            width,
            height,
            buffer,
        }
    }

    pub fn clear_snake(&mut self, snake: &Snake) -> io::Result<()> {

        for segment in snake.body.iter() {
            let (x,y) = *segment;
            self.buffer
                    .queue(cursor::MoveTo(x,y))?
                    .queue(Print(" "))?;
        }
        self.buffer.flush()?;
        Ok(())
    }

    pub fn draw_border(&mut self) -> io::Result<()> {
        self.buffer.execute(terminal::Clear(terminal::ClearType::All))?;

        for y in 0..self.height {
            for x in 0..self.width {
                if (y == 0 || y == self.height - 1) || (x == 0 || x == self.width - 1) {
                    // in this loop we are more efficient by not flushing the buffer.
/*                     if y % 2 != 0 {
                       self.buffer
                        .queue(cursor::MoveTo(x,y))?
                        .queue(style::PrintStyledContent( "▀".red()))?; 
                    }
                    else {
                        self.buffer
                        .queue(cursor::MoveTo(x,y))?                        
                        .queue(style::PrintStyledContent( "▄".red()))?;
                    } */
                   self.buffer
                        .queue(cursor::MoveTo(x,y))?                        
                        .queue(style::PrintStyledContent( "▄".red()))?;
                }
            }   
        }
        self.buffer.flush()?;
        Ok(())
    }

    pub fn draw_snake(&mut self, snake: &Snake) -> io::Result<()> {
        if let Some(head) = snake.body.first() {
           let (x_head,y_head) = *head;
           self.buffer
                    .queue(cursor::MoveTo(x_head,y_head))?
                    .queue(style::PrintStyledContent( "▀".blue()))?;
        }
        
        for segment in snake.body.iter().skip(1) {
            let (x,y) = *segment;
            self.buffer
                    .queue(cursor::MoveTo(x,y))?
                    .queue(style::PrintStyledContent( "▀".green()))?;
        }
        self.buffer.flush()?;
        Ok(())
    }

    pub fn is_snake_out_of_bound(&self, snake: &Snake) -> bool {
        let(x, y) = snake.body.first().unwrap();
        if (*x<=1) || (*x >= self.width - 1) || (*y<=1) || (*y >= self.height - 1) {
            return true;
        }
        false
    }

    fn find_duplicate_tuples(v: &Vec<(u16, u16)>) -> Option<(u16, u16)> {
        let mut seen = Vec::new();
        for tuple in v.iter() {
            if seen.contains(tuple) {
                return Some(*tuple);
            }
            seen.push(*tuple);
        }
        None
    }

    pub fn is_snake_touch_himself(&self, snake: &Snake) -> bool {
        match Scene::<'a>::find_duplicate_tuples(&snake.body) {
            Some(_) => true,
            None => false,
        }
    }
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub body: Vec<(u16,u16)>,
    pub direction: Direction
}

impl Snake {
    pub fn new(start_x: u16, start_y: u16, direction: Direction) -> Self {
        Snake {
            body: vec![(start_x, start_y)],
            direction
        }
    }

    pub fn update_snake(&mut self, new_head_pos: (u16,u16), add_segment: bool) {
        // Save the old head position
        let mut prev_pos = self.body[0];
        // Update head to the new position
        self.body[0] = new_head_pos;

        // Update each body segment to the previous segment's old position
        for segment in self.body.iter_mut().skip(1) {
            let current_pos = *segment;
            *segment = prev_pos;
            prev_pos = current_pos;
        }
        if add_segment {
            self.add_segment_body(prev_pos);
        }
    }

    pub fn update_direction(&mut self, direction: Direction) {
        self.direction = direction
    }

    pub fn move_right(&mut self, add_segment: bool) {
        let (x_head, y_head) = self.body.first().unwrap();
        let new_head_pos = (*x_head + 1 , *y_head);
        self.update_snake(new_head_pos, add_segment);
    }

    pub fn move_left(&mut self, add_segment: bool) {
        let (x_head, y_head) = self.body.first().unwrap();
        let new_head_pos = if *x_head > 0 {
            (*x_head - 1, *y_head)
        } else {
            // Handle wrapping or block movement here
            (*x_head, *y_head)
        };
        self.update_snake(new_head_pos, add_segment);
    }

    pub fn move_up(&mut self, add_segment: bool) {
        let (x_head, y_head) = self.body.first().unwrap();
        let new_head_pos = if *y_head > 0 {
            (*x_head, *y_head - 1)
        } else {
            // Handle wrapping or block movement here
            (*x_head, *y_head)
        };
        self.update_snake(new_head_pos, add_segment);
    }

    pub fn move_down(&mut self, add_segment: bool) {
        let (x_head, y_head) = self.body.first().unwrap();
        let new_head_pos = (*x_head , *y_head + 1);
        self.update_snake(new_head_pos, add_segment);
    }

    pub fn move_snake(&mut self, add_segment: bool) {
        match self.direction {
            Direction::Right => self.move_right(add_segment),
            Direction::Left => self.move_left(add_segment),
            Direction::Up => self.move_up(add_segment),
            Direction::Down => self.move_down(add_segment),            
        }
    }

    pub fn add_segment_body(&mut self, pos: (u16,u16)) {
        self.body.push(pos);
    }
}

fn init_game(stdout: &mut Stdout) -> io::Result<(Snake,Scene)>{
    terminal::enable_raw_mode()?; // <- enable raw mode
    // Passer en mode alternatif pour éviter les problèmes de redimensionnement
    crossterm::execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    clear(stdout)?;

    let (term_width, term_height) = terminal::size()?;
    let width = std::cmp::min(150, term_width);
    let height = std::cmp::min(40, term_height);

    let snake = Snake::new(width / 2, height / 2, Direction::Down);
    let scene= Scene::new(stdout, width, height);
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

fn main() -> io::Result<()> {

    let mut stdout = io::stdout();

    let (mut snake, mut scene) = init_game(&mut stdout)?;

    scene.draw_border()?;
    scene.draw_snake(&snake)?;

    let mut end_of_game = false;
    
    let mut count_loop = 0;
    let mut add_segment = false;
    const LOOP_COUNT_INCREASE_SNAKE_SIZE: u32 = 5;
    const DURATION_LOOP_MS: u64 = 300;

    loop {
        let mut latest_direction = None;

        // Drain all input events
        while event::poll(time::Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                let current_direction = snake.direction;

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
                        end_of_game = false;

                    }
                    KeyCode::Esc => return exit_game(&mut io::stdout()), // quit the game
                    _ => {}
                }
            }
        }

        if scene.is_snake_out_of_bound(&snake) || scene.is_snake_touch_himself(&snake) {
            crossterm::queue!(&mut io::stdout(), cursor::MoveTo(scene.width/3 as u16, scene.height/2 as u16), Print("LOST Again/Exit: ENTER/ECHAP"))?;
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



        thread::sleep(time::Duration::from_millis(DURATION_LOOP_MS));

        if count_loop % LOOP_COUNT_INCREASE_SNAKE_SIZE == 0 {
            add_segment = true
        }
        else {
            add_segment = false
        }
        count_loop += 1;
    }
}