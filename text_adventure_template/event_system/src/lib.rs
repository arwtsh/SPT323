mod events;
mod events_generated;

/// Allows for events/delegates to be stored and sent.
/// Events can have any number of parameters. The parameter types and amount are determined by the EventType enum.
/// This crate utilizes generated code. If the event system is having compile errors, check the events_generated file.
struct EventSystem {
    subscribed_events: events_generated::AllEvents
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            subscribed_events: events_generated::AllEvents::new()
        }
    }

    /// Add a listener to the attached event
    pub fn add_listener(&mut self, event: events_generated::EventDelegate) {
        self.subscribed_events.bind(event);
    }

    /// Invoke the specified event
    pub fn invoke(&self, event: events::EventType) {
        self.subscribed_events.broadcast(event)
    }
}