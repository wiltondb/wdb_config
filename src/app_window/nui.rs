
use super::*;

pub struct AppWindowNui {
    inner: Rc<AppWindow>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl nwg::NativeUi<AppWindowNui> for AppWindow {
    fn build_ui(mut dialog: AppWindow) -> Result<AppWindowNui, nwg::NwgError> {
        dialog.c.build()?;
        dialog.layout.build(&dialog.c)?;
        dialog.events.build(&dialog.c)?;
        dialog.init();
        dialog.c.shake_window();

        let wrapper = AppWindowNui {
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

        *wrapper.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&wrapper.c.window.handle, handle_events));

        return Ok(wrapper);
    }

}

impl Drop for AppWindowNui {
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for AppWindowNui {
    type Target = AppWindow;

    fn deref(&self) -> &AppWindow {
        &self.inner
    }
}
