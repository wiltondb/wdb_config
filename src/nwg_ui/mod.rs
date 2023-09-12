
mod controls;
mod events;
mod font_size_builder;
mod layout;
mod margin_builder;
mod message_box;
mod popup_args;
mod popup_dialog;
mod popup_join_handle;
mod size_builder;
mod sync_notice;

pub use nwg::stretch::style::AlignSelf;
pub use nwg::stretch::style::JustifyContent;
pub use nwg::stretch::style::FlexDirection;

pub use controls::Controls;
pub use events::Event;
pub use events::Events;
pub use layout::Layout;
pub use message_box::message_box;
pub use message_box::message_box_debug;
pub use popup_args::PopupArgs;
pub use popup_dialog::PopupDialog;
pub use popup_join_handle::PopupJoinHandle;
pub use sync_notice::SyncNotice;
pub use sync_notice::SyncNoticeSender;

use events::EventBuilder;
use font_size_builder::FontSizeBuilder;
use margin_builder::MarginBuilder;
use size_builder::SizeBuilder;
use sync_notice::SyncNoticeBuilder;

pub fn size_builder() -> SizeBuilder {
    Default::default()
}

pub fn font_size_builder() -> FontSizeBuilder {
    Default::default()
}

pub fn margin_builder() -> MarginBuilder {
    Default::default()
}

pub fn event_builder<W>() -> EventBuilder<W> {
    EventBuilder::new()
}

pub fn notice_builder() -> SyncNoticeBuilder {
    SyncNoticeBuilder::new()
}

pub fn shake_window(window: &nwg::Window) {
    // workaround for garbled text
    let (wx, wy) = window.size();
    window.set_size(wx + 1, wy + 1);
    window.set_size(wx, wy);
}

pub fn wrap_label_text(text: &str, limit_chars: usize) -> String {
    if text.len() < limit_chars {
        return text.to_string();
    }
    let mut res = String::new();
    let mut count = 0;
    for ch in text.chars() {
        if count >= limit_chars && ' ' == ch {
            res.push('\r');
            res.push('\n');
            count = 0;
        } else {
            res.push(ch);
        }
        count += 1;
    }
    res
}
