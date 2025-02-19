use algebra::vec2::{i16::I16Vec2, u16::U16Vec2};
use crossterm::event::{Event as CrosstermEvent, MouseButton, MouseEventKind};
use utils::key::Key;

pub enum Event {
    Quit,
    Click(U16Vec2),
    Scroll(I16Vec2),
    Input(Key),
}

#[derive(Debug, Default)]
pub struct EventManager {}

impl EventManager {
    pub fn get_event(&mut self, evt: CrosstermEvent) -> Option<Event> {
        match evt {
            CrosstermEvent::Key(evt) => {
                let key = Key::from(evt);
                return Some(Event::Input(key));
            }
            CrosstermEvent::Mouse(evt) => match evt.kind {
                MouseEventKind::ScrollUp => return Some(Event::Scroll(I16Vec2::up())),
                MouseEventKind::ScrollDown => return Some(Event::Scroll(I16Vec2::down())),
                MouseEventKind::ScrollLeft => return Some(Event::Scroll(I16Vec2::left())),
                MouseEventKind::ScrollRight => return Some(Event::Scroll(I16Vec2::right())),
                MouseEventKind::Down(btn) => {
                    if btn == MouseButton::Left {
                        return Some(Event::Click(U16Vec2::new(evt.column, evt.row)));
                    }
                }
                _ => {}
            },
            _ => {}
        };

        None
    }
}
