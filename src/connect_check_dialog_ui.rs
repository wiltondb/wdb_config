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

    root_layout: nwg::FlexboxLayout,
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
            .range(0..1)
            .parent(&self.window)
            .build(&mut self.progress_bar)?;

        nwg::Label::builder()
            .text("Checking ...")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.label)?;


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
            .flex_direction(FlexDirection::Column)

            .child(&self.progress_bar)
            .child_size(ui::size_builder()
                .height_points(50)
                .width_auto()
                .build())
            .child_align_self(AlignSelf::Stretch)

            .child(&self.label)
            .child_size(ui::size_builder()
                .height_points(50)
                .width_auto()
                .build())
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
