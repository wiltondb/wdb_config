

pub struct EventHandler<W> {
    pub control: nwg::ControlHandle,
    pub event: nwg::Event,
    pub handler: fn(&W) -> ()
}

pub struct EventBuilder<W> {
    control: Option<nwg::ControlHandle>,
    event: Option<nwg::Event>,
    handler: Option<fn(&W) -> ()>
}

impl<W> EventBuilder<W> {
    pub fn new() -> Self {
        Self {
            control: None,
            event: None,
            handler: None
        }
    }

    pub fn control(mut self, control: nwg::ControlHandle) -> Self {
        self.control = Some(control);
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
        match self.control {
            None => return Err(nwg::NwgError::EventsBinding("Control not specified".to_string())),
            Some(control) => {
                match self.event {
                    None => return Err(nwg::NwgError::EventsBinding("Event not specified".to_string())),
                    Some(event) => {
                        match self.handler {
                            None => return Err(nwg::NwgError::EventsBinding("Handler not specified".to_string())),
                            Some(handler) => {
                                events.push(EventHandler {
                                    control, event, handler
                                });
                                Ok(())
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn builder<W>() -> EventBuilder<W> {
    EventBuilder::new()
}
