mod core;

fn main() {
    let mut window = core::window::Window::new(500, 500, "hello");
    window.limit_fps(16600);

    while window.should_close() {
        for x in 0..window.width {
            for y in 0..window.height {
                window.put_pixel(x, y, 0x5A3C5A);
            }
        }
        window.update();
    }
}