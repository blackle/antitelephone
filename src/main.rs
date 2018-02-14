#[macro_use] extern crate serenity;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate regex;
extern crate duration_parser;
extern crate chrono;
extern crate typemap;
extern crate timer;
extern crate cast;
extern crate fs2;

mod scheduled_message;
use scheduled_message::ScheduledMessage;

mod error;
use error::Error;
use std::error::Error as StdError;

mod message_scheduler;
use message_scheduler::MessageScheduler;

mod message_database;
use message_database::MessageDatabase;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::StandardFramework;
use duration_parser::parse_duration;
use duration_parser::Error as DurationError;
use chrono::{Duration, Utc};
use std::env;
use typemap::Key;
use regex::Regex;
use std::sync::{Mutex, Arc};
use cast::i64;

static HELP_TEXT: &'static str = r#"```
ANTITELEPHONE HELP MANUAL
anti.help - this help message
anti.list - show scheduled messages. will not reveal their contents
anti.del <message id> - deletes a message from the list from its id
anti.msg <offset> <message> - schedules <message> to appear <offset> from now. Offset is a number followed by a character like so:
	8s - eight seconds
	12m - twelve minutes
	6h - six hours
	7d - seven days
	2w - two weeks
	3y - three years
	you may also chain them together like so:
	2w5d12m10s - two weeks, five days, twelve minutes, and ten seconds from now
```"#;


struct MessageSchedulerKey;

impl Key for MessageSchedulerKey {
	type Value = Arc<Mutex<MessageScheduler>>;
}

struct Handler;

impl EventHandler for Handler {}

fn main() {
	// Login with a bot token from the environment
	let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
		.expect("Error creating client");

	{
		let mut data = client.data.lock();
		let db = MessageDatabase::new().unwrap();
		let scheduler = Arc::new(Mutex::new(MessageScheduler::new(db)));
		data.insert::<MessageSchedulerKey>(scheduler);
	}

	client.with_framework(StandardFramework::new()
		.configure(|c| c.prefix("anti.")) // set the bot's prefix to "~"
		.cmd("list", list)
		.cmd("?", help)
		.cmd("msg", msg));

	// start listening for events by starting a single shard
	if let Err(why) = client.start() {
		println!("An error occurred while running the client: {:?}", why);
	}
}

command!(help(_context, message) {
	if let Err(why) = message.channel_id.say(HELP_TEXT) {
		println!("Error sending message: {:?}", why);
	}
});

// TODO: separate struct/impl for handing these?
command!(list(context, message) {
	// let data = context.data.lock();
	// let scheduler = data.get::<MessageSchedulerKey>().unwrap();

	// let scheduler_unlocked = scheduler.lock().unwrap();
	// for scheduled_message in scheduler_unlocked.queue.iter() {
	// 	if scheduled_message.channel_id == message.channel_id {
	// 		println!("{:?}", scheduled_message.content);
	// 	}
	// }

	// let mut messages = Vec::new();
	// for item in scheduler.iter() {
	// 	messages.push(item.message.content.clone());
	// }

	// message.channel_id.send_message(|m|
	// 	m.content(messages.join("\n")));
});

fn parse_msg(message : &String) -> Result<(String, Duration), Error> {
	lazy_static! {
		static ref COMMAND_RE: Regex = Regex::new(r"^(?P<command>[^ ]+) (?P<duration>[^ ]+) (?P<content>.*)$").unwrap();
	}

	//there is only one capture here because the regex has start and end anchors
	for capture in COMMAND_RE.captures_iter(message) {
		let duration_str = &capture["duration"];
		let content = &capture["content"];

		let duration_seconds = parse_duration(&String::from(duration_str))?;

		let duration_millis = match duration_seconds.checked_mul(1000) {
			Some(val) => val,
			None => return Err(Error::Duration(DurationError::Overflow))
		};

		//I actually hate this
		let duration = Duration::milliseconds(match i64(duration_millis) {
			Ok(val) => val,
			Err(_) => return Err(Error::Duration(DurationError::Overflow))
		});

		if content.len() > 2048 {
			return Err(Error::MessageTooLong);
		}

		return Ok((String::from(content), duration));
	}

	return Err(Error::IncorrectFormat);
}

command!(msg(context, message) {
	let (content, duration) = match parse_msg(&message.content) {
		Ok(tuple) => tuple,
		Err(e) => {
			if let Err(why) = message.channel_id.say(format!("⚠ {}", e.description())) {
				println!("Error sending message: {:?}", why);
			}
			return Ok(());
		}
	};

	let now = Utc::now();
	let scheduled_msg = ScheduledMessage::new(content, message.id, message.author.id, message.channel_id, now, now + duration);

	//we don't care if we can delete the message or not. that permission might not be enabled.
	message.delete().ok();
	if let Err(why) = message.channel_id.say(format!("✅ Message from @{} consumed by the antitelephone. Scheduled for {}", &message.author.name, &scheduled_msg.destination.to_rfc2822() )) {
		println!("Error sending message: {:?}", why);
	}

	let data = context.data.lock();
	let scheduler = data.get::<MessageSchedulerKey>().unwrap();
	MessageScheduler::push(&scheduler, scheduled_msg);
});
