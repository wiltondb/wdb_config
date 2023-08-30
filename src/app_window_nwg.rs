
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use super::*;
use app_window::AppWindow;

pub struct AppWindowNwg {
    inner: Rc<AppWindow>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl nwg::NativeUi<AppWindowNwg> for AppWindow {
    fn build_ui(mut data: AppWindow) -> Result<AppWindowNwg, nwg::NwgError> {

        app_window_controls::build(&mut data)?;
        app_window_layout::build(&mut data)?;

        let ui = AppWindowNwg {
            inner:  Rc::new(data),
            default_handler: Default::default(),
        };

        // Events
        let evt_ui = Rc::downgrade(&ui.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(evt_ui) = evt_ui.upgrade() {
                for eh in evt_ui.events.iter() {
                    if handle == eh.control && evt == eh.event {
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

impl Drop for AppWindowNwg {
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for AppWindowNwg {
    type Target = AppWindow;

    fn deref(&self) -> &AppWindow {
        &self.inner
    }
}
