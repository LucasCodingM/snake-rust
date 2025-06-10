use std::{io::{self, Stdout, Write}};
use crossterm::{
    cursor, 
    style::{self, Print, Stylize},
    terminal,
    ExecutableCommand, 
    QueueableCommand};


use crate::snake::Snake;
use rand::{prelude::IndexedRandom};

pub struct Scene<'a> {
    width: u16,
    height: u16,
    buffer: &'a mut Stdout,
    border_width: u16,
    list_fruits: Vec<(u16,u16)>
}

impl<'a> Scene<'a> {
    pub fn new(buffer: &'a mut Stdout, width: u16, height: u16, border_width: u16) -> Self {
        Scene{
            width,
            height,
            buffer,
            border_width,
            list_fruits: Vec::new()
        }
    }

    pub fn width(&self) ->u16 {
        self.width
    }

    pub fn height(&self) ->u16 {
        self.height
    }

    pub fn clear_snake(&mut self, snake: &Snake) -> io::Result<()> {

        for segment in snake.body().iter() {
            let (x,y) = *segment;
            self.buffer
                    .queue(cursor::MoveTo(x,y))?
                    .queue(Print("  "))?;
        }
        self.buffer.flush()?;
        Ok(())
    }

    pub fn draw_border(&mut self) -> io::Result<()> {
        self.buffer.execute(terminal::Clear(terminal::ClearType::All))?;

        for y in 0..self.height {
            for x in 0..self.width {
                if (x == 0) || (x >= self.width - self.border_width) || (y == 0) || (y >= self.height - self.border_width){
                    self.buffer
                        .queue(cursor::MoveTo(x,y))?                        
                        .queue(style::PrintStyledContent( "█".dark_red()))?;
                }
            }   
        }
        self.buffer.flush()?;
        Ok(())
    }

    pub fn draw_snake(&mut self, snake: &Snake) -> io::Result<()> {
        if let Some(head) = snake.body().first() {
           let (x_head,y_head) = *head;
           self.buffer
                    .queue(cursor::MoveTo(x_head,y_head))?
                    .queue(style::PrintStyledContent( "█".blue()))?;
        }
        
        for segment in snake.body().iter().skip(1) {
            let (x,y) = *segment;
            self.buffer
                    .queue(cursor::MoveTo(x,y))?
                    .queue(style::PrintStyledContent( "░".green()))?;
        }
        self.buffer.flush()?;
        Ok(())
    }

    pub fn is_snake_out_of_bound(&self, snake: &Snake) -> bool {
        let body = snake.body();
        let(x, y) = body.first().unwrap();
        if (*x<= self.border_width) || (*x >= self.width - self.border_width) ||
         (*y<=self.border_width) || (*y >= self.height - self.border_width) {
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
        match Scene::<'a>::find_duplicate_tuples(&snake.body()) {
            Some(_) => true,
            None => false,
        }
    }

    fn generate_random_tuple(&self, range_x: std::ops::Range<u16>,
                                range_y: std::ops::Range<u16>,
                                    excluded: &[(u16, u16)]) -> Option<(u16, u16)> {
        let mut all_values = Vec::new();

        for x in range_x.clone() {
            for y in range_y.clone() {
                let tuple = (x, y);
                if !excluded.contains(&tuple) {
                    all_values.push(tuple);
                }
            }
        }

            let mut rng = rand::rng();
            all_values.choose(&mut rng).copied()
    }


    pub fn add_fruit(&mut self, snake: &Snake) {

        if let Some(fruit) = self.generate_random_tuple(std::ops::Range { start: 1, end: self.width - 2 },
            std::ops::Range { start: 1, end: self.height - 2 },&snake.body()) {
                self.list_fruits.push(fruit);
            }
    }

    pub fn draw_fruit(&mut self) -> io::Result<()>{
        for (x_fruit, y_fruit) in self.list_fruits.iter() {
            self.buffer
                    .queue(cursor::MoveTo(*x_fruit,*y_fruit))?
                    .queue(style::PrintStyledContent("●".yellow()))?;
        }  
        self.buffer.flush()?;
        Ok(())
    }

    pub fn remove_fruit(&mut self, fruit_to_remove: &(u16,u16)) -> bool{
        let _ = self.buffer.flush();
        if let Some(index) = self.list_fruits.iter().position(|fruit| fruit == fruit_to_remove) {
            self.list_fruits.remove(index);
            return true;
        }
        false
    }

    pub fn snake_ate_food(&mut self, snake: &Snake) -> bool {

        if let Some(head_pos) = snake.body().first() {
            return self.remove_fruit(head_pos);
        }
        false
    }
}
