pub mod events;
pub mod events_generated;

/// The singleton of the EventSystem
static mut INSTANCE: Option<EventSystem> = Option::None;

/// Get the EventSystem singleton as immutable.
pub fn get_event_system() -> &'static EventSystem {
    let event_system: &EventSystem;
    unsafe {
        //Initialize EventSystem if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(EventSystem::new());
        }
        event_system = INSTANCE.as_ref().unwrap();
    }
    event_system
}

/// Get the EventSystem singleton as mutable.
pub fn get_mut_event_system() -> &'static mut EventSystem {
    let event_system: &mut EventSystem;
    unsafe {
        //Initialize EventSystem if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(EventSystem::new());
        }
        event_system = INSTANCE.as_mut().unwrap();
    }
    event_system
}

/// Allows for events/delegates to be stored and sent.
/// Events can have any number of parameters. The parameter types and amount are determined by the EventType enum.
/// This crate utilizes generated code. If the event system is having compile errors, check the events_generated file.
pub struct EventSystem {
    subscribed_events: events_generated::AllEvents
}

impl EventSystem {
    pub fn new() -> Self {
        println!("Created new event system");
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