use crate::entity::runtime_instance::RuntimeInstance;
use tokio::sync::mpsc::{Sender, Receiver};

use crate::core::timer_event::TimerEvent;
use crate::entity::{timer::Timer, runtime_kind::RuntimeKind};


#[derive(Clone, Debug, Default)]
pub struct TimerBuilder {
    pub(crate) runtime_instance: RuntimeInstance,
    timer_event_channel: Option<(Sender<TimerEvent>, Receiver<TimerEvent>)>,
}


impl TimerBuilder {
    pub fn build(mut self) -> Timer {
        self.init_delay_timer()
    }

    fn init_delay_timer(&mut self) -> Timer {
        if self.runtime_instance.inner.is_none()
        {
            self.runtime_instance = RuntimeInstance::init_runtime();
        }

        let shared_header = SharedHeader {
            runtime_instance: self.runtime_instance.clone(),
            ..Default::default()
        };

        let timer_event_sender = self.get_timer_event_sender();

        self.lauch(shared_header.clone())
            .expect("delay-timer The base task failed to launch.");

        #[cfg(feature = "status-report")]
            let mut status_reporter = None;
        #[cfg(feature = "status-report")]
        if self.enable_status_report {
            status_reporter = Some(StatusReporter::new(self.get_status_report_receiver()));
        }

        DelayTimer {
            shared_header,
            timer_event_sender,
            #[cfg(feature = "status-report")]
            status_reporter,
        }
    }
}