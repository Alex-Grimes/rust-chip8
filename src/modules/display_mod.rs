use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use CHIP8_HEIGHT;
use CHIP8_WIDTH;

const SCALE: u32 = 20;
const WIDTH: u32 = CHIP8_WIDTH as u32 * SCALE;
const HEIGHT: u32 = CHIP8_HEIGHT as u32 * SCALE;

pub struct DisplayModule {
    canvas: Canvas<Window>,
}

impl DisplayModule {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("CHIP-8", WIDTH, HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        DisplayModule { canvas: canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = x as u32 * SCALE;
                let y = y as u32 * SCALE;

                self.canvas.set_draw_color(color(col));
                let _ = self
                    .canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE, SCALE));
            }
        }
        self.canvas.present();
    }
}
fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(255, 255, 255)
    }
}
