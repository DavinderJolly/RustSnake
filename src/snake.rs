use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::collections::LinkedList;
use std::iter::IntoIterator;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct Snake {
    pub body: LinkedList<(i32, i32)>,
    pub dir: Direction,
}

impl Snake {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 40) as f64, (y * 40) as f64, 40_f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter().for_each(|square| {
                graphics::rectangle(GREEN, square, transform, gl);
            });
        });
    }

    pub fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.handle_bounds(&mut new_head);

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }

    pub fn detect_collision(&mut self) -> bool {
        let head = *self.body.front().expect("Snake has no body");
        let mut body = self.body.clone();
        body.pop_front();

        if body.contains(&head) {
            self.body = LinkedList::new();
            self.body.push_back(head);
            return true;
        }
        return false;
    }

    pub fn grow(&mut self) {
        let last_tail = self.body.back().unwrap();
        let new_tail = match self.dir {
            Direction::Left => (last_tail.0 - 1, last_tail.1),
            Direction::Right => (last_tail.0 + 1, last_tail.1),
            Direction::Up => (last_tail.0, last_tail.1 - 1),
            Direction::Down => (last_tail.0, last_tail.1 + 1),
        };

        self.body.push_back(new_tail);
    }

    fn handle_bounds(&mut self, new_head: &mut (i32, i32)) {
        if new_head.0 < 0 {
            new_head.0 = 19;
        }
        if new_head.0 > 19 {
            new_head.0 = 0;
        }
        if new_head.1 < 0 {
            new_head.1 = 19;
        }
        if new_head.1 > 19 {
            new_head.1 = 0;
        }
    }
}
