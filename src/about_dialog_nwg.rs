use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use crate::*;
use about_dialog::AboutDialog;

pub struct AboutDialogNwg {
    inner: Rc<AboutDialog>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl nwg::NativeUi<AboutDialogNwg> for AboutDialog {
    fn build_ui(mut data: AboutDialog) -> Result<AboutDialogNwg, nwg::NwgError> {

        about_dialog_controls::build(&mut data)?;
        about_dialog_layout::build(&mut data)?;

        let ui = AboutDialogNwg {
            inner:  Rc::new(data),
            default_handler: Default::default(),
        };

        // Events
        let evt_ui = Rc::downgrade(&ui.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(evt_ui) = evt_ui.upgrade() {
                for eh in evt_ui.events.iter() {
                    if handle == eh.control_handle && evt == eh.event {
                        (eh.handler)(&evt_ui);
                        break;
                    }
                }
            }
        };

        *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));

        return Ok(ui);
    }

}

impl Drop for AboutDialogNwg {
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for AboutDialogNwg {
    type Target = AboutDialog;

    fn deref(&self) -> &AboutDialog {
        &self.inner
    }
}
