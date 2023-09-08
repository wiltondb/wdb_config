
mod controls;
mod events;
mod font_size_builder;
mod layout;
mod margin_builder;
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
