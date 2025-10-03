pub mod copy;
pub mod paste;

#[derive(Clone)]
pub enum ClipboardModifier {
    Strip,
    LeftStrip(Option<String>),
    LeftPush(String),
}

pub fn apply_clipboard_modifiers(modifiers: &[ClipboardModifier], string: &mut String) {
    for modifier in modifiers {
        match modifier {
            ClipboardModifier::Strip => *string = string.trim().to_owned(),
            ClipboardModifier::LeftStrip(strip) => match strip {
                Some(prefix) => {
                    if let Some(stripped) = string.strip_prefix(prefix) {
                        *string = stripped.to_owned();
                    }
                }
                None => {
                    *string = string.trim_start().to_owned();
                }
            },
            ClipboardModifier::LeftPush(push) => string.insert_str(0, push),
        }
    }
}
