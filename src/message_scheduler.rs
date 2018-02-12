use std::sync::{Mutex, Arc};
use std::collections::BinaryHeap;
use scheduled_message::ScheduledMessage;
use timer::{Timer, Guard};
use chrono::{Duration, Utc};

type ScheduledMessageHeap = BinaryHeap<ScheduledMessage>;

pub struct MessageScheduler {
	queue : ScheduledMessageHeap,
	timer : Timer,
	guard : Option<Guard>,
}

impl MessageScheduler {
	pub fn new() -> MessageScheduler {
		MessageScheduler {
			queue: ScheduledMessageHeap::new(),
			timer: Timer::new(),
			guard: None
		}
	}

	pub fn push(self_arc : &Arc<Mutex<MessageScheduler>>, message : ScheduledMessage) {
		let arc_clone = self_arc.clone();
		{
			let mut self_unlocked = self_arc.lock().unwrap();
			self_unlocked.queue.push(message);
		}

		MessageScheduler::make_timer(&arc_clone);
	}

	pub fn timeout(self_arc : &Arc<Mutex<MessageScheduler>>) {
		let arc_clone = self_arc.clone();

		{
			let mut self_unlocked = self_arc.lock().unwrap();
			while let Some(message) = self_unlocked.queue.pop() {
				if message.destination < Utc::now() {
					message.post();
				} else {
					self_unlocked.queue.push(message);
					break;
				}
			}
		}

		MessageScheduler::make_timer(&arc_clone);
	}

	fn make_timer(self_arc : &Arc<Mutex<MessageScheduler>>) {
		let arc_clone = self_arc.clone();
		let mut self_unlocked = self_arc.lock().unwrap();

		if let Some(message) = self_unlocked.queue.pop() {
			let duration = message.destination.signed_duration_since(Utc::now());
			self_unlocked.queue.push(message);

			self_unlocked.guard = Some(self_unlocked.timer.schedule_with_delay(duration, move || {
				MessageScheduler::timeout(&arc_clone);
			}));
		}
	}

}