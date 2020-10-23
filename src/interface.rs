use super::{Event, EventLoopProxy, CustomEvent};
use crate::field::Field;
use super::{WINDOW_WIDTH, WINDOW_HEIGHT};

#[derive(Debug)]
pub enum InterfaceEvent {
    RevealCell(u8, u8),
    ToggleFlagCell(u8, u8),
}

#[derive(Debug)]
pub struct Interface {
    event_loop_proxy: EventLoopProxy,

    cursor_position: (u32, u32),
}

impl Interface {
    pub fn new(event_loop_proxy: EventLoopProxy) -> Self {
        Self {
            event_loop_proxy,

            cursor_position: (0, 0),
        }
    }

    pub fn on_event(&mut self, event: &Event, field: &Field) {
        use winit::event::WindowEvent::*;

        if let winit::event::Event::WindowEvent { event, .. } = event {
            match event {
                CursorMoved { position, .. } => {
                    self.cursor_position = (position.x as u32, position.y as u32);
                }

                MouseInput { state, button, .. } =>
                    if let winit::event::ElementState::Pressed = state {
                        use winit::event::MouseButton;

                        let (x, y) = (
                            (self.cursor_position.0 * field.width() as u32 / WINDOW_WIDTH) as u8,
                            (self.cursor_position.1 * field.height() as u32 / WINDOW_HEIGHT) as u8,
                        );

                        match button {
                            MouseButton::Left =>
                                self.event_loop_proxy.send_event(
                                    CustomEvent::InterfaceEvent(
                                        InterfaceEvent::RevealCell(x, y))).unwrap(),
                            MouseButton::Right =>
                                self.event_loop_proxy.send_event(
                                    CustomEvent::InterfaceEvent(
                                        InterfaceEvent::ToggleFlagCell(x, y))).unwrap(),

                            _ => (),
                        }
                    }

                _ => (),
            };
        }
    }
}
