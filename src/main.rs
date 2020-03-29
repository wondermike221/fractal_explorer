extern crate minifb;
extern crate rayon;
mod complex;
mod coords;
mod corners;
mod fractals;
mod utils;

use minifb::{Key, MouseButton, MouseMode, CursorStyle, Scale, ScaleMode, Window, WindowOptions};
pub use crate::complex::Complex;
pub use crate::coords::Coords;
pub use crate::corners::Corners;
pub use crate::fractals::mandelbrot::Mandelbrot;
use std::collections::VecDeque;


const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 800;
const GENERATION_INFINITY: u32 = 256;



fn main() {
    let mut buffer: Vec<u32>;

    let mut window = Window::new(
        "Fractal Explorer - ESC to exit",
        DEFAULT_WIDTH,
        DEFAULT_HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X1,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to Open Window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    window.set_cursor_style(CursorStyle::Crosshair);

    let mut x_min = -2f64;
    let mut y_min = -1.5f64;

    let mut x_max = 1f64;
    let mut y_max = 1.5f64;

    let default_corners = Corners {
        first: Some(Coords{x: x_min, y: y_min}),
        second: Some(Coords{x: x_max, y: y_max})
    };

    let mandel = Mandelbrot { domain_range: default_corners };
    buffer = mandel.generate_fractal(default_corners);

    // We unwrap here as we want this code to exit if it fails
    window.update_with_buffer(&buffer, DEFAULT_WIDTH, DEFAULT_HEIGHT).unwrap();
    let mut left_mouse_state: MouseState = MouseState::MouseUp;
    let mut corners_history: VecDeque<Corners> = VecDeque::with_capacity(15);
    let mut corners: Corners = Corners {
        first: None,
        second: None
    };
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
            // let screen_pos = ((y as usize) * (WIDTH)) + x as usize;

            let real = utils::interpolate(x as f64, 0., DEFAULT_WIDTH as f64, x_min, x_max);
            let imag = utils::interpolate(y as f64, 0., DEFAULT_HEIGHT as f64, y_min, y_max);

            if window.get_mouse_down(MouseButton::Left) {
                if left_mouse_state == MouseState::MouseUp {
                    corners.first = Some(Coords{x:real, y:imag});
                }
                left_mouse_state = MouseState::MouseDown;
            } else {
                if left_mouse_state == MouseState::MouseDown {
                    left_mouse_state = MouseState::MouseUp;
                    if corners_history.len() <= 15 {
                        corners.second = Some(Coords{x:real, y:imag});
                        let unpacked_corners = corners.unpack();
                        x_min = unpacked_corners.0;
                        y_min = unpacked_corners.1;
                        x_max = unpacked_corners.2;
                        y_max = unpacked_corners.3;
                        buffer = mandel.generate_fractal(corners);

                        corners_history.push_front(corners);
                        window.update_with_buffer(&buffer, DEFAULT_WIDTH, DEFAULT_HEIGHT).unwrap();
                    }
                }
            }

            if window.get_mouse_down(MouseButton::Right) {
                println!("(x: {}, y: {} -> r: {}, i: {}", x, y, real, imag);
            }

            if window.get_mouse_down(MouseButton::Middle) {
                corners_history.clear();
                let unpacked_corners = default_corners.unpack();
                x_min = unpacked_corners.0;
                y_min = unpacked_corners.1;
                x_max = unpacked_corners.2;
                y_max = unpacked_corners.3;
                buffer = mandel.generate_fractal(default_corners);
                window.update_with_buffer(&buffer, DEFAULT_WIDTH, DEFAULT_HEIGHT).unwrap();
            }

        });

        window.update();
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum MouseState {
    MouseDown,
    MouseUp
}



fn fill(n: u32) -> u32 {
    if GENERATION_INFINITY == n {
        return 0x00;
    } else {
        return n * 32 % 255;
    }
}

