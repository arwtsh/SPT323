use log::{info, trace};

use crate::event_system::generated::{AllEvents, EventDelegate};
use crate::event_system::events::EventType;

/// The singleton of the EventSystem
static mut INSTANCE: Option<EventSystem> = Option::None;

/// Get the EventSystem singleton as immutable.
pub fn get_event_system() -> &'static EventSystem {
    unsafe {
        //Initialize EventSystem if it hasn't been already.
        INSTANCE.get_or_insert_with(|| EventSystem::new())
    }
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
    subscribed_events: AllEvents
}

impl EventSystem {
    pub fn new() -> Self {
        info!("Initialized event system");
        Self {
            subscribed_events: AllEvents::new()
        }
    }

    /// Add a listener to the attached event
    pub fn add_listener(&mut self, event: EventDelegate) {
        self.subscribed_events.bind(event);
    }

    /// Invoke the specified event
    pub fn invoke(&self, event: EventType) {
        self.subscribed_events.broadcast(event);
    }
}