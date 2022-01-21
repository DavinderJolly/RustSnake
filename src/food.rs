use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use rand::{thread_rng, Rng};

pub struct Food {
    pub pos_x: i32,
    pub pos_y: i32,
}

impl Food {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square =
            graphics::rectangle::square((self.pos_x * 40) as f64, (self.pos_y * 40) as f64, 40_f64);
        gl.draw(args.viewport(), |c, gl| {
            graphics::rectangle(RED, square, c.transform, gl);
        });
    }

    pub fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.pos_x = rng.gen_range(0..20);
        self.pos_y = rng.gen_range(0..20);
    }
}
