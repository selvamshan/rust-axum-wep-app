use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	MissingCtx,

	// -- RPC Router
	RpcMethodUnknown(String),
	RpcIntoParamsMissing,

	// -- Modules
	Model(lib_core::model::Error),

	// -- External Modules
	SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

// region: --- Forms
impl From<lib_core::model::Error> for Error {
	fn from(val: lib_core::model::Error) -> Self {
		Self::Model(val)
	}
}

impl From<serde_json::Error> for Error {
	fn from(val: serde_json::Error) -> Self {
		Self::SerdeJson(val)
	}
}
// endregion: --- Form

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
