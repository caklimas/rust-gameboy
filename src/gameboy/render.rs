use ggez::{Context, GameResult, nalgebra as na};
use ggez::event;
use ggez::graphics::{self, Color, DrawMode, DrawParam, FillOptions, Image, MeshBuilder};
use ggez::timer;

use super::Gameboy;
use crate::constants::gpu::*;
use crate::constants::screen::*;

impl Gameboy {
    fn draw_screen(&mut self, ctx: &mut Context) {
        let mut builder = MeshBuilder::new();
        let screen = self.get_screen();
        let mut coordinates = 0u16;
        for i in (0..screen.len()).step_by(COLOR_PER_PIXEL) {
            let r = screen[i];
            let g = screen[i + 1];
            let b = screen[i + 2];
            let color = Color::from_rgb(r, g, b);
            let rectangle = graphics::Rect::new(
                (coordinates % SCREEN_WIDTH) as f32,
                (coordinates / SCREEN_WIDTH) as f32,
                1.0,
                1.0
            );
            builder.rectangle(DrawMode::Fill(FillOptions::DEFAULT), rectangle, color);
            coordinates += 1;
        }

        let mesh = builder.build(ctx).expect("Error building screen");
        graphics::draw(ctx, &mesh, DrawParam::new()).expect("Error drawing the screen");
    }
}

impl event::EventHandler for Gameboy {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        let mut frame_complete = false;
        while !frame_complete {
            self.clock();
            frame_complete = self.frame_complete();
        }

        self.draw_screen(ctx);
        graphics::present(ctx).expect("Error rendering the screen");

        Ok(())
    }
}