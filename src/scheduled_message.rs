use chrono::{DateTime, Utc};
use serenity::model::id::{ChannelId, MessageId, UserId};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize)]
pub struct ScheduledMessage {
	pub content : String,
	pub message_id : MessageId,
	pub author_id : UserId,
	pub channel_id : ChannelId,
	pub origin : DateTime<Utc>,
	pub destination : DateTime<Utc>
}

impl ScheduledMessage {
	pub fn new(content : String, message_id : MessageId, author_id : UserId, channel_id : ChannelId, origin : DateTime<Utc>, destination : DateTime<Utc>) -> ScheduledMessage {
		ScheduledMessage {
			content: content,
			message_id: message_id,
			author_id: author_id,
			channel_id: channel_id,
			origin: origin,
			destination: destination
		}
	}

	//TODO: make this not a bool and instead a Result<(), Error>
	pub fn post(self) -> bool {
		let author = match self.author_id.get() {
			Ok(author) => author,
			Err(why) => {
				println!("Error retrieving author information: {:?}", why);
				return false;
			}
		};

		let avatar = match author.static_avatar_url() {
			Some(value) => value,
			None => String::new()
		};
		let name = &author.name;
		let content = &self.content;

		if let Err(why) = self.channel_id.send_message(|m|
			m.content(format!("☎ Ring Ring! Message from {} has arrived!", &self.origin.to_rfc2822() ))
			.embed(|e|
				e.description(content)
				.timestamp(&self.origin)
				.author(|a|
					a.name(name)
					.icon_url(&avatar)
				)
			)
		) {
			println!("Error sending message: {:?}", why);
		}

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
		self.message_id == other.message_id
	}
}

impl Eq for ScheduledMessage {}
