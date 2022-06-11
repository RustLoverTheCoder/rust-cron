use crate::entity::timer_builder::TimerBuilder;

#[derive(Clone, Debug)]
pub struct Timer {
    #[allow(dead_code)]
    shared_header: SharedHeader,
    timer_event_sender: TimerEventSender,
}

impl Default for Timer {
    fn default() -> Self {
        TimerBuilder::default().build()
    }
}
