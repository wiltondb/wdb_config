use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use nwg::stretch::style::AlignSelf;
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
    close_button: nwg::Button,

    root_layout: nwg::FlexboxLayout,
    label_layout: nwg::FlexboxLayout,
    pub check_notice: notice::SyncNoticeValue<bool>,
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
            .size((320, 140))
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
            .h_align(nwg::HTextAlign::Center)
            .v_align(nwg::VTextAlign::Top)
            .font(Some(&self.font_normal))
            //.background_color(Some([42 as u8, 42 as u8, 42 as u8]))
            .parent(&self.window)
            .build(&mut self.label)?;

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
            .child(&self.label)
            .child_size(ui::size_builder()
                .height_pt(30)
                .width_auto()
                .build())
            .child_align_self(AlignSelf::Stretch)
            .child_margin(ui::margin_builder()
                .top_pt(10)
                .build())
            .child_flex_grow(1.0)
            .build_partial(&mut self.label_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Column)

            .child(&self.progress_bar)
            .child_size(ui::size_builder()
                .height_pt(30)
                .width_auto()
                .build())
            .child_align_self(AlignSelf::Stretch)

            .child_layout(&self.label_layout)
            .child_flex_grow(1.0)

            .child(&self.close_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_align_self(AlignSelf::FlexEnd)

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
