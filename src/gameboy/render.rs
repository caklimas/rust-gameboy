// use ggez::{Context, GameResult};
// use ggez::event::{self, KeyCode, KeyMods};
// use ggez::graphics::{self, Color, DrawMode, DrawParam, FillOptions, MeshBuilder};

// use super::Gameboy;
// use crate::constants::gpu::*;
// use crate::constants::screen::*;

// impl Gameboy {
//     fn draw_screen(&mut self, ctx: &mut Context) {
//         let mut builder = MeshBuilder::new();
//         let screen = self.get_screen();
//         let mut coordinates = 0usize;
//         for i in (0..screen.len()).step_by(COLOR_PER_PIXEL) {
//             let r = screen[i];
//             let g = screen[i + 1];
//             let b = screen[i + 2];
//             let color = Color::from_rgb(r, g, b);
//             let y_offset = (coordinates / (SCREEN_WIDTH as usize)) * PIXEL_SIZE;
//             let x_offset = (coordinates % (SCREEN_WIDTH as usize)) * PIXEL_SIZE;
//             let rectangle = graphics::Rect::new(
//                 x_offset as f32,
//                 y_offset as f32,
//                 PIXEL_SIZE as f32,
//                 PIXEL_SIZE as f32
//             );
//             builder.rectangle(DrawMode::Fill(FillOptions::DEFAULT), rectangle, color);
//             coordinates += 1;
//         }

//         let mesh = builder.build(ctx).expect("Error building screen");
//         graphics::draw(ctx, &mesh, DrawParam::new()).expect("Error drawing the screen");
//     }
// }

// impl event::EventHandler for Gameboy {
//     fn update(&mut self, _ctx: &mut Context) -> GameResult {
//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult {
//         graphics::clear(ctx, graphics::WHITE);
//         let mut frame_complete = false;
//         while !frame_complete {
//             self.clock();
//             frame_complete = self.frame_complete();
//         }

//         self.draw_screen(ctx);
//         graphics::present(ctx).expect("Error rendering the screen");

//         Ok(())
//     }

//     fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
//         let mut controls_updated = false;
//         let mut current_controls = self.get_controls();

//         match keycode {
//             KeyCode::Up => {
//                 controls_updated = true;
//                 current_controls.up = true;
//             },
//             KeyCode::Down => {
//                 controls_updated = true;
//                 current_controls.down = true;
//             },
//             KeyCode::Left => {
//                 controls_updated = true;
//                 current_controls.left = true;
//             },
//             KeyCode::Right => {
//                 controls_updated = true;
//                 current_controls.right = true;
//             },
//             KeyCode::Z => {
//                 controls_updated = true;
//                 current_controls.b = true;
//             },
//             KeyCode::X => {
//                 controls_updated = true;
//                 current_controls.a = true;
//             },
//             KeyCode::Return => {
//                 controls_updated = true;
//                 current_controls.start = true;
//             },
//             KeyCode::RShift => {
//                 controls_updated = true;
//                 current_controls.select = true;
//             },
//             _ => {}
//         }

//         if controls_updated {
//             self.update_controls(current_controls);
//         }
//     }

//     fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
//         let mut controls_updated = false;
//         let mut current_controls = self.get_controls();
//         match keycode {
//             KeyCode::Up => {
//                 controls_updated = true;
//                 current_controls.up = false;
//             },
//             KeyCode::Down => {
//                 controls_updated = true;
//                 current_controls.down = false;
//             },
//             KeyCode::Left => {
//                 controls_updated = true;
//                 current_controls.left = false;
//             },
//             KeyCode::Right => {
//                 controls_updated = true;
//                 current_controls.right = false;
//             },
//             KeyCode::Z => {
//                 controls_updated = true;
//                 current_controls.b = false;
//             },
//             KeyCode::X => {
//                 controls_updated = true;
//                 current_controls.a = false;
//             },
//             KeyCode::Return => {
//                 controls_updated = true;
//                 current_controls.start = false;
//             },
//             KeyCode::RShift => {
//                 controls_updated = true;
//                 current_controls.select = false;
//             },
//             _ => {}
//         }

//         if controls_updated {
//             self.update_controls(current_controls);
//         }
//     }
// }