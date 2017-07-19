
extern crate love2d;
use love2d::{Window, Event};
use love2d::{ElementState, VirtualKeyCode};

use std::collections::HashMap;

fn main() {
    let mut app = Window::new("Hello World", 640, 480);
    let mut keymap: HashMap<VirtualKeyCode, bool> = HashMap::new();

    'main: loop {
        {
            let mut frame = app.next_frame();
            frame.clear();

            // let delta = frame.delta();

            frame.set_color_html("312");
            frame.draw_rect((-0.5, -0.5), (0.5, 0.5));

            frame.set_color_html("#033112");
            frame.draw_rect((0.0, 0.0), (-1.0, -1.0));

            frame.set_color(1.0, 0.0, 0.0, 1.0);
            frame.draw_line(0.0, 0.0, 1.0, 1.0);

            frame.set_color(0.0, 0.0, 1.0, 0.3);
            frame.draw(&[(0.0, 0.0), (1.0, 1.0), (1.0, 0.0)]);

            frame.set_color(0.0, 1.0, 0.0, 0.3);
            frame.draw(&[(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)]);

            frame.set_color(1.0, 0.0, 0.0, 0.3);
            frame.draw_circle(0.25, -0.25, 0.75, 0.25, 10);

            frame.finish();
        }

        for ev in app.poll_events() {
            match ev {
                Event::Closed => break 'main,
                Event::KeyboardInput(ElementState::Pressed, Some(key)) => {
                    match key {
                        VirtualKeyCode::Space => println!("Space!"),
                        VirtualKeyCode::Escape => break 'main,
                        _ => (),
                    };
                    keymap.insert(key, true);
                }
                Event::KeyboardInput(ElementState::Released, Some(key)) => {
                    keymap.insert(key, false);
                }
                _ => (),
            }
        }
    }
}
