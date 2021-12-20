use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind};

pub fn back_inline_keyboard_button(kind: InlineKeyboardButtonKind) -> InlineKeyboardButton {
    const BACK_SYMBOL: char = '\u{2b05}';
    InlineKeyboardButton::new(format!("{} Back", BACK_SYMBOL), kind)
}
