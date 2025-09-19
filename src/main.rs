mod boid;
mod vec2;

use boid::Boid;
use vec2::Vec2;

use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use rand::Rng;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;


struct MainState
{
    boids: Vec<Boid>,
}

impl MainState
{
    fn new() -> GameResult<MainState>
    {
        let mut rng = rand::thread_rng();
        let mut boids = Vec::new();
        for _ in 0..100
        {
            boids.push(
                    Boid
                    {
                    pos:
                        Vec2
                        {
                            x: rng.gen_range(0.0..WINDOW_WIDTH),
                            y: rng.gen_range(0.0..WINDOW_HEIGHT),
                        },
                    vel:
                        Vec2
                        {
                            x: rng.gen_range(-2.0..2.0),
                            y: rng.gen_range(-2.0..2.0),
                        },
                    });
        }
        Ok(MainState { boids })
    }
}

impl EventHandler for MainState
{
    fn update(&mut self, _ctx: &mut Context) -> GameResult
    {
        let boids_copy = self.boids.clone();
        
        for boid in &mut self.boids
        {
            boid.update(&boids_copy);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult
    {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        for boid in &self.boids
        {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                [boid.pos.x, boid.pos.y],
                3.0,
                0.1,
                Color::WHITE,
            )?;

            canvas.draw(&circle, graphics::DrawParam::default());
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult
{
    let (ctx, event_loop) = ggez::ContextBuilder::new("boided", "easteregg")
        .window_setup(ggez::conf::WindowSetup::default().title("Boided"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
