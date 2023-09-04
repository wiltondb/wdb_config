use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use nwg::stretch::style::AlignSelf;
use nwg::stretch::style::JustifyContent;
use nwg::stretch::style::FlexDirection;

use crate::*;
use dialogs::DialogUi;
use dialogs::PopupDialog;
use connect_check_dialog::ConnectCheckDialog;
use nwg::Window;

#[derive(Default)]
pub struct ConnectCheckDialogUi {
    events: events::Events<ConnectCheckDialog>,
    font_normal: nwg::Font,

    window: nwg::Window,
    pub progress_bar: nwg::ProgressBar,
    pub label: nwg::Label,
    details_box: nwg::TextBox,
    copy_clipboard_button: nwg::Button,
    close_button: nwg::Button,

    root_layout: nwg::FlexboxLayout,
    buttons_layout: nwg::FlexboxLayout,

    check_notice: notice::SyncNoticeValue<String>,
}

impl ConnectCheckDialogUi {
    pub fn check_notice(&self) -> &notice::SyncNoticeValue<String> {
       &self.check_notice
    }

    pub fn set_label_text(&self, text: &str) {
        self.label.set_text(text);
    }

    pub fn details_text(&self) -> String {
        self.details_box.text()
    }

    pub fn set_details_text(&self, text: &str) {
        self.details_box.set_text(text)
    }
}

impl DialogUi for ConnectCheckDialogUi {
    fn window(&self) -> &Window {
        &self.window
    }

    fn build_controls(&mut self) -> Result<(), nwg::NwgError> {
        nwg::Font::builder()
            .size(ui::font_size_builder()
                .normal()
                .build())
            .build(&mut self.font_normal)?;

        nwg::Window::builder()
            .size((320, 200))
            .center(true)
            .title("Check")
            .build(&mut self.window)?;
        events::builder()
            .control(&self.window)
            .event(nwg::Event::OnWindowClose)
            .handler(ConnectCheckDialog::close)
            .build(&mut self.events)?;

        nwg::ProgressBar::builder()
            .flags(nwg::ProgressBarFlags::VISIBLE | nwg::ProgressBarFlags::MARQUEE)
            .marquee(true)
            .marquee_update(30)
            .range(0..1)
            .parent(&self.window)
            .build(&mut self.progress_bar)?;

        nwg::Label::builder()
            .text("Checking ...")
            .flags(nwg::LabelFlags::VISIBLE | nwg::LabelFlags::ELIPSIS)
            .font(Some(&self.font_normal))
            .v_align(nwg::VTextAlign::Top)
            //.background_color(Some([42 as u8, 42 as u8, 42 as u8]))
            .parent(&self.window)
            .build(&mut self.label)?;

        nwg::TextBox::builder()
            .text("Details pending ...")
            .font(Some(&self.font_normal))
            .readonly(true)
            .parent(&self.window)
            .build(&mut self.details_box)?;

        nwg::Button::builder()
            .text("Copy to clipboard")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.copy_clipboard_button)?;
        events::builder()
            .control(&self.copy_clipboard_button)
            .event(nwg::Event::OnButtonClick)
            .handler(ConnectCheckDialog::copy_to_clipboard)
            .build(&mut self.events)?;

        nwg::Button::builder()
            .text("Close")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;
        events::builder()
            .control(&self.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(ConnectCheckDialog::close)
            .build(&mut self.events)?;

        notice::builder()
            .parent(&self.window)
            .build(&mut self.check_notice)?;
        events::builder()
            .control(&self.check_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(ConnectCheckDialog::on_connection_check_complete)
            .build(&mut self.events)?;

        Ok(())
    }

    fn build_layout(&mut self) -> Result<(), nwg::NwgError> {

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .justify_content(JustifyContent::FlexEnd)
            .auto_spacing(None)

            .child(&self.copy_clipboard_button)
            .child_size(ui::size_builder()
                .width_button_xwide()
                .height_button()
                .build())

            .child(&self.close_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())

            .build_partial(&self.buttons_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Column)

            .child(&self.progress_bar)
            .child_size(ui::size_builder()
                .height_pt(30)
                .width_auto()
                .build())
            .child_align_self(AlignSelf::Stretch)

            .child(&self.label)
            .child_size(ui::size_builder()
                .height_pt(10)
                .width_auto()
                .build())
            .child_align_self(AlignSelf::Stretch)

            .child(&self.details_box)
            .child_size(ui::size_builder()
                .height_auto()
                .width_auto()
                .build())
            .child_align_self(AlignSelf::Stretch)
            .child_flex_grow(1.0)

            .child_layout(&self.buttons_layout)
            .child_align_self(AlignSelf::Stretch)

            .build(&mut self.root_layout)?;

        Ok(())
    }
}

pub struct ConnectCheckDialogNwg {
    pub inner: Rc<ConnectCheckDialog>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl nwg::NativeUi<ConnectCheckDialogNwg> for ConnectCheckDialog {
    fn build_ui(mut data: ConnectCheckDialog) -> Result<ConnectCheckDialogNwg, nwg::NwgError> {
        data.ui.build_controls()?;
        data.ui.build_layout()?;
        data.ui.shake_after_layout();

        let wrapper = ConnectCheckDialogNwg {
            inner:  Rc::new(data),
            default_handler: Default::default(),
        };

        let data_ref = Rc::downgrade(&wrapper.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(evt_data) = data_ref.upgrade() {
                for eh in evt_data.ui.events.iter() {
                    if handle == eh.control_handle && evt == eh.event {
                        (eh.handler)(&evt_data);
                        break;
                    }
                }
            }
        };

        *wrapper.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&wrapper.ui.window.handle, handle_events));

        return Ok(wrapper);
    }
}

impl Drop for ConnectCheckDialogNwg {
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for ConnectCheckDialogNwg {
    type Target = ConnectCheckDialog;

    fn deref(&self) -> &ConnectCheckDialog {
        &self.inner
    }
}
