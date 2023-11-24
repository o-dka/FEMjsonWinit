use stlviewerwinit::run;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap(); 
    window.set_title("Треугольник");
    env_logger::init();
    pollster::block_on( run(event_loop, &window));    
}
