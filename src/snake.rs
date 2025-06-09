#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: Vec<(u16,u16)>,
    direction: Direction
}

impl Snake {
    pub fn new(start_x: u16, start_y: u16, direction: Direction) -> Self {
        Snake {
            body: vec![(start_x, start_y)],
            direction
        }
    }

    pub fn body(&self) ->  Vec<(u16,u16)> {
        self.body.clone()
    }

    pub fn direction(&self) -> Direction {
        self.direction
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