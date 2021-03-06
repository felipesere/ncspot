use cursive::align::HAlign;
use cursive::direction::Direction;
use cursive::event::{Callback, Event, EventResult, EventTrigger};
use cursive::theme::ColorStyle;
use cursive::traits::View;
use cursive::vec::Vec2;
use cursive::Cursive;
use cursive::Printer;
use unicode_width::UnicodeWidthStr;

pub struct SplitButton {
    callbacks: Vec<(EventTrigger, Callback)>,

    left: String,
    right: String,

    enabled: bool,
    last_size: Vec2,
    invalidated: bool,
}

impl SplitButton {
    pub fn new(left: &str, right: &str) -> SplitButton {
        SplitButton {
            callbacks: Vec::new(),
            left: left.to_owned(),
            right: right.to_owned(),
            enabled: true,
            last_size: Vec2::zero(),
            invalidated: true,
        }
    }

    pub fn add_callback<F, E>(&mut self, trigger: E, cb: F)
    where
        E: Into<EventTrigger>,
        F: 'static + Fn(&mut Cursive),
    {
        self.callbacks.push((trigger.into(), Callback::from_fn(cb)));
    }
}

// This is heavily based on Cursive's Button implementation with minor
// modifications to print the track's duration at the right screen border
impl View for SplitButton {
    fn draw(&self, printer: &Printer<'_, '_>) {
        if printer.size.x == 0 {
            return;
        }

        let style = if !(self.enabled && printer.enabled) {
            ColorStyle::secondary()
        } else if !printer.focused {
            ColorStyle::primary()
        } else {
            ColorStyle::highlight()
        };

        // shorten titles that are too long and append ".." to indicate this
        let mut left_shortened = self.left.clone();
        left_shortened.truncate(printer.size.x - self.right.width() - 1);
        if left_shortened.width() < self.left.width() {
            let offset = left_shortened.width() - 2;
            left_shortened.replace_range(offset.., "..");
        }

        printer.with_color(style, |printer| {
            printer.print((0, 0), &left_shortened);
        });

        // track duration goes to the end of the line
        let offset = HAlign::Right.get_offset(self.right.width(), printer.size.x);

        printer.with_color(style, |printer| {
            printer.print((offset, 0), &self.right);
        });
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        for (trigger, callback) in self.callbacks.iter() {
            if trigger.apply(&event) {
                return EventResult::Consumed(Some(callback.clone()));
            }
        }
        EventResult::Ignored
    }

    fn layout(&mut self, size: Vec2) {
        self.last_size = size;
        self.invalidated = false;
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        // we always want the full width
        Vec2::new(constraint.x, 1)
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        self.enabled
    }

    fn needs_relayout(&self) -> bool {
        self.invalidated
    }
}
