extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod food;
mod game;
mod snake;

use food::Food;
use game::Game;
use snake::{Direction, Snake};

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{ButtonEvent, ButtonState, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use rand::{thread_rng, Rng};

use std::collections::LinkedList;
use std::iter::{FromIterator, IntoIterator};

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("RustySnake", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut rng = thread_rng();
    let food_x: i32 = rng.gen_range(0..20);
    let food_y: i32 = rng.gen_range(0..20);

    let game = &mut Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (1, 0), (2, 0)]).into_iter()),
            dir: Direction::Right,
        },
        food: Food {
            pos_x: food_x,
            pos_y: food_y,
        },
        score: 0,
        high_score: 0,
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
            game.render_score(&r);
        }

        if let Some(_u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
