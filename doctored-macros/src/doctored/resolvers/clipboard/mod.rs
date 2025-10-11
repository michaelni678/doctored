pub mod copy;
pub mod paste;

#[derive(Clone)]
pub enum ClipboardModifier {
    Strip,
    StripLeft(Option<String>),
    StripRight(Option<String>),
    PushLeft(String),
    PushRight(String),
}

pub fn apply_clipboard_modifiers(modifiers: &[ClipboardModifier], string: &mut String) {
    for modifier in modifiers {
        match modifier {
            ClipboardModifier::Strip => *string = string.trim().to_owned(),
            ClipboardModifier::StripLeft(strip) => match strip {
                Some(prefix) => {
                    if let Some(stripped) = string.strip_prefix(prefix) {
                        *string = stripped.to_owned();
                    }
                }
                None => {
                    *string = string.trim_start().to_owned();
                }
            },
            ClipboardModifier::StripRight(strip) => match strip {
                Some(suffix) => {
                    if let Some(stripped) = string.strip_suffix(suffix) {
                        *string = stripped.to_owned();
                    }
                }
                None => {
                    *string = string.trim_start().to_owned();
                }
            },
            ClipboardModifier::PushLeft(push) => string.insert_str(0, push),
            ClipboardModifier::PushRight(push) => string.push_str(push),
        }
    }
}
