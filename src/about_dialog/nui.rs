
use std::cell::RefCell;
use std::rc::Rc;

use super::*;

pub(super) struct AboutDialogNui {
    inner: Rc<RefCell<AboutDialog>>,
    inner_events: Rc<AboutDialogEvents>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl AboutDialogNui {
    pub(super) fn result(&mut self) -> () {
        self.inner.borrow_mut().result()
    }
}

impl nwg::NativeUi<AboutDialogNui> for AboutDialog {
    fn build_ui(mut dialog: AboutDialog) -> Result<AboutDialogNui, nwg::NwgError> {
        let mut events: AboutDialogEvents = Default::default();
        dialog.c.build()?;
        events.build(&dialog.c)?;
        dialog.init();

        let window_handle = dialog.c.window().handle.clone();

        let wrapper = AboutDialogNui {
            inner:  Rc::new(RefCell::new(dialog)),
            inner_events: Rc::new(events),
            default_handler: Default::default(),
        };

        let dialog_ref = Rc::downgrade(&wrapper.inner);
        let events_ref = Rc::downgrade(&wrapper.inner_events);
        let handle_events = move |evt, evt_data, handle| {
            if let Some(evt_dialog_ref) = dialog_ref.upgrade() {
                if let Some(evt_events_ref) = events_ref.upgrade() {
                    for eh in evt_events_ref.events.iter() {
                        if handle == eh.control_handle && evt == eh.event {
                            let mut evt_dialog = evt_dialog_ref.borrow_mut();
                            (eh.handler)(&mut evt_dialog, evt_data);
                            break;
                        }
                    }
                }
            }
        };

        *wrapper.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&window_handle, handle_events));

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
