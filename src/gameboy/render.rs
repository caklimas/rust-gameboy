use ggez::{Context, GameResult};
use ggez::event;
use ggez::graphics::{self, Color, DrawMode, DrawParam, FillOptions, MeshBuilder};

use super::Gameboy;
use crate::constants::gpu::*;
use crate::constants::screen::*;

impl Gameboy {
    fn draw_screen(&mut self, ctx: &mut Context) {
        let mut builder = MeshBuilder::new();
        let screen = self.get_screen();
        let mut coordinates = 0usize;
        for i in (0..screen.len()).step_by(COLOR_PER_PIXEL) {
            let r = screen[i];
            let g = screen[i + 1];
            let b = screen[i + 2];
            let color = Color::from_rgb(r, g, b);
            let y_offset = (coordinates / (SCREEN_WIDTH as usize)) * PIXEL_SIZE;
            let x_offset = (coordinates % (SCREEN_WIDTH as usize)) * PIXEL_SIZE;
            let rectangle = graphics::Rect::new(
                x_offset as f32,
                y_offset as f32,
                PIXEL_SIZE as f32,
                PIXEL_SIZE as f32
            );

            if color != Color::from_rgb(RGB_WHITE.0, RGB_WHITE.1, RGB_WHITE.2) {
                let aasd = 213;
            }

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