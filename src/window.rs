extern crate glfw;
use glfw::{Action, Context, Key, WindowEvent, Glfw, WindowMode};

pub struct Window {
    pub glfw: Glfw,
    pub window: glfw::Window,
    pub events: std::sync::mpsc::Receiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("failed to init glfw");
        let (mut window, events) = glfw
            .create_window(width, height, title, WindowMode::Windowed)
            .expect("failed to create window");

        window.make_current();
        window.set_key_polling(true);

        Self { glfw, window, events }
    }

    pub fn get_proc_address(&mut self, s: &str) -> *const std::ffi::c_void {
        self.window.get_proc_address(s)
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();

        /* hard coded exit */
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }

                _ => {}
            }
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}
