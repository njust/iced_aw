//! Widgets for iced_web

#[cfg(feature = "badge")]
pub mod badge;
#[cfg(feature = "badge")]
pub use badge::Badge;

#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "card")]
pub use card::Card;

#[cfg(feature = "color_picker")]
pub mod color_picker;
#[cfg(feature = "color_picker")]
pub use color_picker::ColorPicker;

#[cfg(feature = "date_picker")]
pub mod date_picker;
#[cfg(feature = "date_picker")]
pub use date_picker::DatePicker;

#[cfg(feature = "floating_button")]
pub mod floating_button;
#[cfg(feature = "floating_button")]
pub use floating_button::FloatingButton;

#[cfg(feature = "modal")]
pub mod modal;
#[cfg(feature = "modal")]
pub use modal::Modal;

#[cfg(feature = "time_picker")]
pub mod time_picker;
#[cfg(feature = "time_picker")]
pub use time_picker::TimePicker;
