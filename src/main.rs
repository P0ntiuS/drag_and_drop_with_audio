use std::io::BufReader;
use std::thread;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {

    let mut children = vec![];

    children.push(thread::spawn(move || {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let file = std::fs::File::open("music.mp3").unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

        sink.sleep_until_end();
    }));
    
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::WindowEvent {event, .. } => match event 
            {
                WindowEvent::DroppedFile (ref path) => println!("{:?}", path),
                _ => println!("None"),
            },
            _ => (),
        }
    });
}