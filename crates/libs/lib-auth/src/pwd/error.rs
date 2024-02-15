use derive_more::From;
use serde::Serialize;

use super::scheme;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, From)]
pub enum Error {
	PwdWithSchemeFailedParse,
	FailSpawnBlockForValidate,
	FailSpawnBlockForHash,
	// -- Moules
	#[from]
	Scheme(scheme::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
