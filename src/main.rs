extern crate gl;

mod window;

fn main() {
    let mut window = window::Window::new(800, 600, "olive - music player");
    gl::load_with(|s| window.get_proc_address(s));

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        window.poll_events();
    }
}
