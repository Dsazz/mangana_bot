use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind};

pub fn checkbox_inline_keyboard_button(title: &str, checked: bool, kind: InlineKeyboardButtonKind) -> InlineKeyboardButton {
    const CHECKED_SYMBOL: char = '\u{2b50}';
    const UNCHECKED_SYMBOL: char = '\u{2606}';
    InlineKeyboardButton::new(
        format!("{} {}", if checked { CHECKED_SYMBOL } else { UNCHECKED_SYMBOL }, title),
        kind
    )
}
