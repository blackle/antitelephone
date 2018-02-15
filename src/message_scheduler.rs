use std::sync::{Mutex, Arc};
use scheduled_message::ScheduledMessage;
use std::ops::DerefMut;
use timer::{Timer, Guard};
use chrono::Utc;
use message_database::MessageDatabase;

pub struct MessageScheduler {
	//TODO: does this really need to be pub??? what is MessageScheduler's responsibility??
	pub db : MessageDatabase,
	timer : Timer,
	guard : Option<Guard>,
}

impl MessageScheduler {
	pub fn new(db : MessageDatabase) -> MessageScheduler {
		MessageScheduler {
			db: db,
			timer: Timer::new(),
			guard: None
		}
	}

	pub fn push(self_arc : &Arc<Mutex<MessageScheduler>>, message : ScheduledMessage) {
		let arc_clone = self_arc.clone();
		{
			let mut self_unlocked = self_arc.lock().unwrap();
			self_unlocked.db.push(message);
		}

		MessageScheduler::make_timer(&arc_clone);
	}

	pub fn timeout(self_arc : &Arc<Mutex<MessageScheduler>>) {
		let arc_clone = self_arc.clone();

		{
			let mut self_unlocked = self_arc.lock().unwrap();
			self_unlocked.db.post_all_until(Utc::now());
		}

		MessageScheduler::make_timer(&arc_clone);
	}

	fn make_timer(self_arc : &Arc<Mutex<MessageScheduler>>) {
		let arc_clone = self_arc.clone();
		// I don't know why this works but it does ;_;
		let mut self_unlocked = self_arc.lock().unwrap();
		let real_self = self_unlocked.deref_mut();

		if let Some(message) = real_self.db.peek() {
			let duration = message.destination.signed_duration_since(Utc::now());

			real_self.guard = Some(real_self.timer.schedule_with_delay(duration, move || {
				MessageScheduler::timeout(&arc_clone);
			}));
		}
	}

}