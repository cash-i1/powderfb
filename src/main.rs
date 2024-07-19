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
    fn builder() -> GraphicsBuilder {
        GraphicsBuilder::new()
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

struct GraphicsBuilder {
    buffer: Option<Vec<u32>>,
    dimensions: Option<WindowDimensions>,
    fps: Option<usize>
}
impl GraphicsBuilder {
    pub fn new() -> Self {
        Self {
            buffer: None,
            dimensions: None,
            fps: None
        }
    }
    pub fn width(&mut self, width: usize) -> &mut Self {
        if let Some(dims) = &mut self.dimensions {
            dims.width = width;
        } else {
            self.dimensions = Some(WindowDimensions { width, height: 0 })
        }
        self
    }
    pub fn height(&mut self, height: usize) -> &mut Self {
        if let Some(dims) = &mut self.dimensions {
            dims.height = height;
        } else {
            self.dimensions = Some(WindowDimensions { width: 0, height })
        }
        self
    }
    pub fn fps(&mut self, fps: usize) -> &mut Self {
        self.fps = Some(fps);
        self
    }
    pub fn build(&mut self) -> Graphics {
        let buffer = self.buffer.take().unwrap();
        let dimensions = self.dimensions.take().unwrap();

        let mut window = Window::new(
            "window",
            dimensions.width,
            dimensions.height,
            WindowOptions {
                ..WindowOptions::default()
            },
        )
        .expect("unable to create the window");

        if let Some(fps) = self.fps {
            window.set_target_fps(fps);
        }

        Graphics {
            buffer,
            dimensions,
            window,
        }
    }
}

fn main() {
    let mut gfx = Graphics::builder()
        .width(1280)
        .height(720)
        .fps(60)
        .build();

    while gfx.window.is_open() {
        gfx.rectangle((100, 100), (20, 20));
        gfx.pixel((10, 10));
        gfx.update();
    }
}
