

pub struct EventHandler<W> {
    pub control_handle: nwg::ControlHandle,
    pub event: nwg::Event,
    pub handler: fn(&W) -> ()
}

pub struct EventBuilder<W> {
    control_handle: Option<nwg::ControlHandle>,
    event: Option<nwg::Event>,
    handler: Option<fn(&W) -> ()>
}

impl<W> EventBuilder<W> {
    pub fn new() -> Self {
        Self {
            control_handle: None,
            event: None,
            handler: None
        }
    }

    pub fn control<C: Into<nwg::ControlHandle>>(mut self, control: C) -> Self {
        self.control_handle = Some(control.into());
        self
    }

    pub fn event(mut self, event: nwg::Event) -> Self {
        self.event = Some(event);
        self
    }

    pub fn handler(mut self, handler: fn(&W) -> ()) -> Self {
        self.handler = Some(handler);
        self
    }

    pub fn build(self, events: &mut Vec<EventHandler<W>>) -> Result<(), nwg::NwgError> {
        let control_handle = match self.control_handle {
            None => return Err(nwg::NwgError::events_binding("Control not specified".to_string())),
            Some(ch) => Ok::<nwg::ControlHandle, nwg::NwgError>(ch)
        }?;
        let event = match self.event {
            None => return Err(nwg::NwgError::events_binding("Event not specified".to_string())),
            Some(ev) => Ok::<nwg::Event, nwg::NwgError>(ev)
        }?;
        let handler = match self.handler {
            None => return Err(nwg::NwgError::events_binding("Handler not specified".to_string())),
            Some(h) => Ok::<fn(&W) -> (), nwg::NwgError>(h)
        }?;

        events.push(EventHandler {
            control_handle, event, handler
        });

        Ok(())
    }
}

pub fn builder<W>() -> EventBuilder<W> {
    EventBuilder::new()
}
