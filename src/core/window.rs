use minifb;

pub struct Window
{
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
    window: minifb::Window,
}

impl Window
{

    pub fn new(width: usize, height: usize, title: &str) -> Window
    {
        let mut window = Window
        {
            width,
            height,
            buffer: vec![0; width * height],
            window: minifb::Window::new
                (
                    title,
                    width,
                    height,
                    minifb::WindowOptions::default(),
                )
                .unwrap_or_else(|e| { panic!("{}", e); }),
        };
        window
    }

    pub fn update(&mut self)
    {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32)
    {
        let index = y * self.width + x;
        self.buffer[index] = color;
    }

    pub fn should_close(&self) -> bool
    {
        self.window.is_open()
    }

    pub fn limit_fps(&mut self, fps: u64)
    {
        self.window.limit_update_rate(Some(std::time::Duration::from_micros(fps)));
    }
}