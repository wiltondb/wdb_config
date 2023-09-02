use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use nwg::stretch::style::AlignSelf;
use nwg::stretch::style::FlexDirection;

use crate::*;
use dialogs::DialogUi;
use dialogs::PopupDialog;
use connect_dialog::ConnectDialog;
use nwg::Window;

#[derive(Default)]
pub struct ConnectDialogUi {
    events: events::Events<ConnectDialog>,
    font_normal: nwg::Font,

    window: nwg::Window,
    test_button: nwg::Button,

    root_layout: nwg::FlexboxLayout,

    pub check_dialog_notice: notice::SyncNotice,
}

impl DialogUi for ConnectDialogUi {
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
            .size((480, 320))
            .center(true)
            .title("Connect")
            .build(&mut self.window)?;
        events::builder()
            .control(&self.window)
            .event(nwg::Event::OnWindowClose)
            .handler(ConnectDialog::close)
            .build(&mut self.events)?;

        nwg::Button::builder()
            .text("Test")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.test_button)?;
        events::builder()
            .control(&self.test_button)
            .event(nwg::Event::OnButtonClick)
            .handler(ConnectDialog::open_check_dialog)
            .build(&mut self.events)?;

        notice::builder()
            .parent(&self.window)
            .build(&mut self.check_dialog_notice)?;
        events::builder()
            .control(&self.check_dialog_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(ConnectDialog::await_check_dialog)
            .build(&mut self.events)?;

        Ok(())
    }

    fn build_layout(&mut self) -> Result<(), nwg::NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Column)

            .child(&self.test_button)
            .child_size(ui::size_builder()
                .height_button()
                .width_button_normal()
                .build())
            .child_align_self(AlignSelf::FlexEnd)

            .build(&mut self.root_layout)?;

        Ok(())
    }

}

pub struct ConnectDialogNwg {
    inner: Rc<ConnectDialog>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl nwg::NativeUi<ConnectDialogNwg> for ConnectDialog {
    fn build_ui(mut data: ConnectDialog) -> Result<ConnectDialogNwg, nwg::NwgError> {
        data.ui.build_controls()?;
        data.ui.build_layout()?;
        data.ui.shake_after_layout();

        let wrapper = ConnectDialogNwg {
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

impl Drop for ConnectDialogNwg {
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for ConnectDialogNwg {
    type Target = ConnectDialog;

    fn deref(&self) -> &ConnectDialog {
        &self.inner
    }
}
