use ggez::{Context, GameResult};
use ggez::event;
use ggez::graphics;
use ggez::timer;
use super::Gameboy;

impl event::EventHandler for Gameboy {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if timer::ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::delta(ctx));
            println!("Average FPS: {}", timer::fps(ctx));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        let mut frame_complete = false;
        while !frame_complete {
            self.clock();
            frame_complete = self.frame_complete();
        }

        Ok(())
    }
}