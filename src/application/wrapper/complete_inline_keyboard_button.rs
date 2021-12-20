use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind};

pub fn complete_inline_keyboard_button(kind: InlineKeyboardButtonKind) -> InlineKeyboardButton {
    const COMPLETE_SYMBOL: char = '\u{1f3c1}';
    InlineKeyboardButton::new(
        format!("{} Завершить", COMPLETE_SYMBOL),
        kind,
    )
}
