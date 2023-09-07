
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use ui::Events;
use ui::Layout;
use super::*;

pub(super) struct AboutDialogNui {
    inner: Rc<AboutDialog>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl nwg::NativeUi<AboutDialogNui> for AboutDialog {
    fn build_ui(mut dialog: AboutDialog) -> Result<AboutDialogNui, nwg::NwgError> {
        dialog.controls.build()?;
        dialog.layout.build(&dialog.controls)?;
        dialog.events.build(&dialog.controls)?;
        dialog.controls.shake_window();

        let wrapper = AboutDialogNui {
            inner:  Rc::new(dialog),
            default_handler: Default::default(),
        };

        let data_ref = Rc::downgrade(&wrapper.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(evt_data) = data_ref.upgrade() {
                for eh in evt_data.events.events.iter() {
                    if handle == eh.control_handle && evt == eh.event {
                        (eh.handler)(&evt_data);
                        break;
                    }
                }
            }
        };

        *wrapper.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&wrapper.controls.window.handle, handle_events));

        return Ok(wrapper);
    }
}

impl Drop for AboutDialogNui {
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for AboutDialogNui {
    type Target = AboutDialog;

    fn deref(&self) -> &AboutDialog {
        &self.inner
    }
}