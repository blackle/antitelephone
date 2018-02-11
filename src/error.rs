use duration_parser::Error as DurationError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
	IncorrectFormat,
	MessageTooLong,
	Duration(DurationError)
}

impl From<DurationError> for Error {
	fn from(e: DurationError) -> Error { Error::Duration(e) }
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(self.description())
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		match *self {
			Error::IncorrectFormat => "Command is in an incorrect format",
			Error::MessageTooLong => "Message is longer than 2048 characters",
			Error::Duration(ref inner) => inner.description(),
		}
	}
}