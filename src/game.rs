use crate::food::Food;
use crate::snake::{Direction, Snake};
use graphics::glyph_cache::rusttype::GlyphCache;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, TextureSettings};
use piston::input::{Button, Key, RenderArgs};
pub struct Game {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub food: Food,
    pub score: i32,
    pub high_score: i32,
}

impl Game {
    pub fn render(&mut self, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, g| {
            graphics::clear(BLACK, g);
        });

        self.snake.render(&mut self.gl, args);

        self.food.render(&mut self.gl, args);
    }

    pub fn render_score(&mut self, args: &RenderArgs) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let mut glyph_cache =
            GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
        self.gl.draw(args.viewport(), |c, g| {
            graphics::Text::new_color(WHITE, 24)
                .draw(
                    &format!("Score: {}", self.score),
                    &mut glyph_cache,
                    &c.draw_state,
                    c.transform.trans(10.0, 50.0),
                    g,
                )
                .unwrap();
            graphics::Text::new_color(WHITE, 24)
                .draw(
                    &format!("HighScore: {}", self.high_score),
                    &mut glyph_cache,
                    &c.draw_state,
                    c.transform.trans(620.0, 50.0),
                    g,
                )
                .unwrap();
        });
    }

    pub fn update(&mut self) {
        self.snake.update();
        if self.snake.detect_collision() {
            self.food.randomize();
            self.high_score = if self.score > self.high_score {
                self.score
            } else {
                self.high_score
            };
            self.score = 0;
        }
        if self.snake.body.front().unwrap() == &(self.food.pos_x, self.food.pos_y) {
            self.snake.grow();
            self.food.randomize();
            self.score += 1;
        }
    }

    pub fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction,
        }
    }
}
