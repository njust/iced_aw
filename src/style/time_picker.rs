//! TODO
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};
#[cfg(target_arch = "wasm32")]
use iced_web::{Background, Color};

/// TODO
#[allow(missing_debug_implementations)]
pub struct Style {
    /// TODO
    pub background: Background,

    /// TODO
    pub border_radius: f32,

    /// TODO
    pub border_width: f32,

    /// TODO
    pub border_color: Color,

    /// TODO
    pub text_color: Color,

    /// TODO
    pub clock_number_color: Color,

    /// TODO
    pub clock_number_background: Color,

    /// TODO
    pub clock_dots_color: Color,

    /// TODO
    pub clock_hand_color: Color,

    /// TODO
    pub clock_hand_width: f32,
}

/// The appearance of a [`DatePicker`](crate::native::DatePicker).
pub trait StyleSheet {
    /// The normal appearance of a [`DatePicker`](crate::native::DatePicker).
    fn active(&self) -> Style;

    /// TODO
    fn hovered(&self) -> Style;

    /// TODO
    fn selected(&self) -> Style;
}

/// TODO
#[derive(Clone, Debug)]
pub struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            background: Color::WHITE.into(),
            border_radius: 15.0,
            border_width: 1.0,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
            clock_number_color: Color::BLACK,
            clock_number_background: Color::WHITE,
            clock_dots_color: [0.87, 0.87, 0.87].into(),
            clock_hand_color: [0.87, 0.87, 0.87].into(),
            clock_hand_width: 1.0,
        }
    }

    fn hovered(&self) -> Style {
        Style {
            clock_number_background: [0.87, 0.87, 0.87].into(),
            .. self.active()
        }
    }

    fn selected(&self) -> Style {
        Style {
            clock_number_background: [0.87, 0.87, 0.87].into(),
            .. self.active()
        }
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}