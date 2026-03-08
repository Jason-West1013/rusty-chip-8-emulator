use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent};

mod cpu;
mod display;

use cpu::Cpu;
use display::Display;

fn main() {
    let event_loop = EventLoop::new();
    let mut display = Display::new(&event_loop);
    let mut cpu = Cpu::new();

    let rom = include_bytes!("../roms/ibm-logo.ch8");
    cpu.memory[0x200..0x200 + rom.len()].copy_from_slice(rom);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::MainEventsCleared => {
                let opcode = cpu.fetch();
                cpu.execute(opcode);
                display.draw(&cpu.display);
            }
            _ => {}
        }
    });
}
