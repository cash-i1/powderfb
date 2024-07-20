use crate::misc::{Rectangle, WindowDimensions};
use minifb::*;

pub struct Graphics {
    pub buffer: Vec<u32>,
    pub dimensions: WindowDimensions,
    pub window: Window,
}

impl Graphics {
    pub fn builder() -> GraphicsBuilder {
        GraphicsBuilder::new()
    }
    pub fn rectangle(&mut self, rect: Rectangle) {
        for i in 0..rect.width {
            for j in 0..rect.height {
                let (new_x, new_y) = (i + rect.x, j + rect.y);
                let new = new_y * self.dimensions.width + new_x;
                if let Some(buf) = self.buffer.get_mut(new) {
                    *buf = rect.color;
                }
            }
        }
    }

    pub fn pixel(&mut self, rect: Rectangle) {
        self.buffer[rect.y * self.dimensions.width + rect.x] = rect.color;
    }
    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.dimensions.width, self.dimensions.height)
            .unwrap();

        self.buffer.fill(0x000000); // i cant believe this was the fix :facepalm:
    }
}

pub struct GraphicsBuilder {
    dimensions: Option<WindowDimensions>,
    fps: Option<usize>,
    resizable: bool,
}
impl GraphicsBuilder {
    pub fn new() -> Self {
        Self {
            resizable: false,
            dimensions: None,
            fps: None,
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
    pub fn resizeable(&mut self, resizable: bool) -> &mut Self {
        self.resizable = resizable;
        self
    }

    pub fn build(&mut self) -> Graphics {
        let dimensions = self.dimensions.take().unwrap();
        let buffer = vec![0u32; dimensions.width * dimensions.height];

        let mut window_opts = WindowOptions::default();

        window_opts.resize = self.resizable;

        let mut window = Window::new(
            "window",
            dimensions.width,
            dimensions.height,
            window_opts,
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
