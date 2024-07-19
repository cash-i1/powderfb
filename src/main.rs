use minifb::*;

struct WindowDimensions {
    width: usize,
    height: usize,
}

struct Graphics {
    buffer: Vec<u32>,
    dimensions: WindowDimensions,
    window: Window,
}

impl Graphics {
    fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0u32; width * height];

        let window = Window::new(
            "window",
            width,
            height,
            WindowOptions {
                ..WindowOptions::default()
            },
        )
        .expect("Unable to create the window");

        Self {
            buffer,
            dimensions: WindowDimensions {width, height},
            window,
        }
    }
    fn rectangle(&mut self, (x, y): (usize, usize), (width, height): (usize, usize)) {
        for i in 0..width {
            for j in 0..height {
                let (new_x, new_y) = (i + x, j + y);
                let new = new_x * self.dimensions.width + new_y;
                self.buffer[new] = 99999;
            }
        }
    }

    fn pixel(&mut self, (x, y): (usize, usize)) {
        self.buffer[y * self.dimensions.width + x] = 1111;
    }
    fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.dimensions.width, self.dimensions.height)
            .unwrap();
    }
}

fn main() {
    let mut gfx = Graphics::new(1280, 720);
    gfx.window.set_target_fps(60);
    while gfx.window.is_open() {
        gfx.rectangle((100, 100), (20, 20));
        gfx.pixel((10, 10));
        gfx.update();
    }
}
