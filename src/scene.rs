use std::{io::{self, Stdout, Write}};
use crossterm::{
    cursor, 
    style::{self, Print, Stylize},
    terminal,
    ExecutableCommand, 
    QueueableCommand};


use crate::snake::Snake;

pub struct Scene<'a> {
    width: u16,
    height: u16,
    buffer: &'a mut Stdout,
    border_width: u16
}

impl<'a> Scene<'a> {
    pub fn new(buffer: &'a mut Stdout, width: u16, height: u16, border_width: u16) -> Self {
        Scene{
            width,
            height,
            buffer,
            border_width
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
                if (x == 0) || (x >= self.width - self.border_width) {
                    self.buffer
                        .queue(cursor::MoveTo(x,y))?                        
                        .queue(style::PrintStyledContent( "█".red()))?;
                }
                if y == 0 {
                   self.buffer
                        .queue(cursor::MoveTo(x,y))?                        
                        .queue(style::PrintStyledContent( "▄".red()))?;
                }
                if y >= self.height - self.border_width {
                    self.buffer
                        .queue(cursor::MoveTo(x,y))?                        
                        .queue(style::PrintStyledContent( "▀".red()))?;
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
                    .queue(style::PrintStyledContent( "█".green()))?;
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
}
