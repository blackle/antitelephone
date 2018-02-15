use std::collections::BinaryHeap;
use scheduled_message::ScheduledMessage;
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::io::{Error, Seek, SeekFrom};
use fs2::FileExt;
use serde_json::{from_reader, to_writer};
use chrono::{DateTime, Utc};
use serenity::model::id::ChannelId;
use std::collections::binary_heap::Iter;
use std::iter::Filter;

type ScheduledMessageHeap = BinaryHeap<ScheduledMessage>;

pub struct MessageDatabase {
	queue : ScheduledMessageHeap,
	source : File,
}

trait Reset {
	fn reset(&mut self) -> Result<(), Error>;
}

impl Reset for File {
	fn reset(&mut self) -> Result<(), Error> {
		//I kinda hate that I have to do this but oh well
		self.seek(SeekFrom::Start(0))?;
		self.set_len(0)
	}
}

impl MessageDatabase {
	pub fn new() -> Result<MessageDatabase, Error> {
		let source = OpenOptions::new()
			.write(true)
			.read(true)
			.create(true)
			.mode(0o660)
			.open("anti.db")?;

		source.try_lock_exclusive()?;

		let queue = if source.metadata()?.len() == 0 {
			ScheduledMessageHeap::new()
		} else {
			//TODO: change me to ? instead of unwrap buckaroo
			from_reader(source.try_clone()?).unwrap()
		};

		let mut db = MessageDatabase {
			queue: queue,
			source: source
		};

		//TODO: ask channels permission to post overdue messages
		db.post_all_until(Utc::now());
		Ok(db)
	}

	fn save(&self) -> Result<(), Error> {
		//TODO: make this better goddamit
		let mut source_copy = self.source.try_clone()?;
		source_copy.reset()?;
		//if this doesn't work we're absolutely fucked
		//TODO: make this the final return instead of unwrap
		to_writer(source_copy, &self.queue).unwrap();
		Ok(())
	}

	pub fn push(&mut self, message : ScheduledMessage) {
		//TODO: limit number of messages on a per-channel basis
		self.queue.push(message);
		//TODO: handle the return value of save
		self.save();
	}

	pub fn peek(&self) -> Option<&ScheduledMessage> {
		return self.queue.peek();
	}

	pub fn post_all_until(&mut self, datetime : DateTime<Utc>) {
		while let Some(message) = self.queue.pop() {
			if message.destination < datetime {
				message.post();
			} else {
				self.queue.push(message);
				break;
			}
		}
		self.save();
	}

	pub fn iter<'a>(&'a self, channel_id : &'a ChannelId) -> Box<Iterator<Item=&'a ScheduledMessage> + 'a> {
		Box::new(self.queue.iter().filter(move |x| &x.channel_id == channel_id))
	}
}

