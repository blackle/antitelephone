use chrono::{DateTime, Utc};
use serenity::model::channel::Message;
use std::cmp::Ordering;

pub struct ScheduledMessage {
	pub message : Message,
	pub content : String,
	pub origin : DateTime<Utc>,
	pub destination : DateTime<Utc>
}

impl ScheduledMessage {
	pub fn new(message : Message, content : String, origin : DateTime<Utc>, destination : DateTime<Utc>) -> ScheduledMessage {
		ScheduledMessage {
			message: message,
			content: content,
			origin: origin,
			destination: destination
		}
	}

	pub fn post(self) -> bool {
		let avatar = match self.message.author.static_avatar_url() {
			Some(value) => value,
			None => String::new()
		};
		let name = &self.message.author.name;
		let content = &self.content;

		self.message.channel_id.send_message(|m|
			m.content(format!("Ring Ring! Message from {} has arrived!", &self.origin.to_rfc2822() ))
			.embed(|e|
				e.description(content)
				.timestamp(&self.origin)
				.author(|a|
					a.name(name)
					.icon_url(&avatar)
				)
			)
		);

		return true;
	}
}

impl Ord for ScheduledMessage {
	fn cmp(&self, other: &ScheduledMessage) -> Ordering {
		other.destination.cmp(&self.destination)
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
