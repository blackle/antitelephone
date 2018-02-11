use chrono::{DateTime, Utc};
use serenity::model::channel::Message;
use std::cmp::Ordering;

pub struct ScheduledMessage {
	pub message: Message,
	pub origin : DateTime<Utc>,
	pub destination : DateTime<Utc>
}

impl ScheduledMessage {
	pub fn new(message : Message, origin : DateTime<Utc>, destination : DateTime<Utc>) -> ScheduledMessage {
		ScheduledMessage {
			message: message,
			origin: origin,
			destination: destination
		}
	}
}

impl Ord for ScheduledMessage {
	fn cmp(&self, other: &ScheduledMessage) -> Ordering {
		self.destination.cmp(&other.destination)
	}
}

impl PartialOrd for ScheduledMessage {
	fn partial_cmp(&self, other: &ScheduledMessage) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for ScheduledMessage {
	fn eq(&self, other: &ScheduledMessage) -> bool {
		self.message.id == other.message.id
	}
}

impl Eq for ScheduledMessage {}
